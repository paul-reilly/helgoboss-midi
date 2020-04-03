use crate::{Channel, SevenBitValue, U14};

pub(crate) fn extract_high_nibble_from_byte(byte: u8) -> Channel {
    unsafe { Channel::new_unchecked((byte >> 4) & 0x0f) }
}

pub(crate) fn extract_low_nibble_from_byte(byte: u8) -> Channel {
    unsafe { Channel::new_unchecked(byte & 0x0f) }
}

pub(crate) fn extract_high_7_bit_value_from_14_bit_value(value: U14) -> SevenBitValue {
    ((u16::from(value) >> 7) & 0x7f) as u8
}

pub(crate) fn extract_low_7_bit_value_from_14_bit_value(value: U14) -> SevenBitValue {
    (u16::from(value) & 0x7f) as u8
}

// TODO Here we should use raw types instead of channel types!
pub(crate) fn build_byte_from_nibbles(high_nibble: Channel, low_nibble: Channel) -> u8 {
    (u8::from(high_nibble) << 4) | u8::from(low_nibble)
}

pub(crate) fn build_14_bit_value_from_two_7_bit_values(
    high: SevenBitValue,
    low: SevenBitValue,
) -> U14 {
    debug_assert!(high <= 0x7f);
    debug_assert!(low <= 0x7f);
    unsafe { U14::new_unchecked(((high as u16) << 7) | (low as u16)) }
}

pub(crate) fn with_low_nibble_added(byte: u8, low_nibble: Channel) -> u8 {
    byte | u8::from(low_nibble)
}