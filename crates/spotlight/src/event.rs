use std::collections::VecDeque;

pub trait Callback<T> = Fn(&T) -> ActionResult + Sync + Send + 'static;
pub struct EventCallback<T> {
    callback: VecDeque<Box<dyn Callback<T>>>,
}

impl<T> Default for EventCallback<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> EventCallback<T> {
    pub const fn new() -> EventCallback<T> {
        EventCallback {
            callback: VecDeque::new(),
        }
    }

    pub fn register<F: Callback<T>>(&mut self, callback: F) {
        self.callback.push_back(Box::new(callback));
    }

    pub fn insert<F: Callback<T>>(&mut self, callback: F) {
        self.callback.push_front(Box::new(callback));
    }

    pub fn interact(&self, event: T) -> ActionResult {
        let mut result = ActionResult::Success;
        for callback in self.callback.iter() {
            match callback(&event) {
                ActionResult::Success => {
                    result = ActionResult::Success;
                    break;
                }
                ActionResult::Fail => {
                    result = ActionResult::Fail;
                    break;
                }
                ActionResult::Pass => {}
            };
        }
        result
    }
}

pub enum ActionResult {
    Success,
    Fail,
    Pass,
}
