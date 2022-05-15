use std::iter::Map;

use crate::common::{Output, Point, PulseWidth};

fn norm(data: Vec<Point>) -> Vec<Point> {
  let max = data
    .iter()
    .map(|p| p.y)
    .max_by(|a, b| a.partial_cmp(b).unwrap())
    .unwrap();
  data
    .iter()
    .map(|p| Point {
      x: p.x * 266.3 / 262143. - 132.3,
      y: p.y / max,
    })
    .collect()
}

fn smooth(data: Vec<Point>) -> Vec<Point> {
  let shift = match data.len() % 2 {
    0 => 1.,
    _ => 0.,
  };
  let n = (0.002 * data.len() as f64).floor() - shift;

  data
    .iter()
    .map(|p| p.x)
    .collect()
    .windows(n)
    .map(|arr| arr.iter().sum())
}

pub fn calculate_auto_correlation_function(data: Vec<Point>) -> Output {
  Output {
    data: vec![Point { x: 1.1, y: 1.1 }],
    enveloping: vec![1.1],
    pw: PulseWidth { b: 1.1, L: 1, R: 2 },
  }
}
