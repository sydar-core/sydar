use crate::Notification;

pub type ChannelConnection = sydar_notify::connection::ChannelConnection<Notification>;
pub use sydar_notify::connection::ChannelType;
