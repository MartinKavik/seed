use std::rc::Rc;
use std::cell::{RefCell, Ref};
use polymap::PolyMap;

type Id = usize;

pub fn use_state<T: 'static>(sm: &StateManager, initial_data: T) -> State<T> {
    let id = sm.next_id();
    if let Some(state) = sm.state(id) {
        state
    } else {
        let state = State::new(initial_data);
        sm.insert_state(id, state.clone());
        state
    }
}

// ------ StateManager ------

pub struct StateManager(RefCell<InternalState>);

struct InternalState {
    id: Id,
    states: PolyMap<Id>,
}

impl StateManager {
    pub fn reset_id(&self) {
        self.0.borrow_mut().id = 0;
    }

    pub fn next_id(&self) -> Id {
        let old_id = self.0.borrow().id;
        self.0.borrow_mut().id = old_id + 1;
        old_id
    }

    pub fn state<T: 'static>(&self, id: Id) -> Option<State<T>> {
        self.0.borrow().states.get(&id).cloned()
    }

    pub fn insert_state<T: 'static>(&self, id: Id, state: State<T>) {
        self.0.borrow_mut().states.insert(id, state);
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self(RefCell::new(InternalState {
            id: 0,
            states: PolyMap::new(),
        }),
        )
    }
}

// ------ State ------

pub struct State<T>(Rc<RefCell<T>>);

impl<T> State<T> {
    fn new(data: T) -> Self {
        Self(Rc::new(RefCell::new(data)))
    }

    pub fn get(&self) -> Ref<T> {
        self.0.borrow()
    }

    pub fn update(&self, f: impl FnOnce(&mut T)) {
        f(&mut self.0.borrow_mut());
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
