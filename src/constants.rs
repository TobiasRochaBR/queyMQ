

// _PUBLISH_ Message
pub const MSG_PUBLISH_REQ: u32 = 1001;
pub const MSG_PUBLISH_ACK: u32 = 1002;
// Unacknowledge Message
pub const MSG_NACK_REQ: u32 = 1003;
pub const MSG_NACK_ACK: u32 = 1004;
// Acknowledge Message
pub const MSG_ACK_REQ: u32 = 1005;
pub const MSG_ACK_ACK: u32 = 1006;
// Reject Message
pub const MSG_REJECT_REQ: u32 = 1007;
pub const MSG_REJECT_ACK: u32 = 1008;
// Next Message
pub const MSG_NEXT_REQ: u32 = 1009;
pub const MSG_NEXT_ACK: u32 = 1010;
// Create pub const Channel
pub const CHANNEL_CREATE_REQ: u32 = 1011;
pub const CHANNEL_CREATE_ACK: u32 = 1012;
// Join pub const Channel
pub const CHANNEL_JOIN_REQ: u32 = 1013;
pub const CHANNEL_JOIN_ACK: u32 = 1014;
// Register Consumer
pub const CONSUMER_REGISTER_REQ: u32 = 1015;
pub const CONSUMER_REGISTER_ACK: u32 = 1016;
// Register _PUBLISH_er
pub const PUBLISHER_REGISTER_REQ: u32 = 1017;
pub const PUBLISHER_REGISTER_ACK: u32 = 1018;
// DISTRIBUTE_
pub const MSG_DISTRIBUTE_REQ: u32 = 1019;
pub const MSG_DISTRIBUTE_ACK: u32 = 1020;