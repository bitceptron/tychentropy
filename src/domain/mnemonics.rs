
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop, Default)]
pub enum MnemonicLength {
    Twelve,
    Fifteen,
    Eighteen,
    TwentyOne,
    #[default]
    TwentyFour,
}
