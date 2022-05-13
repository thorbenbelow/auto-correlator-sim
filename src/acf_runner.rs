use crate::{
  auto_correlator,
  common::{Receiver, Sender, SenderStatus},
};
use std::{sync::mpsc, thread};

pub fn run_acf(mut sender: Box<dyn Sender + Send>, receiver: Box<dyn Receiver + Send>) {
  let (request_sender, request_receiver) = mpsc::channel();
  let (raw_data_sender, raw_data_receiver) = mpsc::channel();
  let (parsed_data_sender, parsed_data_receiver) = mpsc::channel();

  let senderThread = thread::spawn(move || loop {
    request_receiver.recv();

    match sender.next() {
      Ok(data_set) => {
        raw_data_sender.send(data_set);
      }
      Err(status) => match status {
        SenderStatus::Waiting => loop {
          if let Ok(data_set) = sender.next() {
            raw_data_sender.send(data_set);
            break;
          }
        },
        SenderStatus::Empty => break,
      },
    }
  });

  let acf = thread::spawn(move || loop {
    request_sender.send(true);
    let i = raw_data_receiver.recv();
    println!("ACF::Received");
    if let Ok(v) = i {
      let parsed = auto_correlator::calculate_auto_correlation_function(v.data);
      println!("ACF::Send");
      parsed_data_sender.send((v.id, parsed));
    } else {
      println!("ACF::Break");
      break;
    }
  });

  let receiverThread = thread::spawn(move || loop {
    let i = parsed_data_receiver.recv();
    if let Ok((id, output)) = i {
      receiver.receive(id, output);
    } else {
      break;
    }
  });

  senderThread.join().unwrap();
  receiverThread.join().unwrap();
  acf.join().unwrap();
}
