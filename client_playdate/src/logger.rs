use {
  alloc::{ format, sync::Arc, vec::Vec, },
  bouquet_ribbon::message::{ Message, MessageSendee, },
  client_core::{
    program_state::ProgramState,
    message::{ ClientMessage, DebugMessage, ServerMessage, },
  },
  crankstart::{ system::System },
};

pub struct Logger { }

impl Logger {
  pub fn new() -> Logger {
    Logger { }
  }
}

impl
  MessageSendee::<ProgramState, ClientMessage, ServerMessage, DebugMessage>
  for Logger
{
  fn send(
    &mut self,
    message: Arc::<Message::<ClientMessage, ServerMessage, DebugMessage>>,
    _program_state: &mut ProgramState,
  ) -> Vec<Arc::<Message::<ClientMessage, ServerMessage, DebugMessage>>>
  {
    let result = Vec::new();
    match &*message {
      Message::Initialize => log("Initializing program."),
      Message::Terminate => log("Terminating program."),
      //Message::Update(duration) => log(format!("Update: {}", duration)),
      Message::Debug(DebugMessage::Log(s)) => log(format!("{}", s)),
      //_ => log(format!("{:?}", message)),
      _ => (),
    }
    result
  }
}

fn log<S: AsRef<str>>(message: S) {
  System::log_to_console(message.as_ref());
}

