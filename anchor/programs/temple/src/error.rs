/// Errors that may be returned by the TokenSwap program.
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Input account owner is not the program address")]
    InvalidOwner,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Invalid incense ID")]
    InvalidIncenseId,
    #[msg("Insufficient SOL balance to pay for incense")]
    InsufficientSolBalance,
    #[msg("Temple treasury account mismatch")]
    InvalidTempleTreasury,
    #[msg("Daily incense limit exceeded")]
    DailyIncenseLimitExceeded,
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Exceed daily incense limit")]
    ExceedDailyIncenseLimit,
    #[msg("Insufficient incense balance")]
    InsufficientIncenseBalance,
    #[msg("This incense type is only available through donations")]
    DonationOnlyIncense,
    #[msg("Failed to trigger special effect")]
    SpecialEffectFailed,
    #[msg("Insufficient merit points")]
    InsufficientMerit,
    #[msg("Daily wish limit exceeded")]
    DailyWishLimitExceeded,
    #[msg("Wish already exists")]
    WishAlreadyExists,
    #[msg("Invalid wish account")]
    InvalidWishAccount,
    #[msg("Invalid user state")]
    InvalidUserState,
    #[msg("Cannot like own wish")]
    CannotLikeOwnWish,
    #[msg("User has Buddha NFT")]
    UserHasBuddhaNFT,
    #[msg("Buddha NFT supply exceeded")]
    BuddhaNFTSupplyExceeded,

    #[msg("Insufficient donation")]
    InsufficientDonation,

    #[msg("Randomness not ready")]
    RandomnessNotReady,

    #[msg("Randomness not resolved")]
    RandomnessNotResolved,

    #[msg("Randomness already used")]
    RandomnessAlreadyUsed,

    #[msg("Randomness expired")]
    RandomnessExpired,

    #[msg("Invalid randomness")]
    InvalidRandomness,
    #[msg("Randomness request required")]
    RandomnessRequestRequired,

    // 动态配置相关错误
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid incense type")]
    InvalidIncenseType,
    #[msg("Invalid fortune config")]
    InvalidFortuneConfig,
    #[msg("Invalid donation level")]
    InvalidDonationLevel,
    #[msg("Invalid temple level")]
    InvalidTempleLevel,

    // 勋章相关错误
    #[msg("User already has medal NFT")]
    UserAlreadyHasMedalNFT,
    #[msg("Invalid medal level")]
    InvalidMedalLevel,
    #[msg("Medal level not increasing")]
    MedalLevelNotIncreasing,
    #[msg("Insufficient donation for medal upgrade")]
    InsufficientDonationForUpgrade,
    #[msg("Invalid medal owner")]
    InvalidMedalOwner,
    #[msg("Insufficient donation for medal")]
    InsufficientDonationForMedal,

    // 质押相关错误
    #[msg("User does not have medal NFT")]
    UserDoesNotHaveMedalNFT,
    #[msg("Medal already staked")]
    MedalAlreadyStaked,
    #[msg("Medal not staked")]
    MedalNotStaked,

    // 分享相关错误
    #[msg("Share too late after fortune draw")]
    ShareTooLate,
    #[msg("Not approved")]
    NotApproved,

    // 御守相关错误
    #[msg("Insufficient pending amulets balance")]
    InsufficientPendingAmulets,

    // 商城相关错误
    #[msg("Insufficient stock for the item")]
    InsufficientStock,
    #[msg("Invalid shop item ID")]
    InvalidShopItemId,
    #[msg("Shop item not available")]
    ShopItemNotAvailable,

    // 愿力塔相关错误
    #[msg("Invalid max level for wish tower")]
    InvalidMaxLevel,
    #[msg("Wish tower is completed")]
    WishTowerCompleted,
    #[msg("Wish tower level is full")]
    WishTowerLevelFull,
    #[msg("Wish not owned by user")]
    WishNotOwnedByUser,
}
