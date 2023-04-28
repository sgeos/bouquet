use {
  alloc::{ string::String, string::ToString, sync::Arc, vec::Vec, },
  bouquet_ribbon::message::{ Message, MessageSendee, },
  client_core::{
    program_state::ProgramState,
    message::{ ClientMessage, DebugMessage, ServerMessage, },
  },
  core::{ convert::TryInto, },
  rhai::{ INT, },
};

pub struct Terminal { }

impl Terminal {
  pub fn new() -> Terminal {
    Terminal { }
  }
}

impl
  MessageSendee::<ProgramState, ClientMessage, ServerMessage, DebugMessage>
  for Terminal
{
  fn send(
    &mut self,
    message: Arc::<Message::<ClientMessage, ServerMessage, DebugMessage>>,
    program_state: &mut ProgramState,
  ) -> Vec<Arc::<Message::<ClientMessage, ServerMessage, DebugMessage>>>
  {
    let mut result = Vec::new();
    match &*message {
      Message::Initialize => log("Initializing program."),
      Message::Terminate => log("Terminating program."),
      Message::Update(duration) => {
        if program_state.persistent_data.done {
          return result;
        }
        prompt(format!("{:>6}> ", duration));
        let input = readline();
        let mut args = input.trim().split_whitespace().peekable();
        let command = args.next().unwrap_or("");
        match command.to_lowercase().as_str() {
          "client" => {
            result.push(Arc::new(Message::Client(ClientMessage::Message)));
          },
          "debug" => {
            result.push(Arc::new(Message::Debug(DebugMessage::Message)));
          },
          "done" => {
            log(format!("Done: {}", program_state.persistent_data.done));
          },
          "exit" | "quit" => {
            result.push(Arc::new(Message::Terminate));
          },
          "frame" => {
            log(format!("Frame: {}", program_state.last_frame_data.frame));
          },
          "initialize" => result.push(Arc::new(Message::Initialize)),
          "log" => {
            let output =
              args
              .fold(String::new(), |a, b| a + " " + b).trim().to_string();
            result.push(Arc::new(Message::Debug(DebugMessage::Log(output.into()))));
          },
          "server" => {
            result.push(Arc::new(Message::Server(ServerMessage::Message)));
          },
          "terminate" => result.push(Arc::new(Message::Terminate)),
          "update" => {
            let duration: INT = args.next()
              .unwrap_or("0").parse().unwrap_or(0);
            result.push(Arc::new(Message::Update(duration)))
          },
          "" => (),
          _ => {
            log(format!("Echo: {} ", command));
            if args.peek().is_some() {
              for arg in args {
                prompt(format!("{} ", arg));
              }
              log("");
            }
          },
        }
      },
      Message::Debug(DebugMessage::Log(s)) => log(format!("{}", s)),
      _ => log(format!("{:?}", message)),
    }
    result
  }
}

fn log<S: AsRef<str>>(message: S) {
  let format = format!("{}\n\0", message.as_ref());
  unsafe {
    libc::printf(format.as_ptr() as *const _);
  }
}

fn prompt<S: AsRef<str>>(message: S) {
  let format = format!("{}\0", message.as_ref());
  unsafe {
    libc::printf(format.as_ptr() as *const _);
  }
}

fn readline() -> String {
  let mut buffer = Vec::new();
  loop {
    let c = unsafe { libc::getchar() };
    if c == '\n' as i32 {
      return String::from_utf8(buffer).unwrap_or(String::from(""));
    }
    buffer.push(c.try_into().unwrap());
  }
}

