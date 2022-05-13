use std::{
  fs::{read_to_string, write},
  str::FromStr,
  sync::WaitTimeoutResult,
};

use regex::Regex;

use crate::common::{DataSet, Output, Point, PulseWidth, Receiver, Sender, SenderStatus};

pub struct FileReceiver {}
impl Receiver for FileReceiver {
  fn receive(&self, id: usize, data: Output) -> () {
    write(
      format!("./data/out{}.txt", id),
      format!(
        "# FWHM: {}, {}, {}\n{}",
        data.pw.b,
        data.pw.L,
        data.pw.R,
        data
          .data
          .iter()
          .zip(data.enveloping.iter())
          .map(|(point, env)| format!("{}\t{}\t{}\n", point.x, point.y, env))
          .collect::<String>()
      ),
    )
    .unwrap();
  }
}

pub struct FileSender {
  i: usize,
}
impl FileSender {
  pub fn new() -> FileSender {
    FileSender { i: 0 }
  }
}

impl Sender for FileSender {
  fn next(&mut self) -> Result<DataSet, crate::common::SenderStatus> {
    if self.i < 10 {
      let content = read_to_string(format!("./data/{}.txt", self.i)).unwrap();
      let re = Regex::new(r"^(\d+)\t(\d+)$").unwrap();
      let data = content
        .lines()
        .filter_map(|line| {
          if let Some(matches) = re.captures(line) {
            if let [Some(y), Some(x)] = [matches.get(1), matches.get(2)] {
              Some(Point {
                x: x.as_str().parse::<f64>().unwrap(),
                y: y.as_str().parse::<f64>().unwrap(),
              })
            } else {
              None
            }
          } else {
            None
          }
        })
        .collect::<Vec<_>>();
      println!("Send file {}", self.i);
      self.i += 1;
      Ok(DataSet {
        id: self.i - 1,
        data,
      })
    } else {
      Err(SenderStatus::Empty)
    }
  }
}
