#![no_std]
#![feature(rustc_private)]

#[macro_use]
extern crate alloc;
extern crate libc;

use alloc::{ boxed::Box, string::String, };
use core::{ convert::TryInto, };

mod message;
mod simulation;
mod terminal;

use message::{ Message, MessageBus, MessageSendee, };
use simulation::{ Simulation, };
use terminal::{ Terminal, };

#[no_mangle]
pub extern "C" fn run() {
  let mut mb = MessageBus::<String, String, String>::new();
  let simulation = Box::new(Simulation::new());
  let terminal = Box::new(Terminal::new());
  let mut time = unsafe { libc::time(0 as *mut i64) };

  mb.register("simulation", simulation);
  mb.register("terminal", terminal);
  mb.send(Message::Client(format!("Hello, Bouquet!")));
  mb.send(Message::Server(format!("Hello, Bouquet!")));
  mb.send(Message::Debug(format!("Hello, Bouquet!")));
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

