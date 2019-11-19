use crate::type_system::{ObjectPath, Signature};
use byteorder::{ByteOrder, WriteBytesExt};
use std::io;

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait DbusWrite {
    fn write<T1, T2>(&self, writer: &mut DbusWriter<T1>, bytes_written: u64) -> Result<u64>
    where
        T1: io::Write,
        T2: ByteOrder;
}

pub struct DbusWriter<T: io::Write> {
    writer: T,
}

impl<T: io::Write> DbusWriter<T> {
    pub fn new(writer: T) -> DbusWriter<T> {
        DbusWriter { writer }
    }

    /// Add padding to multiple of 8
    pub fn write_padding(&mut self, bytes_written: u64, align_to: u64) -> Result<u8> {
        let padding_length = (align_to - (bytes_written % align_to)) % align_to;
        for _ in 0..padding_length {
            self.write_u8(0)?;
        }
        Ok(padding_length as u8)
    }

    pub fn write_invalid(&self) -> Result<()> {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "HeaderField::Invalid can not be marshaled!",
        ))
    }

    /// A single 8-bit byte.
    pub fn write_u8(&mut self, n: u8) -> Result<u64> {
        self.writer.write_u8(n)?;
        Ok(1)
    }

    /// As for UINT32, but only 0 and 1 are valid values.
    pub fn write_boolean<T1: ByteOrder>(&mut self, b: bool, bytes_written: u64) -> Result<u64> {
        self.write_u32::<T1>(b as u32, bytes_written)
    }

    /// 16-bit signed integer in the message's byte order.
    pub fn write_i16<T1: ByteOrder>(&mut self, i: i16, bytes_written: u64) -> Result<u64> {
        self.write_padding(bytes_written, 2)?;
        self.writer.write_i16::<T1>(i)?;
        Ok(16 / 8)
    }

    /// 16-bit unsigned integer in the message's byte order.
    pub fn write_u16<T1: ByteOrder>(&mut self, u: u16, bytes_written: u64) -> Result<u64> {
        self.write_padding(bytes_written, 2)?;
        self.writer.write_u16::<T1>(u)?;
        Ok(16 / 8)
    }

    /// 32-bit signed integer in the message's byte order.
    pub fn write_i32<T1: ByteOrder>(&mut self, i: i32, bytes_written: u64) -> Result<u64> {
        self.write_padding(bytes_written, 4)?;
        self.writer.write_i32::<T1>(i)?;
        Ok(32 / 8)
    }

    /// 32-bit unsigned integer in the message's byte order.
    pub fn write_u32<T1: ByteOrder>(&mut self, u: u32, bytes_written: u64) -> Result<u64> {
        self.write_padding(bytes_written, 4)?;
        self.writer.write_u32::<T1>(u)?;
        Ok(32 / 8)
    }

    /// 64-bit signed integer in the message's byte order.
    pub fn write_i64<T1: ByteOrder>(&mut self, i: i64, bytes_written: u64) -> Result<u64> {
        self.write_padding(bytes_written, 8)?;
        self.writer.write_i64::<T1>(i)?;
        Ok(64 / 8)
    }

    /// 64-bit unsigned integer in the message's byte order.
    pub fn write_u64<T1: ByteOrder>(&mut self, u: u64, bytes_written: u64) -> Result<u64> {
        self.write_padding(bytes_written, 8)?;
        self.writer.write_u64::<T1>(u)?;
        Ok(64 / 8)
    }

    /// A UINT32 indicating the string's length in bytes excluding its terminating nul,
    /// followed by non-nul string data of the given length, followed by a terminating nul byte.
    pub fn write_string<T1: ByteOrder>(&mut self, s: &str, bytes_written: u64) -> Result<u64> {
        let mut bytes_written = 0;
        bytes_written += self.write_u32::<T1>(s.len() as u32, bytes_written)?;

        let s_bytes = s.as_bytes();
        self.writer.write_all(s_bytes)?;
        bytes_written += s_bytes.len() as u64;

        bytes_written += self.write_u8(b'\n')?;

        Ok(bytes_written)
    }

    /// Exactly the same as STRING except the content must be a valid object path (see above).
    pub fn write_object_path<T1: ByteOrder>(&mut self, object_path: ObjectPath, bytes_written: u64) -> Result<u64> {
        self.write_string::<T1>(&object_path.0, bytes_written)
    }

    /// The same as STRING except the length is a single byte (thus signatures
    /// have a maximum length of 255) and the content must be a valid signature (see above).
    pub fn write_signature<T1: ByteOrder>(&mut self, signature: Signature, bytes_written: u64) -> Result<u64> {
        self.write_string::<T1>(&signature.0, bytes_written)
    }

    /// A UINT32 giving the length of the array data in bytes, followed by alignment
    /// padding to the alignment boundary of the array element type, followed by each array element.
    pub fn write_array<T1: ByteOrder, T2: DbusWrite>(&mut self, a: &[T2], bytes_written: u64) -> Result<u64> {
        let mut bytes_written = 0;

        bytes_written += self.write_u32::<T1>(a.len() as u32, bytes_written)?;

        for x in a {
            bytes_written += x.write::<_, T1>(self, bytes_written)?;
        }

        Ok(bytes_written)
    }
}
