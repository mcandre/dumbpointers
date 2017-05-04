extern crate core;

use std::boxed::Box;
use std::ops::Deref;
use std::option::Option;
use std::time::{Duration, Instant};

#[cfg(test)]
use std::thread;

pub struct Tc<T : ?Sized> {
  bx : Box<T>,
  expiration : Instant
}

pub fn new<T>(value : T) -> Tc<T> {
  return Tc {
    bx: Box::new(value),
    expiration: Instant::now() + Duration::from_secs(3)
  }
}

impl<T : ?Sized> Tc<T> {
  fn examine(&self) -> Option<&T> {
    if Instant::now() < self.expiration {
      return Option::Some(&self.bx);
    }

    return Option::None
  }
}

impl<T : ?Sized> Deref for Tc<T> {
  type Target = T;

  fn deref(&self) -> &T {
    return self.examine().unwrap();
  }
}

#[test]
fn smoketest() {
    let tc : Tc<u16> = new(1337);
    assert_eq!(*tc, 1337);

    thread::sleep(Duration::from_secs(4));

    assert_eq!(tc.examine(), Option::None);
}
