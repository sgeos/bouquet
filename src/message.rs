use alloc::{ string::String, };

#[derive(Debug)]
pub enum Message {
  Initialize,
  Terminate,
  Update(usize),
  Log(String),
}

impl Message {
  pub fn log<S: Into<String>>(s: S) -> Message {
    Message::Log(s.into())
  }
}

impl Clone for Message {
  fn clone(&self) -> Message {
    match self {
      Message::Initialize => Message::Initialize,
      Message::Terminate => Message::Terminate,
      Message::Update(n) => Message::Update(*n),
      Message::Log(s) => Message::Log(s.clone()),
    }
  }
}

