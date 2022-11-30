use alloc::{ string::String, vec::Vec, };
use core::{ convert::TryInto, };

use crate::message::{ Message, };
use crate::message_bus::{ MessageSendee, };

pub struct Terminal {
  pub done: bool,
}

impl Terminal {
  pub fn new() -> Terminal {
    Terminal {
      done: false,
    }
  }
}

impl MessageSendee::<Message> for Terminal {
  fn send(&mut self, message: Message) -> (bool, Vec<Message>) {
    let mut result = Vec::new();
    match (message, self.done) {
      (Message::Initialize, _) => {
        self.done = false;
        log("Initializing program.");
      },
      (Message::Terminate, _) => {
        self.done = true;
        log("Terminating program.");
      },
      (Message::Update(duration), false) => {
        prompt(format!("{:>6}> ", duration));

        let input = readline();
        let mut args = input.trim().split_whitespace().peekable();
        let command = args.next().unwrap_or("");
        match command.to_lowercase().as_str() {
          "exit" | "quit" => {
            self.done = true;
            result.push(Message::Terminate);
          },
          "done" => log(format!("Done: {}", self.done)),
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
      (Message::Log(s), _) => log(s),
      _ => (),
    }
    (self.done, result)
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

