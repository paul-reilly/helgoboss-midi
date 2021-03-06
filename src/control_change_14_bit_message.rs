use crate::{
    extract_high_7_bit_value_from_14_bit_value, extract_low_7_bit_value_from_14_bit_value, Channel,
    ControllerNumber, ShortMessageFactory, U14,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A 14-bit MIDI Control Change message.
///
/// Unlike a [`ShortMessage`] of type [`ShortMessageType::ControlChange`], this one supports 14-bit
/// resolution, that means 16384 different values instead of only 128. MIDI systems emit those by
/// sending 2 short Control Change messages in a row. The [`ControlChange14BitMessageScanner`]
/// can be used to extract such messages from a stream of [`ShortMessage`]s.
///
/// # Example
///
/// ```
/// use helgoboss_midi::{
///     controller_numbers, Channel, ControlChange14BitMessage, RawShortMessage, U14,
/// };
///
/// let msg = ControlChange14BitMessage::new(
///     Channel::new(5),
///     controller_numbers::CHANNEL_VOLUME,
///     U14::new(1057),
/// );
/// assert_eq!(msg.channel().get(), 5);
/// assert_eq!(msg.msb_controller_number().get(), 7);
/// assert_eq!(
///     msg.lsb_controller_number(),
///     controller_numbers::CHANNEL_VOLUME_LSB
/// );
/// use helgoboss_midi::test_util::control_change;
/// assert_eq!(msg.value().get(), 1057);
/// let short_messages: [RawShortMessage; 2] = msg.to_short_messages();
/// assert_eq!(
///     short_messages,
///     [control_change(5, 7, 8), control_change(5, 39, 33)]
/// );
/// ```
///
/// [`ShortMessage`]: trait.ShortMessage.html
/// [`ShortMessageType::ControlChange`]: enum.ShortMessageType.html#variant.ControlChange
/// [`ControlChange14BitMessageScanner`]: struct.ControlChange14BitMessageScanner.html
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ControlChange14BitMessage {
    channel: Channel,
    msb_controller_number: ControllerNumber,
    value: U14,
}

impl ControlChange14BitMessage {
    /// Creates a 14-bit Control Change message.
    ///
    /// # Panics
    ///
    /// This function panics if `msb_controller_number` can't serve as controller number for
    /// transmitting the most significant byte of a 14-bit Control Change message.
    pub fn new(
        channel: Channel,
        msb_controller_number: ControllerNumber,
        value: U14,
    ) -> ControlChange14BitMessage {
        assert!(
            msb_controller_number
                .corresponding_14_bit_lsb_controller_number()
                .is_some()
        );
        ControlChange14BitMessage {
            channel,
            msb_controller_number,
            value,
        }
    }

    /// Returns the channel of this message.
    pub fn channel(&self) -> Channel {
        self.channel
    }

    /// Returns the controller number for transmitting the most significant byte of this message.
    pub fn msb_controller_number(&self) -> ControllerNumber {
        self.msb_controller_number
    }

    /// Returns the controller number for transmitting the least significant byte of this message.
    pub fn lsb_controller_number(&self) -> ControllerNumber {
        self.msb_controller_number
            .corresponding_14_bit_lsb_controller_number()
            .expect("impossible")
    }

    /// Returns the 14-bit value of this message.
    pub fn value(&self) -> U14 {
        self.value
    }

    /// Translates this message into 2 short messages, which need to be sent in a row in order to
    /// encode this 14-bit Control Change message.
    pub fn to_short_messages<T: ShortMessageFactory>(&self) -> [T; 2] {
        [
            T::control_change(
                self.channel,
                self.msb_controller_number(),
                extract_high_7_bit_value_from_14_bit_value(self.value),
            ),
            T::control_change(
                self.channel,
                self.lsb_controller_number(),
                extract_low_7_bit_value_from_14_bit_value(self.value),
            ),
        ]
    }
}

impl<T: ShortMessageFactory> From<ControlChange14BitMessage> for [T; 2] {
    fn from(msg: ControlChange14BitMessage) -> Self {
        msg.to_short_messages()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{channel as ch, controller_number as cn, u14, u7};
    use crate::RawShortMessage;

    #[test]
    fn basics() {
        // Given
        let msg = ControlChange14BitMessage::new(ch(5), cn(2), u14(1057));
        // When
        // Then
        assert_eq!(msg.channel(), ch(5));
        assert_eq!(msg.msb_controller_number(), cn(2));
        assert_eq!(msg.lsb_controller_number(), cn(34));
        assert_eq!(msg.value(), u14(1057));
        let short_msgs = msg.to_short_messages();
        assert_eq!(
            short_msgs,
            [
                RawShortMessage::control_change(ch(5), cn(2), u7(8)),
                RawShortMessage::control_change(ch(5), cn(34), u7(33))
            ]
        );
        let short_msgs_2: [RawShortMessage; 2] = msg.into();
        assert_eq!(short_msgs_2, short_msgs);
    }
}
