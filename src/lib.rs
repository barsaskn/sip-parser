mod constants;
mod sip_uri;

pub use constants::Method;
pub use constants::Transport;
pub use sip_uri::SipUri;


pub struct SipUser {
    pub uri: SipUri,
    pub tag: String
}
pub struct SipPacket {
    pub method: Method,
    pub transport: Transport,
    pub max_forwards: u16,
    pub via_user: SipUser,
    pub to_user: SipUser,
    pub call_id: String, //may change
    pub user_agent: String,
    pub expires: u32,
    pub contact: SipUser
}

