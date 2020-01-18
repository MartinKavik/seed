use atomic_refcell::{AtomicRef, AtomicRefCell};

pub struct State<T>(AtomicRefCell<T>);

impl<T> State<T> {
    pub const fn new(data: T) -> Self {
        Self(AtomicRefCell::new(data))
    }

    pub fn get(&self) -> AtomicRef<T> {
        self.0.borrow()
    }

    pub fn update(&self, f: impl FnOnce(&mut T)) {
        f(&mut self.0.borrow_mut());
    }
}
