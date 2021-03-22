use std::{sync::{Arc, atomic::Ordering}, thread};

use crate::{osc::{Dsp, Osc}, utils::AtomicF32};

#[allow(dead_code)]
/// A type with a shared thread-safe part and a non-shared part
/// ## Example
/// ```rust
/// let mut _sharer = Sharer::new(AtomicF32::new(3.1), 1);
///     let shared = _sharer.get_shared();
///     thread::spawn(move || {
///         _sharer.owned += 1;
///         _sharer.shared.load(Ordering::Relaxed);
///     });
///     &shared.store(444., Ordering::Relaxed);
///     &shared;
/// ```
struct Sharer<S: Sync, O> {
    shared: Arc<S>,
    owned: O,
}

#[allow(dead_code)]
impl<S: Sync, O> Sharer<S, O> {
    fn new(shared: S, owned: O) -> Self {
        Sharer {
            shared: Arc::new(shared),
            owned: owned,
        }
    }
    fn get_shared(&self) -> Arc<S> {
        Arc::clone(&self.shared)
    }
}



#[test]
fn test_sharer() {
    let mut _sharer = Sharer::new(AtomicF32::new(3.1), 1);
    let shared = _sharer.get_shared();
    thread::spawn(move || {
        _sharer.owned += 1;
        _sharer.shared.load(Ordering::Relaxed);
    });
    &shared.store(444., Ordering::Relaxed);
    &shared;
}
