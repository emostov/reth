use reth_storage_errors::db::DatabaseError;
use std::fmt::Debug;

/// Trait that will transform the data to be saved in the DB in a (ideally) compressed format
pub trait Compress: Send + Sync + Sized + Debug {
    /// Compressed type.
    type Compressed: bytes::BufMut
        + AsRef<[u8]>
        + AsMut<[u8]>
        + Into<Vec<u8>>
        + Default
        + Send
        + Sync
        + Debug;

    /// If the type cannot be compressed, return its inner reference as `Some(self.as_ref())`
    fn uncompressable_ref(&self) -> Option<&[u8]> {
        None
    }

    /// Compresses data going into the database.
    fn compress(self) -> Self::Compressed {
        let mut buf = Self::Compressed::default();
        self.compress_to_buf(&mut buf);
        buf
    }

    /// Compresses data to a given buffer.
    fn compress_to_buf<B: bytes::BufMut + AsMut<[u8]>>(self, buf: &mut B);
}

/// Trait that will transform the data to be read from the DB.
pub trait Decompress: Send + Sync + Sized + Debug {
    /// Decompresses data coming from the database.
    fn decompress<B: AsRef<[u8]>>(value: B) -> Result<Self, DatabaseError>;

    /// Decompresses owned data coming from the database.
    fn decompress_owned(value: Vec<u8>) -> Result<Self, DatabaseError> {
        Self::decompress(value)
    }
}

/// Trait that will transform the data to be saved in the DB.
pub trait Encode: Send + Sync + Sized + Debug {
    /// Encoded type.
    type Encoded: AsRef<[u8]> + Into<Vec<u8>> + Send + Sync + Ord + Debug;

    /// Encodes data going into the database.
    fn encode(self) -> Self::Encoded;
}

/// Trait that will transform the data to be read from the DB.
pub trait Decode: Send + Sync + Sized + Debug {
    /// Decodes data coming from the database.
    fn decode<B: AsRef<[u8]>>(value: B) -> Result<Self, DatabaseError>;
}
