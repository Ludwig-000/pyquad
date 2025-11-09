#![allow(unused)]
#![allow(dead_code)]
/// WIP, will see if this ever gets anywhere. 
/// 

use std::cell::UnsafeCell;
use std::ops::Deref;
use std::sync::{mpsc, Mutex};

pub enum PromiseState<T> {
    Pending(Option<mpsc::Receiver<T>>),
    Ready,
}

pub struct Promise<T> {
    state: Mutex<PromiseState<T>>,
    value: UnsafeCell<Option<T>>,
}

// Required to make &Promise<T> safe to share across threads.
// UnsafeCell<T> itself isn’t Sync, but we guard mutation with a Mutex.
unsafe impl<T: Send> Send for Promise<T> {}
unsafe impl<T: Send + Sync> Sync for Promise<T> {}

impl<T> Promise<T> {
    pub fn from_receiver(rx: mpsc::Receiver<T>) -> Self {
        Self {
            state: Mutex::new(PromiseState::Pending(Some(rx))),
            value: UnsafeCell::new(None),
        }
    }

    pub fn from_value(val: T) -> Self {
        Self {
            state: Mutex::new(PromiseState::Ready),
            value: UnsafeCell::new(Some(val)),
        }
    }

    fn resolve(&self) {
        let mut state_guard = self.state.lock().unwrap();

        // Only resolve once
        if let PromiseState::Pending(ref mut maybe_rx) = *state_guard {
            if let Some(rx) = maybe_rx.take() {
                drop(state_guard); // unlock before blocking

                let val = rx.recv().expect("Promise sender dropped before sending value");

                // SAFETY: we have unique access through &self (since we're under lock)
                unsafe {
                    *self.value.get() = Some(val);
                }

                let mut state_guard = self.state.lock().unwrap();
                *state_guard = PromiseState::Ready;
            }
        }
    }
}

impl<T> Deref for Promise<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Fast path: check if already ready
        let is_ready = {
            let state_guard = self.state.lock().unwrap();
            matches!(*state_guard, PromiseState::Ready)
        };

        if !is_ready {
            self.resolve();
        }

        unsafe {
            (*self.value.get())
                .as_ref()
                .expect("Promise value not set after resolve")
        }
    }
}
