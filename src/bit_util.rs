use crate::{Channel, U14, U7};

pub fn extract_high_7_bit_value_from_14_bit_value(value: U14) -> U7 {
    U7(((value.get() >> 7) & 0x7f) as u8)
}

pub fn extract_low_7_bit_value_from_14_bit_value(value: U14) -> U7 {
    U7((value.get() & 0x7f) as u8)
}

pub fn build_14_bit_value_from_two_7_bit_values(high: U7, low: U7) -> U14 {
    U14((u16::from(high) << 7) | u16::from(low))
}

pub fn build_status_byte(type_byte: u8, channel: Channel) -> u8 {
    type_byte | channel.get()
}

pub fn extract_channel_from_status_byte(byte: u8) -> Channel {
    Channel(byte & 0x0f)
}
