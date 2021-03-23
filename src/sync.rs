use std::{
    cell::RefCell,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

#[allow(dead_code)]
/// A type with a shared thread-safe part and a non-shared part
/// ## Example
/// ```rust
/// use osch::sync::{Sharer, AtomicF32};
/// use std::{sync::atomic::{Ordering},thread,};
///
/// let mut _sharer = Sharer::new(AtomicF32::new(3.1), 1);
///     let shared = _sharer.get_shared();
///     
///     thread::spawn(move || {
///         let mut owned = _sharer.owned.borrow_mut();
///         *owned += 1;
///         _sharer.shared.load(Ordering::Relaxed);
///     });
///     &shared.store(444., Ordering::Relaxed);
///     &shared;
/// ```
pub struct Sharer<S: Sync, O> {
    pub shared: Arc<S>,
    pub owned: RefCell<O>,
}

#[allow(dead_code)]
impl<S: Sync, O> Sharer<S, O> {
    pub fn new(shared: S, owned: O) -> Self {
        Sharer {
            shared: Arc::new(shared),
            owned: RefCell::new(owned),
        }
    }
    /// Returns an `Arc` pointer to the shared part
    pub fn get_shared(&self) -> Arc<S> {
        Arc::clone(&self.shared)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
/// Atomic float type, with only `load`, `store` and `swap` operations.
///
/// Uses [AtomicU32] internally. Operations need an [Ordering].
pub struct AtomicF32(AtomicU32);

impl AtomicF32 {
    pub fn new(x: f32) -> Self {
        AtomicF32(AtomicU32::new(x.to_bits()))
    }
    /// Atomically load the current value
    pub fn load(&self, order: Ordering) -> f32 {
        f32::from_bits(self.0.load(order))
    }
    /// Atomically store `val`
    pub fn store(&self, val: f32, order: Ordering) {
        self.0.store(val.to_bits(), order)
    }
    /// Atomically read the current value and store a new one
    pub fn swap(&self, val: f32, order: Ordering) -> f32 {
        f32::from_bits(self.0.swap(val.to_bits(), order))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomic_f32_load() {
        let a = AtomicF32::new(3.212);
        assert_eq!(a.load(Ordering::Relaxed), 3.212)
    }
    #[test]
    fn atomic_f32_store() {
        let a = AtomicF32::new(3.212);
        a.store(4.1, Ordering::Relaxed);
        assert_eq!(a.load(Ordering::Relaxed), 4.1)
    }
    #[test]
    fn atomic_f32_swap() {
        let a = AtomicF32::new(3.212);
        assert_eq!(a.swap(4.1, Ordering::Relaxed), 3.212);
        assert_eq!(a.load(Ordering::Relaxed), 4.1)
    }

    #[test]
    fn test_stuff() {
        use crate::sync::{AtomicF32, Sharer};
        use std::{sync::atomic::Ordering, thread};
        let mut _sharer = Sharer::new(AtomicF32::new(3.1), 1);
        let shared = _sharer.get_shared();

        thread::spawn(move || {
            let mut owned = _sharer.owned.borrow_mut();
            *owned += 1;
            _sharer.shared.load(Ordering::Relaxed);
        });
        &shared.store(444., Ordering::Relaxed);
        &shared;
    }
}
