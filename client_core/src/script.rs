#![allow(deprecated)]
use {
  alloc::{ format, vec::Vec, },
  crate::{
    program_state::{ ProgramState, PersistentData, FrameData, },
    message::{ ClientMessage, ServerMessage, DebugMessage, },
  },
  bouquet_ribbon::message::{ Message, MessageSendee, },
  rhai::{
    AST, CustomType, Engine, ImmutableString, INT, Scope, TypeBuilder,
  },
};

#[derive(Clone, Debug)]
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

  fn push_debug_log(mut self, message: ImmutableString) -> Mailbox
  {
    self.mailbox.push(Message::Debug(DebugMessage::Log(message)));
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
      .with_fn("push_update", Self::push_update)
      .with_fn("push_debug_log", Self::push_debug_log);
  }
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
    //let result: Result<Mailbox, Box<EvalAltResult>> = match message {
    let result = match message {
      Message::Initialize => {
        engine.call_fn::<Mailbox>(scope, ast, "on_initialize", () )
      },
      Message::Terminate => {
        engine.call_fn::<Mailbox>(scope, ast, "on_terminate", () )
      },
      Message::Update(delta_t) => {
        engine.call_fn::<Mailbox>(scope, ast, "on_update", ( delta_t, ) )
      },
      _ => Ok(Mailbox::new()),
    }.unwrap_or_else(
      |err| {
        match *err {
          //EvalAltResult::ErrorFunctionNotFound(..) => Mailbox::new(),
          _ => Mailbox::new().push_debug_log(format!("{:?}", err).into()),
        }
      }
    );
    //.push_debug_log(format!("{:?}", message).into());
    result.mailbox
    //Mailbox::new().push_debug_log(format!("{:?}", result.mailbox).into()).mailbox
  }
}

