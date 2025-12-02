use bytes::Bytes;

use crate::error::Error;

/// Extension trait for [`wreq::Response`] that provides convenient methods for consuming response
/// bodies.
///
/// This trait wraps the underlying [`wreq::Response`] methods and converts their errors to our
/// custom `Error` type.
pub trait ResponseExt {
    /// Returns an error if the body cannot be decoded as valid UTF-8.
    async fn text(self) -> Result<String, Error>;

    /// Returns an error if the encoding is unsupported or decoding fails.
    async fn text_with_charset(self, encoding: impl AsRef<str>) -> Result<String, Error>;

    /// Returns an error if the body is not valid JSON or cannot be deserialized into `T`.
    async fn json<T: serde::de::DeserializeOwned>(self) -> Result<T, Error>;

    /// This method consumes the response and returns all body data as a [`Bytes`] buffer.
    async fn bytes(self) -> Result<Bytes, Error>;
}

impl ResponseExt for wreq::Response {
    #[inline]
    async fn text(self) -> Result<String, Error> {
        self.text().await.map_err(Error::Library)
    }

    #[inline]
    async fn text_with_charset(self, encoding: impl AsRef<str>) -> Result<String, Error> {
        self.text_with_charset(encoding)
            .await
            .map_err(Error::Library)
    }

    #[inline]
    async fn json<T: serde::de::DeserializeOwned>(self) -> Result<T, Error> {
        self.json::<T>().await.map_err(Error::Library)
    }

    #[inline]
    async fn bytes(self) -> Result<Bytes, Error> {
        self.bytes().await.map_err(Error::Library)
    }
}
