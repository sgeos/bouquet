use alloc::{ fmt::Debug, string::String, };

#[derive(Debug, Clone)]
pub enum ClientMessage
{
  Message,
}

#[derive(Debug, Clone)]
pub enum ServerMessage
{
  Message,
}

#[derive(Debug)]
pub enum DebugMessage
{
  Log(String),
  Message,
}

impl Clone for DebugMessage {
  fn clone(&self) -> DebugMessage {
    match self {
      DebugMessage::Log(s) => DebugMessage::Log(s.clone()),
      DebugMessage::Message => DebugMessage::Message,
    }
  }
}

