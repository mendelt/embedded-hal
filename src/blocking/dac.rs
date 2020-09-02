//! Blocking DAC trait for single channel digital to analog conversion

/// A single DAC channel. Word is the type used to represent a single sample, this would typically
/// be u8, u16 or u32.
/// Note that not all bits will always be used. A 12 bit DAC for example will use u16. In this case
/// the DAC should use the most significant bits and ignore any unused bits to make it easier
/// to exchange dacs with the same Word length but different numbers of bits.
pub trait DAC {
    /// Error type returned by DAC methods
    type Error;

    /// Word length, should be u8, u16 or u32 depending on precision of the DAC
    type Word;

    /// Set the output of the DAC
    fn try_set_output(&mut self, value: Self::Word) -> Result<(), Self::Error>;
}
