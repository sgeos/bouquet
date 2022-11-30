#[derive(Debug, Copy, Clone)]
pub enum Message {
  Initialize,
  Terminate,
  Update(usize),
}

