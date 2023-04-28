use {
  alloc::{
    boxed::Box, fmt::Debug, string::String, sync::Arc, vec::Vec,
  },
  hashbrown::HashMap,
  rhai::{ INT, },
};

#[derive(Debug, Clone)]
pub enum Message<C, S, D>
  where C: Debug, S: Debug, D: Debug,
{
  Initialize,
  Terminate,
  Update(INT),
  Client(C),
  Server(S),
  Debug(D),
}

pub trait MessageSendee<P, C, S, D>
  where P: Debug, C: Debug, S: Debug, D: Debug,
{
  fn send(
    &mut self,
    message: Arc::<Message::<C, S, D>>,
    program_state: &mut P,
  ) -> Vec<Arc::<Message::<C, S, D>>>;
}

pub struct MessageBus<P, C, S, D>
  where P: Debug, C: Debug, S: Debug, D: Debug,
{
  pub done: bool,
  inbox: Vec::<Arc::<Message::<C, S, D>>>,
  outbox: Vec::<Arc::<Message::<C, S, D>>>,
  systems: HashMap::<String, Box::<dyn MessageSendee<P, C, S, D>>>,
}

impl<P, C, S, D> MessageBus<P, C, S, D>
  where P: Debug, C: Debug, S: Debug, D: Debug,
{
  pub fn new() -> MessageBus<P, C, S, D> {
    MessageBus {
      done: false,
      inbox: Vec::new(),
      outbox: Vec::new(),
      systems: HashMap::new(),
    }
  }

  pub fn register<K: Into<String>>(
    &mut self, key: K, system: Box::<dyn MessageSendee<P, C, S, D>>
  ) {
    self.systems.insert(key.into(), system);
  }

  pub fn unregister<K: Into<String>>(&mut self, key: K) {
    self.systems.remove(&key.into());
  }

  pub fn get<K: Into<String>>(
    &mut self, key: K
  ) -> Option<&Box::<dyn MessageSendee<P, C, S, D>>>
 {
    self.systems.get(&key.into())
  }

  pub fn push(&mut self, message: Arc::<Message::<C, S, D>>) {
    self.outbox.push(message);
  }

  pub fn flush(&mut self, program_state: &mut P) {
    for message in &self.outbox {
      self.done = match **message {
        Message::Initialize => false,
        Message::Terminate => true,
        _ => self.done,
      };
      for system in self.systems.values_mut() {
        self.done = match **message {
          Message::Initialize => false,
          Message::Terminate => true,
          _ => self.done,
        };
        let response = system.send(message.clone(), program_state);
        for message in response {
          self.inbox.push(message.clone());
        }
      }
    }
    self.outbox.clear();
    core::mem::swap(&mut self.inbox, &mut self.outbox);
  }
}

impl<P, C, S, D> MessageSendee<P, C, S, D> for MessageBus<P, C, S, D>
  where P: Debug, C: Debug, S: Debug, D: Debug,
{
  fn send(
    &mut self,
    message: Arc::<Message::<C, S, D>>,
    program_state: &mut P,
  ) -> Vec<Arc::<Message::<C, S, D>>> {
    let result = Vec::new();
    self.push(message);
    self.flush(program_state);
    result
  }
}

