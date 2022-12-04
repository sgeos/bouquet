use alloc::{ fmt::Debug, string::String, vec::Vec, };
use bouquet_ribbon::message::{ Message, MessageSendee, };
use client_core::{ program_state::ProgramState, };
use core::{ convert::TryInto, };

pub struct Terminal { }

impl Terminal {
  pub fn new() -> Terminal {
    Terminal { }
  }
}

impl<C, S, D>
  MessageSendee::<ProgramState, C, S, D>
  for Terminal
  where C: Clone + Debug, S: Clone + Debug, D: Clone + Debug
{
  fn send(
    &mut self,
    message: Message::<C, S, D>,
    program_state: &mut ProgramState,
  ) -> Vec<Message::<C, S, D>>
  {
    let mut result = Vec::new();
    match message {
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
          "exit" | "quit" => {
            result.push(Message::Terminate);
          },
          "done" => {
            log(format!("Done: {}", program_state.persistent_data.done));
          },
          "frame" => {
            log(format!("Frame: {}", program_state.last_frame_data.frame));
          },
          "initialize" => result.push(Message::Initialize),
          "terminate" => result.push(Message::Terminate),
          "update" => {
            let duration: usize = args.next()
              .unwrap_or("0").parse().unwrap_or(0);
            result.push(Message::Update(duration))
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
      Message::Client(s) => log(format!("Client: {:?}", s)),
      Message::Server(s) => log(format!("Server: {:?}", s)),
      Message::Debug(s) => log(format!("Debug: {:?}", s)),
      // _ => (),
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

