//! CLI game

extern crate fizzy;

/// CLI entrypoint
fn main() {
  for n in 1..101 {
    println!("{:?}", fizzy::fizzbuzz(n));
  }
}
