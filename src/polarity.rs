/// Polarity values
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum Polarity {
    /// Positive polarity (+1)
    Positive = 1,
    /// Negative polarity (-1)
    Negative = -1,
    /// Undefined polarity (0)
    Undefined = 0,
}
