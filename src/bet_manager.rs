use crate::{errors::{ERR_MARKET_CLOSED, ERR_MARKET_EXISTENCE, ERR_MARKET_OPEN, ERR_ODDS, ERR_SELECTION, ERR_USER_FUNDS}, priority_queue::PriorityQueue, types::{Bet, BetStatus, BetType, MarketStatus}};
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait BetManagerModule: crate::storage::StorageModule 
    + crate::events::EventsModule 
    + crate::nft_manager::NftManagerModule {

    #[payable("*")]
    #[endpoint(placeBet)]
    fn place_bet(&self, market_id: u64, selection_id: u64, odds: BigUint, bet_type: BetType) -> SCResult<(u64, BigUint, BigUint)> {
        let mut market = self.markets(&market_id).get();
        let created_at = self.blockchain().get_block_timestamp();

        require!(!self.markets(&market_id).is_empty(), ERR_MARKET_EXISTENCE);
        require!(market.market_status == MarketStatus::Open, ERR_MARKET_OPEN);
        require!(created_at < market.close_timestamp, ERR_MARKET_CLOSED);
        require!(odds >= BigUint::from(101u32) && odds <= BigUint::from(100000u32), ERR_ODDS);
    
        let caller = self.blockchain().get_caller();
        let (token_identifier, token_nonce, total_amount) = self.call_value().egld_or_single_esdt().into_tuple();
        let bet_id = self.get_last_bet_id() + 1;
    
        let (stake, liability) = match bet_type {
            BetType::Back => {
                (total_amount.clone(), BigUint::zero())
            },
            BetType::Lay => {
                let stake = self.calculate_stake_from_total(&total_amount, &odds);
                let liability = &total_amount - &stake;
                require!(liability == self.calculate_potential_liability(&bet_type, &stake, &odds), "Calculation mismatch");
                (stake, liability)
            }
        };
        
        let selection_index = market.selections.iter()
            .position(|s| &s.selection_id == &selection_id)
            .expect(ERR_SELECTION);
        let mut selection = market.selections.get(selection_index);
    
        let bet = Bet {
            bettor: caller.clone(),
            event: market_id,
            selection: selection.clone(),
            stake_amount: stake.clone(),
            liability: match bet_type {
                BetType::Back => BigUint::zero(),
                BetType::Lay => liability.clone(),
            },
            matched_amount: BigUint::zero(),
            unmatched_amount: stake.clone(),
            potential_profit: self.calculate_potential_profit(&bet_type, &stake, &odds),
            odd: odds.clone(),
            bet_type: bet_type.clone(),
            status: BetStatus::Unmatched,
            payment_token: token_identifier.clone(),
            payment_nonce: token_nonce,
            nft_nonce: bet_id,
            created_at: created_at
        };
    
        let (matching_bets, matched_amount, unmatched_amount) = 
            selection.priority_queue.get_matching_bets(&bet);
            
        for mut matched_bet in matching_bets.iter() {
            if matched_bet.matched_amount == matched_bet.stake_amount {
                matched_bet.status = BetStatus::Matched;
            } else {
                matched_bet.status = BetStatus::PartiallyMatched;
            }
            selection.priority_queue.remove(&matched_bet);
            if matched_bet.unmatched_amount > BigUint::zero() {
                selection.priority_queue.add(matched_bet);
            }
        }   

        let mut updated_bet = bet.clone();
        updated_bet.matched_amount = matched_amount.clone();
        updated_bet.unmatched_amount = unmatched_amount.clone();
        updated_bet.status = if matched_amount == stake {
            BetStatus::Matched
        } else if matched_amount > BigUint::zero() {
            BetStatus::PartiallyMatched
        } else {
            BetStatus::Unmatched
        };
    
        if unmatched_amount > BigUint::zero() {
            selection.priority_queue.add(updated_bet.clone());
        }
    
        let bet_nft_nonce = self.mint_bet_nft(&updated_bet);
        self.bet_by_id(bet_id).set(&updated_bet);
    
        let _ = market.selections.set(selection_index, &selection);
        let _ = market.total_matched_amount += &matched_amount;
        self.markets(&market_id).set(&market);
    
        let total_locked = match bet_type {
            BetType::Back => unmatched_amount.clone(),
            BetType::Lay => liability.clone(),
        };
        self.locked_funds(&caller).update(|current_locked| *current_locked += &total_locked);

        self.send().direct_esdt(&caller, self.bet_nft_token().get_token_id_ref(), bet_nft_nonce, &BigUint::from(1u64));
        
        self.bet_placed_event(
            &caller,
            self.bet_nft_token().get_token_id_ref(),
            &market_id,
            &selection_id,
            &stake,
            &odds,
            bet_type.clone(),
            &token_identifier,
            token_nonce,
            &matched_amount,
            &unmatched_amount,
            &liability
        );
    
        Ok((bet_id, odds, stake))
    }


    fn calculate_potential_profit(&self, bet_type: &BetType, stake: &BigUint, odds: &BigUint) -> BigUint {
        match bet_type {
            BetType::Back => {
                (odds - &BigUint::from(1u32)) * stake
            },
            BetType::Lay => stake.clone()
        }
    }

    fn calculate_stake_from_liability(&self, liability: &BigUint, odds: &BigUint) -> BigUint {
        liability / &(odds - &BigUint::from(1u32))
    }

    fn calculate_potential_liability(&self, bet_type: &BetType, stake: &BigUint, odds: &BigUint) -> BigUint {
        match bet_type {
            BetType::Back => stake.clone(),
            BetType::Lay => {
                // Presupunem că odds este reprezentat ca 170 pentru 1.70
                let odds_minus_one = odds - &BigUint::from(100u32);
                (stake * &odds_minus_one) / &BigUint::from(100u32)
            }
        }
    }
    
    fn calculate_stake_from_total(&self, total: &BigUint, odds: &BigUint) -> BigUint {
        total * &BigUint::from(100u32) / odds
    }
    
    fn calculate_win_amount(&self, bet_type: &BetType, stake_amount: &BigUint, odds: &BigUint) -> BigUint {
        match bet_type {
            BetType::Back => self.calculate_potential_profit(bet_type, stake_amount, odds),
            BetType::Lay => self.calculate_potential_liability(bet_type, stake_amount, odds),
        }
    }
           
}