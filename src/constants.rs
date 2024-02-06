pub enum Method {
    REGISTER,
    INVITE,
    ACK,
    BYE,
    UPDATE,
    OPTIONS,
    CANCEL,
    INFO,
    PRACK,
    MESSAGE,
    SUBSCRIBE,
    NOTIFY,
    REFER,
    PUBLISH,
}

pub enum Transport {
    UDP,
    TCP,
    TLS,
}
