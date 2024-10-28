use crate::types::{BetStatus, BetType, EventResult};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("bet_placed")]
    fn bet_placed_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] betslip_token_id: &TokenIdentifier,
        #[indexed] market_id: &u64,
        #[indexed] selection_id: &u64,
        #[indexed] total_amount: &BigUint,
        #[indexed] odds: &BigUint,
        #[indexed] bet_type: BetType,
        #[indexed] token_identifier: &EgldOrEsdtTokenIdentifier,
        #[indexed] token_nonce: u64,
        #[indexed] matched_amount: &BigUint,
        #[indexed] unmatched_amount: &BigUint,
        #[indexed] collateral: &BigUint

    );

    #[event("bet_closed")]
fn bet_closed_event(
    &self,
    #[indexed] bettor: &ManagedAddress,
    #[indexed] bet_id: &u64,
    #[indexed] market_id: &u64,
    #[indexed] selection_id: &u64,
    #[indexed] refund_amount: &BigUint,
    #[indexed] token_identifier: &EgldOrEsdtTokenIdentifier,
    #[indexed] token_nonce: u64,
);

    #[event("debug")]
    fn debug_event(&self, #[indexed] msg: ManagedBuffer, #[indexed] value: BigUint);

    #[event("market_closed")]
    fn market_closed_event(&self, #[indexed] market_id: BigUint, #[indexed] winning_selection_id: BigUint);

    #[event("expired_markets_closed")]
    fn expired_markets_closed_event(&self, #[indexed] market_ids: ManagedVec<u64>);
    
    #[event("refund_unmatched_bet")]
    fn refund_unmatched_bet_event(
        &self,
        #[indexed] bet_id: u64,
        #[indexed] amount: &BigUint,
        #[indexed] bettor: &ManagedAddress,
    );

    #[event("claimFromBetslip")]
    fn claim_from_betslip_event(
        &self,
        #[indexed] betslip_id: u64,
        #[indexed] payout: &BigUint,
        #[indexed] recipient: &ManagedAddress,
    );

      // Storage pentru rezultatele evenimentelor
      #[view(getEventResult)]
      #[storage_mapper("eventResults")]
      fn event_results(&self, market_id: &u64) -> SingleValueMapper<EventResult>;
  
      #[event("bet_won")]
      fn bet_won_event(
          &self,
          #[indexed] bettor: &ManagedAddress,
          #[indexed] nft_nonce: &u64,
          #[indexed] event_id: &u64,
          #[indexed] selection_id: &u64,
          #[indexed] win_amount: &BigUint,
          #[indexed] token_identifier: &EgldOrEsdtTokenIdentifier,
          #[indexed] token_nonce: u64,
      );

      #[event("reward_distributed")]
      fn reward_distributed_event(
          &self,
          #[indexed] bet_id: u64,
          #[indexed] bettor: &ManagedAddress,
          amount: &BigUint,
      );

      #[event("betCounterUpdate")]
      fn bet_counter_update_event(
          &self,
          #[indexed] old_status: &BetStatus,
          #[indexed] new_status: &BetStatus,
          #[indexed] matched_count: usize,
          #[indexed] unmatched_count: usize,
          #[indexed] partially_matched_count: usize,
          #[indexed] win_count: usize,
          #[indexed] lost_count: usize,
          #[indexed] canceled_count: usize,
      );
  
      #[event("betCounterUpdated")]
      fn bet_counter_updated_event(
          &self,
          #[indexed] old_status: &BetStatus,
          #[indexed] new_status: &BetStatus,
          #[indexed] matched_count: usize,
          #[indexed] unmatched_count: usize,
          #[indexed] partially_matched_count: usize,
          #[indexed] win_count: usize,
          #[indexed] lost_count: usize,
          #[indexed] canceled_count: usize,
      );

      #[event("marketQuery")]
fn market_query_event(
    &self,
    #[indexed] market_id: u64,
    #[indexed] selection_count: usize
);

#[event("selectionCounts")]
fn selection_counts_event(
    &self,
    #[indexed] market_id: u64,
    #[indexed] selection_id: u64,
    #[indexed] matched: &BigUint,
    #[indexed] unmatched: &BigUint,
    #[indexed] partially_matched: &BigUint,
    #[indexed] win: &BigUint,
    #[indexed] lost: &BigUint,
    #[indexed] canceled: &BigUint
);

#[event("totalCounts")]
fn total_counts_event(
    &self,
    #[indexed] market_id: u64,
    #[indexed] matched: &BigUint,
    #[indexed] unmatched: &BigUint,
    #[indexed] partially_matched: &BigUint,
    #[indexed] win: &BigUint,
    #[indexed] lost: &BigUint,
    #[indexed] canceled: &BigUint
);

}