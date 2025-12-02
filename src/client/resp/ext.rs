use crate::error::Error;
use bytes::Bytes;

pub trait WreqResponseExt {
    fn text(self) -> impl Future<Output = Result<String, Error>>;
    fn text_with_charset(
        self,
        encoding: impl AsRef<str>,
    ) -> impl Future<Output = Result<String, Error>>;
    fn json<T: serde::de::DeserializeOwned>(self) -> impl Future<Output = Result<T, Error>>;
    fn bytes(self) -> impl Future<Output = Result<Bytes, Error>>;
}

impl WreqResponseExt for wreq::Response {
    #[inline]
    fn text(self) -> impl Future<Output = Result<String, Error>> {
        async move { self.text().await.map_err(Error::Library) }
    }

    #[inline]
    fn text_with_charset(
        self,
        encoding: impl AsRef<str>,
    ) -> impl Future<Output = Result<String, Error>> {
        async move {
            self.text_with_charset(encoding)
                .await
                .map_err(Error::Library)
        }
    }

    #[inline]
    fn json<T: serde::de::DeserializeOwned>(self) -> impl Future<Output = Result<T, Error>> {
        async move { self.json::<T>().await.map_err(Error::Library) }
    }

    #[inline]
    fn bytes(self) -> impl Future<Output = Result<Bytes, Error>> {
        async move { self.bytes().await.map_err(Error::Library) }
    }
}
