//! A simpleton smartpointer library

extern crate core;

use std::boxed::Box;
use std::ops::Deref;
use std::option::Option;
use std::time::{Duration, Instant};

#[cfg(test)]
use std::thread;

/// A pointer timebomb
pub struct Tc<T : ?Sized> {
  bx : Box<T>,
  expiration : Instant
}

/// Construction of Tc given a subtype
impl<T> Tc<T> {
  pub fn new(value : T, ttl : Duration) -> Tc<T> {
    Tc {
      bx: Box::new(value),
      expiration: Instant::now() + ttl
    }
  }
}

/// Query a Tc for its contents.
impl<T : ?Sized> Tc<T> {
  fn examine(&self) -> Option<&T> {
    if Instant::now() < self.expiration {
      return Option::Some(&self.bx);
    }

    return Option::None
  }
}

/// Query a Tc for its contents, or panic.
impl<T : ?Sized> Deref for Tc<T> {
  type Target = T;

  fn deref(&self) -> &T {
    return self.examine().unwrap();
  }
}

/// Unit tests
#[test]
fn smoketest() {
    let tc = Tc::new(1337, Duration::from_millis(1));
    assert_eq!(*tc, 1337);

    thread::sleep(Duration::from_millis(2));

    assert_eq!(tc.examine(), Option::None);
}
