use {
    crate::{
        message_bus::SystemMessage,
        rendering::{BouquetFloat, Vec2},
    },
    alloc::{fmt::Debug, string::String},
};

// Future Extension: As the engine grows, more and more message variants may
// be added to the Message enum. Depending on the engine structure, it might
// make sense to subdivide the messages into sub-enums (like ClientMessage,
// ServerMessage, etc.) and then use those sub-enums within the main Message
// enum. This can keep your code organized and manageable.

/*
#[derive(Debug, Clone)]
pub enum Message {
    // ... existing messages
    Client(ClientMessage),
    // ... more categories and messages
}

#[derive(Debug, Clone)]
pub enum ClientMessage {
    Connect,
    Disconnect,
    // ... more client-specific messages
}
 */

#[derive(Debug, Clone)]
pub enum Message {
    Initialize,
    Terminate,
    Update(BouquetFloat),
    Resize(Vec2),
    DebugLog(String),
    // ... more messages
}

impl SystemMessage for Message {
    fn is_initialize(&self) -> bool {
        matches!(self, Message::Initialize)
    }

    fn is_terminate(&self) -> bool {
        matches!(self, Message::Terminate)
    }
}
