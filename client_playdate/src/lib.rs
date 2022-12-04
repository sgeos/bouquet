#![no_std]
#![feature(rustc_private)]

#[macro_use]
extern crate alloc;
extern crate libc;

use alloc::{ boxed::Box, };
use bouquet_ribbon::message::{ Message, MessageBus, MessageSendee, };
use client_core::{ program_state::ProgramState, simulation::Simulation };
use client_core::message::{ ClientMessage, DebugMessage, ServerMessage, };
use core::{ convert::TryInto, };

#[no_mangle]
pub extern "C" fn run() {
  let mut ps = ProgramState::new();
  let mut mb = MessageBus::
    <ProgramState, ClientMessage, ServerMessage, DebugMessage>::new();
  let simulation = Box::new(Simulation::new());
  let mut time = unsafe { libc::time(0 as *mut i64) };

  mb.register("simulation", simulation);
  mb.send(
    Message::Debug(DebugMessage::Log(format!("Hello, Bouquet!"))),
    &mut ps
  );
  mb.send(Message::Initialize, &mut ps);
  while !ps.persistent_data.done {
    let old_time = time;
    time = unsafe { libc::time(0 as *mut i64) };
    let delta_t: usize = (time - old_time).try_into().unwrap_or(0);
    mb.send(Message::Update(delta_t), &mut ps);
    mb.send(Message::Terminate, &mut ps);
    ps.next_frame();
  }
  mb.unregister("simulation");
}

