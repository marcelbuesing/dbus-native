use crate::writer::{DbusWrite, DbusWriter};
use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;
use std::str::FromStr;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn interface_name() {
        let large_string = String::from_utf8(vec![b'X'; 256]).unwrap();
        assert_eq!(
            Err(InterfaceNameError::ExceedsMaxSize),
            InterfaceName::from_str(large_string.as_str())
        );

        assert_eq!(
            Err(InterfaceNameError::ElementsMustContainChars),
            InterfaceName::from_str("Elements..MissingChars")
        );

        assert_eq!(
            Err(InterfaceNameError::MustContainPeriod),
            InterfaceName::from_str("MissingPeriod")
        );

        assert_eq!(
            Err(InterfaceNameError::MustNotBeginWithPeriod),
            InterfaceName::from_str(".Must.Not.Start.With.Period")
        );

        assert_eq!(
            Err(InterfaceNameError::ElementMustNotBeginWithDigit),
            InterfaceName::from_str("Must.Not.Start.With.9Digit")
        );

        assert_eq!(
            Err(InterfaceNameError::InvalidCharacter('|')),
            InterfaceName::from_str("Invalid.C|har")
        );

        let valid_string = "Valid.Interface";
        assert_eq!(
            Ok(InterfaceName(valid_string.to_string())),
            InterfaceName::from_str(valid_string)
        );
    }

    #[test]
    fn bus_name() {
        let large_string = String::from_utf8(vec![b'X'; 256]).unwrap();
        assert_eq!(
            Err(BusNameError::ExceedsMaxSize),
            BusName::from_str(large_string.as_str())
        );

        assert_eq!(
            Err(BusNameError::ElementsMustContainChars),
            BusName::from_str("Elements..MissingChars")
        );

        assert_eq!(
            Err(BusNameError::MustContainPeriod),
            BusName::from_str("MissingPeriod")
        );

        assert_eq!(
            Err(BusNameError::MustNotBeginWithPeriod),
            BusName::from_str(".Must.Not.Start.With.Period")
        );

        assert_eq!(
            Err(BusNameError::InvalidCharacter('|')),
            BusName::from_str("Invalid.C|har")
        );

        let valid_string = "Valid.BusName";
        assert_eq!(
            Ok(BusName(valid_string.to_string())),
            BusName::from_str(valid_string)
        );
    }

    #[test]
    fn member_name() {
        let large_string = String::from_utf8(vec![b'X'; 256]).unwrap();
        assert_eq!(
            Err(MemberNameError::ExceedsMaxSize),
            MemberName::from_str(large_string.as_str())
        );

        assert_eq!(
            Err(MemberNameError::InvalidCharacter('|')),
            MemberName::from_str("InvalidC|har")
        );

        assert_eq!(
            Err(MemberNameError::MustNotContainPeriod),
            MemberName::from_str("Contains.Period")
        );

        assert_eq!(
            Err(MemberNameError::MustNotBeginWithDigit),
            MemberName::from_str("1MustNotBeginWithDigit")
        );

        let valid_string = "ValidMemberName";
        assert_eq!(
            Ok(MemberName(valid_string.to_string())),
            MemberName::from_str(valid_string)
        );
    }

    #[test]
    fn error_name() {
        let large_string = String::from_utf8(vec![b'X'; 256]).unwrap();
        assert_eq!(
            Err(ErrorNameError::ExceedsMaxSize),
            ErrorName::from_str(large_string.as_str())
        );

        assert_eq!(
            Err(ErrorNameError::ElementsMustContainChars),
            ErrorName::from_str("Elements..MissingChars")
        );

        assert_eq!(
            Err(ErrorNameError::MustContainPeriod),
            ErrorName::from_str("MissingPeriod")
        );

        assert_eq!(
            Err(ErrorNameError::MustNotBeginWithPeriod),
            ErrorName::from_str(".Must.Not.Start.With.Period")
        );

        assert_eq!(
            Err(ErrorNameError::ElementMustNotBeginWithDigit),
            ErrorName::from_str("Must.Not.Start.With.9Digit")
        );

        assert_eq!(
            Err(ErrorNameError::InvalidCharacter('|')),
            ErrorName::from_str("Invalid.C|har")
        );

        let valid_string = "Valid.Error.Name";
        assert_eq!(
            Ok(ErrorName(valid_string.to_string())),
            ErrorName::from_str(valid_string)
        );
    }
}

/// There is a maximum name length of 255 which applies to bus names, interfaces, and members.
pub const MAX_NAME_LENGHT: usize = 255;

lazy_static! {
    /// The special message bus name org.freedesktop.DBus responds to a number of additional messages at the object path /org/freedesktop/DBus.
    static ref ORG_FREEDESKTOP_DBUS: BusName = BusName("org.freedesktop.DBus".to_string());
}

/// Connections have one or more bus names associated with them.
/// A connection has exactly one bus name that is a unique connection name.
/// The unique connection name remains with the connection for its entire lifetime.
/// A bus name is of type STRING, meaning that it must be valid UTF-8.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InterfaceName(String);

impl DbusWrite for InterfaceName {
    fn write<T1, T2>(&self, writer: &mut DbusWriter<T1>) -> Result<u64, io::Error>
    where
        T1: io::Write,
        T2: ByteOrder,
    {
        writer.write_string::<T2>(&self.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InterfaceNameError {
    /// There is a maximum name length of 255
    ExceedsMaxSize,

    /// Interface names are composed of 1 or more elements separated by a period ('.') character.
    /// All elements must contain at least one character.
    ElementsMustContainChars,

    /// Each element must only contain the ASCII characters "[A-Z][a-z][0-9]_-", with "-" discouraged in new bus names.
    /// Only elements that are part of a unique connection name may begin with a digit, elements in other bus names must not begin with a digit.
    InvalidCharacter(char),

    /// Bus names must contain at least one '.' (period) character (and thus at least two elements).
    MustContainPeriod,

    /// Bus names must not begin with a '.' (period) character.
    MustNotBeginWithPeriod,

    /// Elements must not begin with digit.
    ElementMustNotBeginWithDigit,
}

fn is_valid_interface_name_char(c: char) -> bool {
    match c {
        'A'..='Z' => true,
        'a'..='z' => true,
        '0'..='9' => true,
        '-' => true,
        '.' => true,
        _ => false,
    }
}

impl FromStr for InterfaceName {
    type Err = InterfaceNameError;
    fn from_str(s: &str) -> Result<InterfaceName, InterfaceNameError> {
        if s.len() > MAX_NAME_LENGHT {
            return Err(InterfaceNameError::ExceedsMaxSize);
        }

        if s.starts_with('.') {
            return Err(InterfaceNameError::MustNotBeginWithPeriod);
        }

        let mut last_period_position = 0;
        for (i, c) in s.char_indices() {
            if !is_valid_interface_name_char(c) {
                return Err(InterfaceNameError::InvalidCharacter(c));
            }

            if c == '.' {
                if last_period_position + 1 == i {
                    return Err(InterfaceNameError::ElementsMustContainChars);
                }
                last_period_position = i;
            } else {
                // start of new element
                if last_period_position + 1 == i && c.is_digit(10) {
                    return Err(InterfaceNameError::ElementMustNotBeginWithDigit);
                }
            }
        }

        if last_period_position == 0 {
            return Err(InterfaceNameError::MustContainPeriod);
        }

        Ok(InterfaceName(s.to_string()))
    }
}

/// Connections have one or more bus names associated with them.
/// A connection has exactly one bus name that is a unique connection name.
/// The unique connection name remains with the connection for its entire lifetime.
/// A bus name is of type STRING, meaning that it must be valid UTF-8.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BusName(String);

impl DbusWrite for BusName {
    fn write<T1, T2>(&self, writer: &mut DbusWriter<T1>) -> Result<u64, io::Error>
    where
        T1: io::Write,
        T2: ByteOrder,
    {
        writer.write_string::<T2>(&self.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BusNameError {
    /// There is a maximum name length of 255
    ExceedsMaxSize,

    /// Bus names are composed of 1 or more elements separated by a period ('.') character.
    /// All elements must contain at least one character.
    ElementsMustContainChars,

    /// Each element must only contain the ASCII characters "[A-Z][a-z][0-9]_-", with "-" discouraged in new bus names.
    /// Only elements that are part of a unique connection name may begin with a digit, elements in other bus names must not begin with a digit.
    InvalidCharacter(char),

    /// Bus names must contain at least one '.' (period) character (and thus at least two elements).
    MustContainPeriod,

    /// Bus names must not begin with a '.' (period) character.
    MustNotBeginWithPeriod,
    // TODO
    // Only elements that are part of a unique connection name may begin with a digit, elements in other bus names must not begin with a digit.
}

fn is_valid_bus_name_char(c: char) -> bool {
    match c {
        'A'..='Z' => true,
        'a'..='z' => true,
        '0'..='9' => true,
        '-' => true,
        '.' => true,
        _ => false,
    }
}

impl FromStr for BusName {
    type Err = BusNameError;
    fn from_str(s: &str) -> Result<BusName, BusNameError> {
        if s.len() > MAX_NAME_LENGHT {
            return Err(BusNameError::ExceedsMaxSize);
        }

        if s.starts_with('.') {
            return Err(BusNameError::MustNotBeginWithPeriod);
        }

        let mut last_period_position = 0;
        for (i, c) in s.char_indices() {
            if !is_valid_bus_name_char(c) {
                return Err(BusNameError::InvalidCharacter(c));
            }

            if c == '.' {
                if last_period_position + 1 == i {
                    return Err(BusNameError::ElementsMustContainChars);
                }
                last_period_position = i;
            }
        }

        if last_period_position == 0 {
            return Err(BusNameError::MustContainPeriod);
        }

        Ok(BusName(s.to_string()))
    }
}

/// Member (i.e. method or signal) names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemberName(String);

impl DbusWrite for MemberName {
    fn write<T1, T2>(&self, writer: &mut DbusWriter<T1>) -> Result<u64, io::Error>
    where
        T1: io::Write,
        T2: ByteOrder,
    {
        writer.write_string::<T2>(&self.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MemberNameError {
    /// There is a maximum name length of 255
    ExceedsMaxSize,

    /// Must only contain the ASCII characters "[A-Z][a-z][0-9]_".
    InvalidCharacter(char),

    /// Must not contain the '.' (period) character.
    MustNotContainPeriod,

    /// May not begin with a digit.
    MustNotBeginWithDigit,

    /// Must be at least 1 byte in length.
    // TODO verify this
    MustBeAtLeastOneByte,
}

fn is_valid_member_name_char(c: char) -> bool {
    match c {
        'A'..='Z' => true,
        'a'..='z' => true,
        '0'..='9' => true,
        '-' => true,
        _ => false,
    }
}

impl FromStr for MemberName {
    type Err = MemberNameError;
    fn from_str(s: &str) -> Result<MemberName, MemberNameError> {
        if s.len() > MAX_NAME_LENGHT {
            return Err(MemberNameError::ExceedsMaxSize);
        }

        if s.starts_with(|c: char| c.is_digit(10)) {
            return Err(MemberNameError::MustNotBeginWithDigit);
        }

        for c in s.chars() {
            if c == '.' {
                return Err(MemberNameError::MustNotContainPeriod);
            }

            if !is_valid_member_name_char(c) {
                return Err(MemberNameError::InvalidCharacter(c));
            }
        }

        Ok(MemberName(s.to_string()))
    }
}

/// Error names have the same restrictions as interface names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorName(String);

impl DbusWrite for ErrorName {
    fn write<T1, T2>(&self, writer: &mut DbusWriter<T1>) -> Result<u64, io::Error>
    where
        T1: io::Write,
        T2: ByteOrder,
    {
        writer.write_string::<T2>(&self.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ErrorNameError {
    /// There is a maximum name length of 255
    ExceedsMaxSize,

    /// Error names are composed of 1 or more elements separated by a period ('.') character.
    /// All elements must contain at least one character.
    ElementsMustContainChars,

    /// Each element must only contain the ASCII characters "[A-Z][a-z][0-9]_-",
    /// with "-" discouraged in new bus names. Only elements that are part of a
    /// unique connection name may begin with a digit, elements in other bus names
    /// must not begin with a digit.
    InvalidCharacter(char),

    /// Error names must contain at least one '.' (period) character (and thus at least two elements).
    MustContainPeriod,

    /// Error names must not begin with a '.' (period) character.
    MustNotBeginWithPeriod,

    /// Elements must not begin with digit.
    ElementMustNotBeginWithDigit,
}

impl FromStr for ErrorName {
    type Err = ErrorNameError;
    fn from_str(s: &str) -> Result<ErrorName, ErrorNameError> {
        match InterfaceName::from_str(s) {
            Ok(iface) => Ok(ErrorName(iface.0)),
            Err(InterfaceNameError::ExceedsMaxSize) => Err(ErrorNameError::ExceedsMaxSize),
            Err(InterfaceNameError::ElementsMustContainChars) => {
                Err(ErrorNameError::ElementsMustContainChars)
            }
            Err(InterfaceNameError::InvalidCharacter(c)) => {
                Err(ErrorNameError::InvalidCharacter(c))
            }
            Err(InterfaceNameError::MustContainPeriod) => Err(ErrorNameError::MustContainPeriod),
            Err(InterfaceNameError::MustNotBeginWithPeriod) => {
                Err(ErrorNameError::MustNotBeginWithPeriod)
            }
            Err(InterfaceNameError::ElementMustNotBeginWithDigit) => {
                Err(ErrorNameError::ElementMustNotBeginWithDigit)
            }
        }
    }
}
