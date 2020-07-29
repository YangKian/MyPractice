mod packet;
mod query;
mod util;

pub use error::Result;
pub use util::BytePacketBuffer;
pub use packet::DnsPacket;

mod error;
mod DnsHeader;