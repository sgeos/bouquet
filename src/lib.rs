#![no_std]
#![feature(rustc_private)]

#[macro_use]
extern crate alloc;
extern crate libc;

use alloc::{ boxed::Box, string::String, vec::Vec, };
use core::{ convert::TryInto, };

mod message;

use message::{ MessageBus, MessageSendee, };

#[derive(Debug, Copy, Clone)]
enum Message {
  Initialize,
  Terminate,
  Update(usize),
}

struct Simulation {
  done: bool,
  frame: usize,
}

impl MessageSendee::<Message> for Simulation {
  fn send(&mut self, message: Message) -> (bool, Vec<Message>) {
    let result = Vec::new();
    match message {
      Message::Initialize => self.done = false,
      Message::Terminate => self.done = true,
      Message::Update(_) => self.frame += 1,
      //_ => (),
    }
    (self.done, result)
  }
}

struct Terminal {
  done: bool,
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

#[no_mangle]
pub extern "C" fn run() {
  log("Hello, Bouquet!");

  let simulation = Box::new(Simulation {
    done: false,
    frame: 0,
  });
  let terminal = Box::new(Terminal {
    done: false,
  });
  let mut mb = MessageBus::<Message>::new();

  mb.register("simulation", simulation);
  mb.register("terminal", terminal);
  let mut time = unsafe { libc::time(0 as *mut i64) };
  mb.send(Message::Initialize);
  while !mb.done {
    let old_time = time;
    time = unsafe { libc::time(0 as *mut i64) };
    let delta_t: usize = (time - old_time).try_into().unwrap_or(0);
    mb.send(Message::Update(delta_t));
  }
  mb.unregister("simulation");
  mb.unregister("terminal");
}

