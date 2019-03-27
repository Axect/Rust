extern crate threadpool;
extern crate peroxide;
use threadpool::ThreadPool;
use peroxide::*;
use std::sync::mpsc::channel;

pub fn main() {
  let pool = ThreadPool::new(4);
  let (tx, rx) = channel();

  let mut s = 0f64;
  for _i in 0 .. 10 {
    let tx = tx.clone();
    pool.execute(move || {
      let result = rand(50, 50).det();
      tx.send(result).expect("Can't send data!");
    });
  }

  for _ in 0 .. 10 {
    let r = rx.recv().expect("Can't receive");
    s += r;
  }
  s.print();
}
