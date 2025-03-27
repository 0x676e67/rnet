mod client;
mod request;
mod ws;

pub use self::client::{ClientParams, UpdateClientParams};
pub use self::request::RequestParams;
pub use self::ws::WebSocketParams;

#[macro_export]
macro_rules! extract_option {
    ($ob:expr, $params:expr, $field:ident) => {
        if let Ok(value) = $ob.get_item(stringify!($field)) {
            $params.$field = value.extract()?;
        }
    };
}
