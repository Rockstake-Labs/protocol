pub const ERR_INVALID_MARKET: &str = "Market doesn't exist!";
pub const ERR_MARKET_NOT_OPEN: &str = "Market is not open for betting";
pub const ERR_MARKET_CLOSED: &str = "Market is closed";
pub const ERR_LIABILITY_BACK_BET: &str = "Liability must be zero for Back bets";
pub const ERR_LIABILITY_ZERO: &str = "Liability must be greater than zero for Lay bets";
pub const ERR_LIABILITY_TOTAL_AMOUNT: &str = "Liability parameter doesn't match the required liability for the given total amount";
pub const ERR_BET_ODDS: &str = "Odds must be between 1.01 and 1000.00";
pub const ERR_USER_FUNDS: &str = "Insufficient funds for this bet";
pub const ERR_BET_STATE: &str = "Bet is not in a state eligible for distribution";
pub const ERR_SELECTION: &str = "Selection not found in this market";
pub const ERR_TOKEN_ALREADY_ISSUED: &str = "Token already issued";
pub const ERR_TOKEN_NOT_ISSUED: &str = "Token not issued";
pub const ERR_INVALID_NFT_TOKEN: &str = "Invalid token";
pub const ERR_INVALID_NFT_TOKEN_NONCE: &str = "Invalid token nonce";
pub const ERR_INVALID_ROLE: &str = "Unauthorized! Invalid Role";
