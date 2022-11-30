#![no_std]
#![feature(rustc_private)]

#[macro_use]
extern crate alloc;
extern crate libc;

use alloc::{ boxed::Box, };
use core::{ convert::TryInto, };

mod message;
mod message_bus;
mod simulation;
mod terminal;

use message::{ Message, };
use message_bus::{ MessageBus, MessageSendee, };
use simulation::{ Simulation, };
use terminal::{ Terminal, };

#[no_mangle]
pub extern "C" fn run() {
  let simulation = Box::new(Simulation::new());
  let terminal = Box::new(Terminal::new());
  let mut mb = MessageBus::<Message>::new();
  let mut time = unsafe { libc::time(0 as *mut i64) };

  mb.register("simulation", simulation);
  mb.register("terminal", terminal);
  mb.send(Message::log("Hello, Bouquet!"));
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

