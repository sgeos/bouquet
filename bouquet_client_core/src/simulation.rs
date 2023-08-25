use {
    crate::program_state::ProgramState,
    alloc::{sync::Arc, vec::Vec},
    bouquet_core::{message::Message, message_bus::MessageSendee},
};

#[derive(Default)]
pub struct Simulation {}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {}
    }
}

impl MessageSendee<ProgramState, Message> for Simulation {
    fn send(&mut self, message: Arc<Message>, ps: &mut ProgramState) -> Vec<Arc<Message>> {
        let result = Vec::new();
        match *message {
            Message::Initialize => ps.persistent_data.done = false,
            Message::Terminate => ps.persistent_data.done = true,
            Message::Update(_) => {
                ps.next_frame_data.frame = ps.last_frame_data.frame + 1;
            }
            _ => (),
        }
        result
    }
}
