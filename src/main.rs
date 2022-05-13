use io::{FileReceiver, FileSender};

mod acf_runner;
mod auto_correlator;
mod common;
mod io;

fn main() {
  let sender = Box::new(FileSender::new());
  let receiver = Box::new(FileReceiver {});
  acf_runner::run_acf(sender, receiver)
}
