#![allow(deprecated)]
use {
  alloc::{ vec::Vec, },
  crate::{
    program_state::{ ProgramState, PersistentData, FrameData, },
    message::{ ClientMessage, ServerMessage, DebugMessage, },
  },
  bouquet_ribbon::message::{ Message, MessageSendee, },
  rhai::{ AST, CustomType, Engine, INT, Scope, TypeBuilder, },
};

#[derive(Clone)]
pub struct Mailbox {
  mailbox: Vec<Message::<ClientMessage, ServerMessage, DebugMessage>>,
}

impl Mailbox {
  fn new() -> Self
  {
    Mailbox {
      mailbox: Vec::new(),
    }
  }

  fn push_initialize(mut self) -> Mailbox
  {
    self.mailbox.push(Message::Initialize);
    self
  }

  fn push_terminate(mut self) -> Mailbox
  {
    self.mailbox.push(Message::Terminate);
    self
  }

  fn push_update(mut self, delta_t: INT) -> Mailbox
  {
    self.mailbox.push(Message::Update(delta_t));
    self
  }
}

impl CustomType for Mailbox {
  fn build(mut builder: TypeBuilder<Self>) {
    builder
      .with_name("Mailbox")
      .with_fn("new_mailbox", Self::new)
      .with_fn("push_initialize", Self::push_initialize)
      .with_fn("push_terminate", Self::push_terminate)
      .with_fn("push_update", Self::push_update);
  }
}

fn on_update_default(_delta_t: i64) -> Mailbox
{
  Mailbox::new()
}

fn on_event_default() -> Mailbox
{
  Mailbox::new()
}

pub struct ScriptingEngine {
  engine: Engine,
  scope: Scope<'static>,
  ast: AST,
}

impl ScriptingEngine {
  pub fn new() -> ScriptingEngine {
    let mut engine = Engine::new_raw();
    engine
      .build_type::<Mailbox>()
      .register_fn("on_default", on_event_default)
      .register_fn("on_initialize", on_event_default)
      .register_fn("on_terminate", on_event_default)
      .register_fn("on_update", on_update_default)
      .register_type_with_name::<ProgramState>("ProgramState")
      .register_type_with_name::<PersistentData>("PersistentData")
      .register_type_with_name::<FrameData>("FrameData");
    let scope = Scope::new();
    let script = include_str!("../rhaiscript/simulation.rhai");
    let ast = engine.compile(script).unwrap();
    ScriptingEngine {
      engine: engine,
      scope: scope,
      ast: ast,
    }
  }
}

impl
  MessageSendee::<ProgramState, ClientMessage, ServerMessage, DebugMessage>
  for ScriptingEngine
{
  fn send(
    &mut self,
    message: Message::<ClientMessage, ServerMessage, DebugMessage>,
    _ps: &mut ProgramState,
  ) -> Vec<Message::<ClientMessage, ServerMessage, DebugMessage>>
  {
    let engine = &self.engine;
    let scope = &mut self.scope;
    let ast = &self.ast;
    let result = match message {
      Message::Initialize => {
        engine.call_fn::<>(scope, ast, "on_initialize", () )
      },
      Message::Terminate => {
        engine.call_fn::<>(scope, ast, "on_terminate", () )
      },
      Message::Update(delta_t) => {
        engine.call_fn::<>(scope, ast, "on_update", ( delta_t, ) )
      },
      _ => {
        engine.call_fn::<>(scope, ast, "on_default", () )
      },
    };
    return result.unwrap_or(Mailbox::new()).mailbox;
  }
}

