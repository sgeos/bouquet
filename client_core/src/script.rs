#![allow(deprecated)]
use {
  alloc::{ format, sync::Arc, vec::Vec, },
  crate::{
    program_state::{ ProgramState, PersistentData, FrameData, },
    message::{ ClientMessage, ServerMessage, DebugMessage, },
  },
  bouquet_ribbon::message::{ Message, MessageSendee, },
  rhai::{
    AST, CustomType, Engine, EvalAltResult, ImmutableString, INT,
    packages::{ CorePackage, MoreStringPackage, Package, },
    ParseError, Scope, TypeBuilder,
  },
};

#[derive(Clone, Debug)]
pub struct Mailbox {
  mailbox: Vec<Arc::<Message::<ClientMessage, ServerMessage, DebugMessage>>>,
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
    self.mailbox.push(Arc::new(Message::Initialize));
    self
  }

  fn push_terminate(mut self) -> Mailbox
  {
    self.mailbox.push(Arc::new(Message::Terminate));
    self
  }

  fn push_update(mut self, delta_t: INT) -> Mailbox
  {
    self.mailbox.push(Arc::new(Message::Update(delta_t)));
    self
  }

  fn push_debug_log(mut self, message: ImmutableString) -> Mailbox
  {
    self.mailbox.push(Arc::new(Message::Debug(DebugMessage::Log(message))));
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
  pub fn new(script: &str) -> ScriptingEngine {
    let mut engine = Engine::new_raw();
    engine
      .build_type::<Mailbox>()
      .register_type_with_name::<ProgramState>("ProgramState")
      .register_type_with_name::<PersistentData>("PersistentData")
      .register_type_with_name::<FrameData>("FrameData")
      .set_max_expr_depths(32, 32);
    CorePackage::new().register_into_engine(&mut engine);
    MoreStringPackage::new().register_into_engine(&mut engine);
    let scope = Scope::new();
    let ast = engine.compile(script).unwrap_or(
      engine.compile("").unwrap()
    );
    ScriptingEngine {
      engine: engine,
      scope: scope,
      ast: ast,
    }
  }

  pub fn compile(&mut self, script: &str) -> Result<AST, ParseError>
  {
    self.engine.compile(script)
  }

  pub fn set_ast(&mut self, ast: AST)
  {
    self.ast = ast;
  }

  pub fn compile_and_set_ast(&mut self, script: &str) -> Result<AST, ParseError>
  {
    let result = self.engine.compile(script);
    if let Ok(ast) = result.clone() {
      self.ast = ast;
    }
    result
  }

  pub fn validate_script(&mut self, script: &str)
    -> Arc::<Message<ClientMessage, ServerMessage, DebugMessage>>
  {
    let message = match self.engine.compile(script) {
      Ok(_) => format!("Script OK.").into(),
      Err(err) => format!("{}", err).into(),
    };
    Arc::new(Message::Debug(DebugMessage::Log(message)))
  }
}

impl
  MessageSendee::<ProgramState, ClientMessage, ServerMessage, DebugMessage>
  for ScriptingEngine
{
  fn send(
    &mut self,
    message: Arc::<Message::<ClientMessage, ServerMessage, DebugMessage>>,
    _ps: &mut ProgramState,
  ) -> Vec<Arc::<Message::<ClientMessage, ServerMessage, DebugMessage>>>
  {
    let engine = &self.engine;
    let scope = &mut self.scope;
    let ast = &self.ast;
    let result = match *message {
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
          EvalAltResult::ErrorFunctionNotFound(..) => Mailbox::new(),
          _ => Mailbox::new().push_debug_log(format!("Simulation Scripting Error: {:?}", err).into()),
        }
      }
    );
    result.mailbox
  }
}

