#![no_std]

extern crate alloc;

use alloc::{ boxed::Box, fmt::Debug, string::String, vec::Vec, };
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub enum Message<C, S, D>
  where C: Clone + Debug, S: Clone + Debug, D: Clone + Debug
{
  Initialize,
  Terminate,
  Update(usize),
  Client(C),
  Server(S),
  Debug(D),
}

pub trait MessageSendee<C, S, D>
  where C: Clone + Debug, S: Clone + Debug, D: Clone + Debug
{
  fn send(
    &mut self, message: Message::<C, S, D>
  ) -> Vec<Message::<C, S, D>>;
}

pub struct MessageBus<C, S, D>
  where C: Clone + Debug, S: Clone + Debug, D: Clone + Debug
{
  pub done: bool,
  inbox: Vec::<Message::<C, S, D>>,
  outbox: Vec::<Message::<C, S, D>>,
  systems: HashMap::<String, Box<dyn MessageSendee<C, S, D>>>,
}

impl<C, S, D> MessageBus<C, S, D>
  where C: Clone + Debug, S: Clone + Debug, D: Clone + Debug
{
  pub fn new() -> MessageBus<C, S, D> {
    MessageBus {
      done: false,
      inbox: Vec::new(),
      outbox: Vec::new(),
      systems: HashMap::new(),
    }
  }

  pub fn register<K: Into<String>>(
    &mut self, key: K, system: Box<dyn MessageSendee<C, S, D>>
  ) {
    self.systems.insert(key.into(), system);
  }

  pub fn unregister<K: Into<String>>(&mut self, key: K) {
    self.systems.remove(&key.into());
  }

  pub fn push(&mut self, message: Message::<C, S, D>) {
    self.outbox.push(message);
  }

  pub fn flush(&mut self) {
    for message in &self.outbox {
      self.done = match message {
        Message::Initialize => false,
        Message::Terminate => true,
        _ => self.done,
      };
      for system in self.systems.values_mut() {
        self.done = match message {
          Message::Initialize => false,
          Message::Terminate => true,
          _ => self.done,
        };
        let response = system.send(message.clone());
        for message in response {
          self.inbox.push(message);
        }
      }
    }
    self.outbox.clear();
    core::mem::swap(&mut self.inbox, &mut self.outbox);
  }
}

impl<C, S, D> MessageSendee<C, S, D> for MessageBus<C, S, D>
  where C: Clone + Debug, S: Clone + Debug, D: Clone + Debug
{
  fn send(
    &mut self, message: Message::<C, S, D>
  ) -> Vec<Message::<C, S, D>> {
    let result = Vec::new();
    self.push(message);
    self.flush();
    result
  }
}

