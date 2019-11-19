use byteorder::ByteOrder;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::{BuildHasher, Hash};
use std::io;

use crate::writer::{DbusWrite, DbusWriter};

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn type_code_basic_types() {
        assert_eq!("h", UnixFd(1).to_type_code());
        assert_eq!("y", 1u8.to_type_code());
        assert_eq!("b", true.to_type_code());
        assert_eq!("n", (-10i16).to_type_code());
        assert_eq!("q", 10u16.to_type_code());
        assert_eq!("i", (-20i32).to_type_code());
        assert_eq!("u", 20u32.to_type_code());
        assert_eq!("x", (-30i64).to_type_code());
        assert_eq!("t", (30u64).to_type_code());
        assert_eq!("d", (36.6f64).to_type_code());
        assert_eq!("s", "abc".to_type_code());
        assert_eq!("o", ObjectPath("obj".to_string()).to_type_code());
        assert_eq!("g", Signature("obj".to_string()).to_type_code());
    }

    #[test]
    fn type_code_vec() {
        let vec = vec!["Value1", "Value2"];
        assert_eq!("ass", vec.to_type_code());
    }

    #[test]
    fn type_code_hashmap() {
        let mut hmap = HashMap::new();
        hmap.insert(1u8, "Value_1".to_string());
        hmap.insert(2u8, "Value_2".to_string());
        assert_eq!("{ys}", hmap.to_type_code());
    }
}

pub type TypeCode = String;

/// Marker type for DictEntry enforcing that only basic types can act as key.
/// >  The first single complete type (the "key") must be a basic type rather than a container type.
///    Implementations must not accept [..] dict entries with non-basic-typed keys.
pub trait BasicType {}

impl BasicType for u8 {}
impl BasicType for bool {}
impl BasicType for i16 {}
impl BasicType for u16 {}
impl BasicType for i32 {}
impl BasicType for u32 {}
impl BasicType for i64 {}
impl BasicType for u64 {}
impl BasicType for f64 {}
impl BasicType for UnixFd {}
impl BasicType for String {}
impl BasicType for ObjectPath {}
impl BasicType for Signature {}

pub trait ToTypeCode: Sized {
    fn to_type_code(&self) -> TypeCode;
}

/// The serial of this message, used as a cookie by the sender to identify
/// the reply corresponding to this request. This must not be zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Serial(pub u32);

impl TryFrom<u32> for Serial {
    type Error = io::Error;

    fn try_from(s: u32) -> io::Result<Serial> {
        if s == 0 {
            return Err(io::Error::from(io::ErrorKind::InvalidInput));
        }
        Ok(Serial(s))
    }
}

struct Variant {}

/// VARIANT has ASCII character 'v' as its type code.
/// A marshaled value of type VARIANT will have the signature of a single complete type as part of the value.
/// This signature will be followed by a marshaled value of that type.
impl ToTypeCode for Variant {
    fn to_type_code(&self) -> TypeCode {
        "v".to_string()
        // TODO add remaining variants ?
    }
}

/// An object path is a name used to refer to an object instance.
/// Conceptually, each participant in a D-Bus message exchange may have any number of
/// object instances (think of C++ or Java objects) and each such instance will have a path.
/// Like a filesystem, the object instances in an application form a hierarchical tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectPath(pub String);

// TODO impl from str for ObjectPath see "Valid Object Paths"

impl DbusWrite for ObjectPath {
    fn write<T1, T2>(&self, writer: &mut DbusWriter<T1>, bytes_written: u64) -> Result<u64, io::Error>
    where
        T1: io::Write,
        T2: ByteOrder,
    {
        writer.write_string::<T2>(&self.0, bytes_written)
    }
}

/// based on "Basic type" - Table
impl ToTypeCode for ObjectPath {
    fn to_type_code(&self) -> TypeCode {
        "o".to_string()
    }
}

/// The same as STRING except the length is a single byte
/// (thus signatures have a maximum length of 255) and the
/// content must be a valid signature (see above).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signature(pub String);

// TODO impl from str for Signature see "Valid Signatures"

impl DbusWrite for Signature {
    fn write<T1, T2>(&self, writer: &mut DbusWriter<T1>, bytes_written: u64) -> Result<u64, io::Error>
    where
        T1: io::Write,
        T2: ByteOrder,
    {
        writer.write_string::<T2>(&self.0, bytes_written)
    }
}

/// based on "Basic type" - Table
impl ToTypeCode for Signature {
    fn to_type_code(&self) -> TypeCode {
        "g".to_string()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct UnixFd(pub u32);

/// based on "Basic type" - Table
impl ToTypeCode for UnixFd {
    fn to_type_code(&self) -> TypeCode {
        "h".to_string()
    }
}

/// based on "Basic type" - Table
impl ToTypeCode for u8 {
    fn to_type_code(&self) -> TypeCode {
        "y".to_string()
    }
}

/// based on "Basic type" - Table
impl ToTypeCode for bool {
    fn to_type_code(&self) -> TypeCode {
        "b".to_string()
    }
}

/// based on "Basic type" - Table
impl ToTypeCode for i16 {
    fn to_type_code(&self) -> TypeCode {
        "n".to_string()
    }
}

/// based on "Basic type" - Table
impl ToTypeCode for u16 {
    fn to_type_code(&self) -> TypeCode {
        "q".to_string()
    }
}

/// based on "Basic type" - Table
impl ToTypeCode for i32 {
    fn to_type_code(&self) -> TypeCode {
        "i".to_string()
    }
}

/// based on "Basic type" - Table
impl ToTypeCode for u32 {
    fn to_type_code(&self) -> TypeCode {
        "u".to_string()
    }
}

/// based on "Basic type" - Table
impl ToTypeCode for i64 {
    fn to_type_code(&self) -> TypeCode {
        "x".to_string()
    }
}

/// based on "Basic type" - Table
impl ToTypeCode for u64 {
    fn to_type_code(&self) -> TypeCode {
        "t".to_string()
    }
}

/// based on "Basic type" - Table
impl ToTypeCode for f64 {
    fn to_type_code(&self) -> TypeCode {
        "d".to_string()
    }
}

/// based on "Basic type" - Table
impl ToTypeCode for String {
    fn to_type_code(&self) -> TypeCode {
        "s".to_string()
    }
}

/// based on "Basic type" - Table
impl ToTypeCode for &str {
    fn to_type_code(&self) -> TypeCode {
        "s".to_string()
    }
}

/// based on "Basic type" - Table
/// ARRAY has ASCII character 'a' as type code.
/// The array type code must be followed by a single complete type.
/// The single complete type following the array is the type of each array element.
impl<T: ToTypeCode> ToTypeCode for Vec<T> {
    fn to_type_code(&self) -> TypeCode {
        let mut type_code = String::new();
        type_code.push_str("a");
        for x in self.iter() {
            type_code.push_str(&x.to_type_code());
        }
        type_code
    }
}

impl DbusWrite for Serial {
    fn write<T1, T2>(&self, writer: &mut DbusWriter<T1>, bytes_written: u64) -> Result<u64, io::Error>
    where
        T1: io::Write,
        T2: ByteOrder,
    {
        writer.write_u32::<T2>(self.0, bytes_written)
    }
}

/// /// A DICT_ENTRY works exactly like a struct, but rather than parentheses
/// it uses curly braces, and it has more restrictions.
impl<K, V, S> ToTypeCode for HashMap<K, V, S>
where
    K: BasicType + ToTypeCode + Eq + Hash,
    V: ToTypeCode,
    S: BuildHasher,
{
    fn to_type_code(&self) -> TypeCode {
        let mut type_code = String::new();
        type_code.push_str("{");

        if let Some((key, value)) = self.iter().next() {
            type_code.push_str(&key.to_type_code());
            type_code.push_str(&value.to_type_code());
        }

        type_code.push_str("}");
        type_code
    }
}
