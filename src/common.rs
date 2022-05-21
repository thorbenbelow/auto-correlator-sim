#[derive(Debug)]
pub struct DataSet {
  pub id: usize,
  pub data: Vec<Point>,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
  pub x: f64,
  pub y: f64,
}

pub trait Receiver {
  fn receive(&self, id: usize, data: Output) -> ();
}

#[derive(PartialEq)]
pub enum SenderStatus {
  Empty,
  Waiting,
  Next,
}
pub trait Sender {
  fn next(&mut self) -> Result<DataSet, SenderStatus>;
}

#[derive(Debug)]
pub struct PulseWidth {
  pub b: f64,
  pub L: usize,
  pub R: usize,
}

pub struct Output {
  pub data: Vec<Point>,
  pub enveloping: Vec<f64>,
  pub pw: PulseWidth,
}
