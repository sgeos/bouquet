//extern crate alloc;
//extern crate libc;

use alloc::{ boxed::Box, string::String, vec::Vec, };
use hashbrown::HashMap;

pub trait MessageSendee<T: Copy + Clone> {
  fn send(&mut self, message: T) -> (bool, Vec<T>);
}

pub struct MessageBus<T: Copy + Clone> {
  pub done: bool,
  inbox: Vec::<T>,
  outbox: Vec::<T>,
  systems: HashMap::<String, Box<dyn MessageSendee<T>>>,
}

impl<T: Copy + Clone> MessageBus<T> {
  pub fn new() -> MessageBus<T> {
    MessageBus {
      done: false,
      inbox: Vec::new(),
      outbox: Vec::new(),
      systems: HashMap::new(),
    }
  }

  pub fn register<S: Into<String>>(
    &mut self, name: S, system: Box<dyn MessageSendee<T>>
  ) {
    self.systems.insert(name.into(), system);
  }

  pub fn unregister<S: Into<String>>(&mut self, name: S) {
    self.systems.remove(&name.into());
  }

  pub fn push(&mut self, message: T) {
    self.outbox.push(message);
  }

  pub fn flush(&mut self) {
    for message in &self.outbox {
      for system in self.systems.values_mut() {
        let (done, response) = system.send(*message);
        self.done |= done;
        for message in response {
          self.inbox.push(message);
        }
      }
    }
    self.outbox.clear();
    core::mem::swap(&mut self.inbox, &mut self.outbox);
  }
}

impl<T: Copy + Clone> MessageSendee<T> for MessageBus<T> {
  fn send(&mut self, message: T) -> (bool, Vec<T>) {
    let result = Vec::new();
    self.push(message);
    self.flush();
    (self.done, result)
  }
}

