pub(crate) mod errors;
mod handle_default;
mod handle_dfc;
mod handle_vars;
mod handler;
pub(crate) mod utils;

pub use errors::WebsocketHandlingError;
pub use handler::handle_websocket_message;
