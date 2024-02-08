mod sip_constants;
mod sip_uri;
mod sip_user;

pub use sip_constants::Method;
pub use sip_constants::Transport;
pub use sip_uri::SipUri;
pub use sip_user::SipUser;

pub struct SipPacket {
    pub method: Method,
    pub transport: Transport,
    pub max_forwards: u16,
    pub via_user: SipUser,
    pub to_user: SipUser,
    pub call_id: String, //may change
    pub user_agent: String,
    pub expires: u32,
    pub contact: SipUser,
}
