use {
    alloc::{boxed::Box, fmt::Debug, string::String, sync::Arc, vec::Vec},
    hashbrown::HashMap,
};

pub trait SystemMessage {
    fn is_initialize(&self) -> bool;
    fn is_terminate(&self) -> bool;
}

pub trait MessageSendee<P, M>
where
    P: Debug,
    M: SystemMessage + Clone + Debug,
{
    fn send(&mut self, message: Arc<M>, program_state: &mut P) -> Vec<Arc<M>>;
}

#[derive(Default)]
pub struct MessageBus<P, M>
where
    P: Debug,
    M: SystemMessage + Clone + Debug,
{
    pub done: bool,
    inbox: Vec<Arc<M>>,
    outbox: Vec<Arc<M>>,
    systems: HashMap<String, Box<dyn MessageSendee<P, M>>>,
}

impl<P, M> MessageBus<P, M>
where
    P: Debug,
    M: SystemMessage + Clone + Debug,
{
    pub fn new() -> MessageBus<P, M> {
        MessageBus {
            done: false,
            inbox: Vec::new(),
            outbox: Vec::new(),
            systems: HashMap::new(),
        }
    }

    pub fn register<K: Into<String>>(&mut self, key: K, system: Box<dyn MessageSendee<P, M>>) {
        self.systems.insert(key.into(), system);
    }

    pub fn unregister<K: Into<String>>(&mut self, key: K) {
        self.systems.remove(&key.into());
    }

    pub fn get<K: Into<String>>(&mut self, key: K) -> Option<&dyn MessageSendee<P, M>> {
        self.systems.get(&key.into()).map(|system| &**system)
    }

    pub fn push(&mut self, message: Arc<M>) {
        self.outbox.push(message);
    }

    pub fn flush(&mut self, program_state: &mut P) {
        for message in &self.outbox {
            if message.is_initialize() {
                self.done = false;
            } else if message.is_terminate() {
                self.done = true;
            }
            for system in self.systems.values_mut() {
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

impl<P, M> MessageSendee<P, M> for MessageBus<P, M>
where
    P: Debug,
    M: SystemMessage + Clone + Debug,
{
    fn send(&mut self, message: Arc<M>, program_state: &mut P) -> Vec<Arc<M>> {
        let result = Vec::new();
        self.push(message);
        self.flush(program_state);
        result
    }
}
