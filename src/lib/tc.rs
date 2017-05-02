extern crate core;

use std::mem;
use core::ops::Deref;
use std::thread;
use std::time::Duration;

pub struct Tc<T : ?Sized> {
    value : T
}

pub fn new<T : Copy + Send + Sync + 'static>(value : T) -> Tc<T> {
    let tc : Tc<T> = Tc { value: value };

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        mem::drop(value);
    });

    return tc;
}

unsafe impl<T : Send> Send for Tc<T> {}
unsafe impl<T : Sync> Sync for Tc<T> {}

impl<T : ?Sized> Deref for Tc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

#[test]
fn smoketest() {
    let tc : Tc<u16> = new(1337);
    assert_eq!(*tc, 1337);

    thread::sleep(Duration::from_secs(4));
    assert_ne!(*tc, 1337);
}
