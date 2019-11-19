use std::io;
use byteorder::{ReadBytesExt, ByteOrder};
use crate::type_system::{ObjectPath, Signature};

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait DbusRead<T> {
    fn read<T1, T2>(&self, reader: &mut DbusReader<T1>) -> Result<T>
        where T1: io::Read,
              T2: ByteOrder;
}

pub struct DbusReader<T: io::Read> {
    reader: T,
}

impl<T: io::Read> DbusReader<T> {
    pub fn new(reader: T) -> DbusReader<T> {
        DbusReader {
            reader
        }
    }

    pub fn read_invalid(&self) -> Result<()> {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "HeaderField::Invalid can not be marshaled!"))
    }

    /// A single 8-bit byte.
    pub fn read_u8(&mut self) -> Result<u8> {
        self.reader.read_u8()
    }

    /// As for UINT32, but only 0 and 1 are valid values.
    pub fn read_boolean<T1: ByteOrder>(&mut self) -> Result<bool> {
        let val = self.reader.read_u32::<T1>()?;
        match val {
            0 => Ok(false),
            1 => Ok(true),
            x => {
              let str_err = format!("Invalid boolean `{}`", x);
              Err(io::Error::new(io::ErrorKind::InvalidData, str_err))
            }
        }
    }

    /// 16-bit signed integer in the message's byte order.
    pub fn read_i16<T1: ByteOrder>(&mut self) -> Result<i16> {
        self.reader.read_i16::<T1>()
    }

    /// 16-bit unsigned integer in the message's byte order.
    pub fn read_u16<T1: ByteOrder>(&mut self) -> Result<u16> {
        self.reader.read_u16::<T1>()
    }

    /// 32-bit signed integer in the message's byte order.
    pub fn read_i32<T1: ByteOrder>(&mut self) -> Result<i32> {
        self.reader.read_i32::<T1>()
    }

    /// 32-bit unsigned integer in the message's byte order.
    pub fn read_u32<T1: ByteOrder>(&mut self) -> Result<u32> {
        self.reader.read_u32::<T1>()
    }

    /// 64-bit signed integer in the message's byte order.
    pub fn read_i64<T1: ByteOrder>(&mut self) -> Result<i64> {
        self.reader.read_i64::<T1>()
    }

    /// 64-bit unsigned integer in the message's byte order.
    pub fn read_u64<T1: ByteOrder>(&mut self) -> Result<u64> {
        self.reader.read_u64::<T1>()
    }

    /// A UINT32 indicating the string's length in bytes excluding its terminating nul,
    /// followed by non-nul string data of the given length, followed by a terminating nul byte.
    pub fn read_string<T1: ByteOrder>(&mut self) -> Result<String> {
        let len = self.reader.read_u32::<T1>()?;
        let mut buffer = Vec::with_capacity(len as usize);
        self.reader.read_exact(&mut buffer);

        let str_temination = self.reader.read_u8()?;
        if str_temination != b'\n' {
            let str_err = format!("Invalid termination character `{}`", str_temination);
            return Err(io::Error::new(io::ErrorKind::InvalidData, str_err));
        }

        String::from_utf8(buffer).map_err(|err| {
            let str_err = format!("UT8 error: `{}`", err);
            io::Error::new(io::ErrorKind::InvalidData, str_err)
        })
    }

    /// Exactly the same as STRING except the content must be a valid object path (see above).
    pub fn read_object_path<T1: ByteOrder>(&mut self) -> Result<ObjectPath> {
        let s = self.read_string::<T1>()?;
        Ok(ObjectPath(s))
    }

    /// The same as STRING except the length is a single byte (thus signatures
    /// have a maximum length of 255) and the content must be a valid signature (see above).
    pub fn read_signature<T1: ByteOrder>(&mut self) -> Result<Signature> {
        let s = self.read_string::<T1>()?;
        Ok(Signature(s))
    }

    // A UINT32 giving the length of the array data in bytes, followed by alignment
    // padding to the alignment boundary of the array element type, followed by each array element.
    // pub fn read_array<T1: ByteOrder, T2: DbusRead<T>>(&mut self, a: &[T2]) -> Result<Vec<T2>> {
    //     let len = self.reader.read_u32::<T1>()?;

    //     let vec = Vec::with_capacity(len as usize);
    //     for x in 1..len {
    //         let elem = self.reader.read::<T1>()?;
    //         vec.push(elem);
    //     }
    //     Ok(vec)
    // }

}

