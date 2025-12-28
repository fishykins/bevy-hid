use bevy::reflect::Reflect;
use serde::Deserialize;

/// Splits input into four components.
#[derive(Debug, Clone, PartialEq, Deserialize, Reflect)]
pub enum InputType {
    /// A single button.
    Button(ButtonPointer),
    /// A single axis.
    Axis(AxisPointer),
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Reflect)]
pub struct ButtonPointer(pub u8, pub ButtonQuery);

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Default, Reflect)]
pub struct AxisPointer {
    /// The buffer index on which the fine value is stored. This combines the the coarse value to get the total axis value.
    pub fine: u8,
    /// The buffer on which the coarse value is stored. This is usually the one sequentially after the fine parameter.
    pub coarse: u8,
    /// The number of coarse 'octaves' in the buffer. Low-fidelity axis will have 4, while top-end devices can go all the way up to 256.
    pub octaves: u16,
    /// Inverting of an axis.
    pub inverted: bool,
    /// An absolute axis will only have values between 0 and 1 (once normalized).
    pub abs: bool,
}

/// Rules for collecting button data from input buffers.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Reflect)]
pub enum ButtonQuery {
    /// A bit query does an & opperation on the input buffer.
    Bit(u8),
    /// Eq requires the buffer to be an exact match to the provided value.
    Eq(u8),
}
