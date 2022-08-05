use anchor_lang::prelude::*;

#[error_code]
pub enum BattleRoyaleError {
    #[msg("Invalid statistics")]
    InvalidStatistics,

    #[msg("Invalid collection symbol")]
    CollectionSymbolInvalid,

    #[msg("Invalid provided creators")]
    VerifiedCreatorsInvalid,

    #[msg("Failed collection verification")]
    CollectionVerificationFailed,

    #[msg("Not enough action points")]
    InsufficientActionPoints,
}
