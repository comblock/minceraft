#[cfg(feature = "auth")]
pub mod auth;
#[cfg(feature = "inv")]
pub mod inv;
#[cfg(feature = "net")]
pub mod net;
#[cfg(feature = "p47")]
mod p47;

#[cfg(feature = "derive")]
#[cfg(feature = "net")]
pub use minceraft_derive::Packet;
