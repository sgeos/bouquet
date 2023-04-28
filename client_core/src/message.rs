use {
  alloc::{ fmt::Debug, },
  rhai::{ ImmutableString, },
};

#[derive(Debug)]
pub enum ClientMessage
{
  Message,
}

#[derive(Debug)]
pub enum ServerMessage
{
  Message,
}

#[derive(Debug)]
pub enum DebugMessage
{
  Log(ImmutableString),
  Message,
}

