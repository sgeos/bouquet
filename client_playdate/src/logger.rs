use {
    alloc::{sync::Arc, vec::Vec},
    bouquet_client_core::program_state::ProgramState,
    bouquet_core::{message::Message, message_bus::MessageSendee},
    crankstart::system::System,
};

pub struct Logger {}

impl Logger {
    pub fn new() -> Logger {
        Logger {}
    }
}

impl MessageSendee<ProgramState, Message> for Logger {
    fn send(
        &mut self,
        message: Arc<Message>,
        _program_state: &mut ProgramState,
    ) -> Vec<Arc<Message>> {
        let result = Vec::new();
        match &*message {
            Message::Initialize => log("Initializing program."),
            Message::Terminate => log("Terminating program."),
            //Message::Update(duration) => log(format!("Update: {}", duration)),
            Message::DebugLog(s) => log(s),
            //_ => log(format!("{:?}", message)),
            _ => (),
        }
        result
    }
}

fn log<S: AsRef<str>>(message: S) {
    System::log_to_console(message.as_ref());
}
