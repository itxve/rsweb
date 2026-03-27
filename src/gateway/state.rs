use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct IndexState {
    pub id: Arc<Mutex<i32>>,
}

impl IndexState {
    pub fn new() -> Self {
        Self {
            id: Arc::new(Mutex::new(0i32)),
        }
    }
}
