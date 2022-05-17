use std::cmp::Ordering;

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
  let l = data.len();
  let n = (0.002 * l as f64).floor() - shift;
  let tau = (n - 1.) / 2.;

  data
    .windows(n as usize)
    .map(|points| points.iter().map(|p| p.x).sum::<f64>() / n)
    .zip(&data)
    .map(|(new_x, old_point)| Point {
      x: new_x,
      y: old_point.y,
    })
    .skip(tau as usize)
    .take(l - n as usize)
    .collect::<Vec<_>>()
}

fn enveloping(data: &Vec<Point>) -> Vec<f64> {
  let first_half = data
    .iter()
    .map_while(|p| if p.y < 1. { Some(p.y) } else { None })
    .scan(0., |max, y| {
      if (*max).partial_cmp(&y) == Some(Ordering::Less) {
        *max = y;
      }
      Some(*max)
    });

  let second_half = data
    .iter()
    .rev()
    .map_while(|p| if p.y < 1. { Some(p.y) } else { None })
    .scan(0., |max, y| {
      if (*max).partial_cmp(&y) == Some(Ordering::Less) {
        *max = y;
      }
      Some(*max)
    });

  first_half.chain(second_half).collect::<Vec<_>>()
}

fn pulsewidth(enveloping: &Vec<f64>) -> PulseWidth {
  PulseWidth {
    b: 1.2,
    L: 3200,
    R: 5400,
  }
}

pub fn calculate_auto_correlation_function(data: Vec<Point>) -> Output {
  let normed = norm(data);
  let smoothed = smooth(normed);
  let enveloping = enveloping(&smoothed);
  let pw = pulsewidth(&enveloping);
  Output {
    data: smoothed,
    enveloping,
    pw,
  }
}
