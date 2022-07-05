#![no_std]

extern crate alloc;
extern crate libc;

trait MessageSender<T> {
  fn send(&self, message: T);
}

enum Message<'a> {
  Initialize,
  Terminate,
  Update(core::time::Duration),
  Register(&'a dyn MessageSender<Message<'a>>),
}

#[no_mangle]
pub extern "C" fn run() {
  const MESSAGE: &'static str = "Hello, Bouquet!\n\0";
  unsafe {
    libc::printf(MESSAGE.as_ptr() as *const _);
  }
  let mut vec = alloc::vec::Vec::<Message>::new();
  vec.push(Message::Initialize);
  vec.push(Message::Terminate);
  vec.push(Message::Update(core::time::Duration::new(1, 0)));
  //vec.push(Message::Register(nil));
  for (i, message) in vec.iter().enumerate() {
    const FORMAT: &'static str = "Message %d : %s\n\0";
    let string: &'static str = match message {
      Message::Initialize => "Initialize\0",
      Message::Terminate => "Terminate\0",
      Message::Update(_) => "Update\0",
      Message::Register(_) => "Register\0",
    };
    unsafe {
      libc::printf(FORMAT.as_ptr() as *const _, i, string.as_ptr() as *const _);
    }
  }
}

