use core::f32;
use std::{
    intrinsics::transmute,
    sync::atomic::{AtomicU32, Ordering},
};

#[derive(Debug)]
pub struct AtomicF32(AtomicU32);

impl AtomicF32 {
    pub fn new(x: f32) -> Self {
        AtomicF32(AtomicU32::new(x.to_bits()))
    }
    pub fn load(&self, order: Ordering) -> f32 {
        f32::from_bits(self.0.load(order))
    }
    pub fn store(&self, val: f32, order: Ordering) {
        self.0.store(val.to_bits(), order)
    }
    pub fn swap(&self, val: f32, order: Ordering) -> f32 {
        f32::from_bits(self.0.swap(val.to_bits(), order))
    }
}

#[test]
fn atomic_f32_load() {
    let a = AtomicF32::new(3.212);
    assert_eq!(a.load(Ordering::Relaxed), 3.212)
}
