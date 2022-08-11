//! Error handling on the casper platform.
use types::ApiError;

/// Errors which can be returned by the library.
///
/// When an `Error` is returned from a smart contract, it is converted to an [`ApiError::User`].
///
/// Where a smart contract consuming this library needs to define further error variants, it can
/// return those via the [`Error::User`] variant or equivalently via the [`ApiError::User`]
/// variant.
///
/// Such a user error should be in the range `[0..(u16::MAX - 30)]` (i.e. [0, 65505]) to avoid
/// conflicting with the other `Error` variants.
pub enum Error {
    /// ERC20 contract called from within an invalid context.
    InvalidContext,
    /// Spender does not have enough balance.
    InsufficientBalance,
    /// Spender does not have enough allowance approved.
    InsufficientAllowance,
    /// Operation would cause an integer overflow.
    Overflow,
    /// Tokens addresses are identical.
    IdenticalAddresses,
    /// Tokens address is null.
    ZeroAddress,
    /// At least one of the pool's reserves is empty.
    InsufficientLiquidity,
    /// Input amount for the swap is null.
    InsufficientInputAmount,
    /// Output amount for the swap is null.
    InsufficientOutputAmount,
    /// Given amount is null.
    InsufficientAmount,
    /// Path from the two tokens is inferior to 2.
    InvalidPath,
    /// Deadline Expired
    Expired,
    /// The amount of the A token is inferior to the minimum amount requested by the provider.
    InsufficientAAmount,
    /// The amount of the B token is inferior to the minimum amount requested by the provider.
    InsufficientBAmount,
    /// The input amount required for the swap surpasses the amount_in_max.
    ExcessiveInputAmount,
    /// Caller tries to withdraw more CSPR than his WCSPR balance.
    ExcessiveAmount,
    /// Tried to create a pair that already exists.
    PairExists,
    /// The caller is not authorized to call the function.
    Forbidden,
    /// The liquidity minted inside `mint()` equals zero.
    InsufficientLiquidityMinted,
    /// The liquidity that is set to be burned inside `burn()` equals zero.
    InsufficientLiquidityBurned,
    /// Tried to call a locked contract's function.
    Locked,
    /// Calling Pair's swap operation while `to` is one of the pair's tokens addresses.
    InvalidTo,
    /// In Pair's swap(): When the pair tokens' balances product is inferior than the reserves product multiplied by 1000^2.
    K,
    /// The given signature for permit() is erronous.
    InvalidSignature,
    /// Cannot mint tokens to zero hash address.
    CannotMintToZeroHash,
    /// Cannot burn tokens from zero hash address.
    CannotBurnFromZeroHash,
    /// Trying to burn an amount that surpasses the owner's balance.
    BurnAmountExceedsBalance,
    /// At leaset one of the pair's token reserves equals zero.
    NoReserves,
    /// Trying to call `simple-oracle::update` before `period` has elapsed since the last update.
    PeriodNotElapsed,
    /// Trying to call `simple-oracle::consult` while providing an invalid token address.
    InvalidToken,
    /// Trying to deploy a payment contract while providing an invalid deposit entry_point name.
    InvalidDepositEntryPointName,
    /// User error.
    User(u16),
}

// u16::MAX = 65535
const ERROR_INVALID_CONTEXT: u16 = u16::MAX; // 65535
const ERROR_INSUFFICIENT_BALANCE: u16 = u16::MAX - 1; // 65534
const ERROR_INSUFFICIENT_ALLOWANCE: u16 = u16::MAX - 2; // 65533
const ERROR_OVERFLOW: u16 = u16::MAX - 3; // 65532
const ERROR_IDENTICAL_ADDRESSES: u16 = u16::MAX - 4; // 65531
const ERROR_ZERO_ADDRESS: u16 = u16::MAX - 5; // 65530
const ERROR_INSUFFICIENT_LIQUIDITY: u16 = u16::MAX - 6; // 65529
const ERROR_INSUFFICIENT_INPUT_AMOUNT: u16 = u16::MAX - 7; // 65528
const ERROR_INSUFFICIENT_OUTPUT_AMOUNT: u16 = u16::MAX - 8; // 65527
const ERROR_INSUFFICIENT_AMOUNT: u16 = u16::MAX - 9; // 65526
const ERROR_INVALID_PATH: u16 = u16::MAX - 10; // 65525
const ERROR_EXPIRED: u16 = u16::MAX - 11; // 65524
const ERROR_INSUFFICIENT_A_AMOUNT: u16 = u16::MAX - 12; // 65523
const ERROR_INSUFFICIENT_B_AMOUNT: u16 = u16::MAX - 13; // 65522
const ERROR_EXCESSIVE_INPUT_AMOUNT: u16 = u16::MAX - 14; // 65521
const ERROR_EXCESSIVE_AMOUNT: u16 = u16::MAX - 15; // 65520
const ERROR_PAIR_EXISTS: u16 = u16::MAX - 16; // 65519
const ERROR_FORBIDDEN: u16 = u16::MAX - 17; // 65518
const ERROR_INSUFFICIENT_LIQUIDITY_MINTED: u16 = u16::MAX - 18; // 65517
const ERROR_INSUFFICIENT_LIQUIDITY_BURNED: u16 = u16::MAX - 19; // 65516
const ERROR_LOCKED: u16 = u16::MAX - 20; // 65515
const ERROR_INVALID_TO: u16 = u16::MAX - 21; // 65514
const ERROR_K: u16 = u16::MAX - 22; // 65513
const ERROR_INVALID_SIGNATURE: u16 = u16::MAX - 23; // 65512
const ERROR_CANNOT_MINT_TO_ZERO_HASH: u16 = u16::MAX - 24; // 65511
const ERROR_CANNOT_BURN_FROM_ZERO_HASH: u16 = u16::MAX - 25; // 65510
const ERROR_BURN_AMOUNT_EXCEEDS_BALANCE: u16 = u16::MAX - 26; // 65509
const ERROR_NO_RESERVES: u16 = u16::MAX - 27; // 65508
const ERROR_PERIOD_NOT_ELAPSED: u16 = u16::MAX - 28; // 65507
const ERROR_INVALID_TOKEN: u16 = u16::MAX - 29; // 65506
const ERROR_INVALID_DEPOSIT_ENTRY_POINT_NAME: u16 = u16::MAX - 30; // 65505

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        let user_error = match error {
            Error::InvalidContext => ERROR_INVALID_CONTEXT,
            Error::InsufficientBalance => ERROR_INSUFFICIENT_BALANCE,
            Error::InsufficientAllowance => ERROR_INSUFFICIENT_ALLOWANCE,
            Error::Overflow => ERROR_OVERFLOW,
            Error::IdenticalAddresses => ERROR_IDENTICAL_ADDRESSES,
            Error::ZeroAddress => ERROR_ZERO_ADDRESS,
            Error::InsufficientLiquidity => ERROR_INSUFFICIENT_LIQUIDITY,
            Error::InsufficientInputAmount => ERROR_INSUFFICIENT_INPUT_AMOUNT,
            Error::InsufficientOutputAmount => ERROR_INSUFFICIENT_OUTPUT_AMOUNT,
            Error::InsufficientAmount => ERROR_INSUFFICIENT_AMOUNT,
            Error::InvalidPath => ERROR_INVALID_PATH,
            Error::Expired => ERROR_EXPIRED,
            Error::InsufficientAAmount => ERROR_INSUFFICIENT_A_AMOUNT,
            Error::InsufficientBAmount => ERROR_INSUFFICIENT_B_AMOUNT,
            Error::ExcessiveInputAmount => ERROR_EXCESSIVE_INPUT_AMOUNT,
            Error::ExcessiveAmount => ERROR_EXCESSIVE_AMOUNT,
            Error::PairExists => ERROR_PAIR_EXISTS,
            Error::Forbidden => ERROR_FORBIDDEN,
            Error::InsufficientLiquidityMinted => ERROR_INSUFFICIENT_LIQUIDITY_MINTED,
            Error::InsufficientLiquidityBurned => ERROR_INSUFFICIENT_LIQUIDITY_BURNED,
            Error::Locked => ERROR_LOCKED,
            Error::InvalidTo => ERROR_INVALID_TO,
            Error::K => ERROR_K,
            Error::InvalidSignature => ERROR_INVALID_SIGNATURE,
            Error::CannotMintToZeroHash => ERROR_CANNOT_MINT_TO_ZERO_HASH,
            Error::CannotBurnFromZeroHash => ERROR_CANNOT_BURN_FROM_ZERO_HASH,
            Error::BurnAmountExceedsBalance => ERROR_BURN_AMOUNT_EXCEEDS_BALANCE,
            Error::NoReserves => ERROR_NO_RESERVES,
            Error::PeriodNotElapsed => ERROR_PERIOD_NOT_ELAPSED,
            Error::InvalidToken => ERROR_INVALID_TOKEN,
            Error::InvalidDepositEntryPointName => ERROR_INVALID_DEPOSIT_ENTRY_POINT_NAME,
            Error::User(user_error) => user_error,
        };
        ApiError::User(user_error)
    }
}
