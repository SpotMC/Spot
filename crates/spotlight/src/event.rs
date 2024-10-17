use std::collections::VecDeque;

pub type Callback<T> = dyn Fn(T) -> ActionResult + Sync + Send;
pub struct EventCallback<T: Clone> {
    callback: VecDeque<Box<Callback<T>>>,
}

impl<T: Clone> Default for EventCallback<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> EventCallback<T> {
    pub fn new() -> EventCallback<T> {
        EventCallback {
            callback: VecDeque::new(),
        }
    }

    pub fn register<F: Fn(T) -> ActionResult + 'static + Sync + Send>(&mut self, callback: F) {
        self.callback.push_back(Box::new(callback));
    }

    pub fn insert<F: Fn(T) -> ActionResult + 'static + Sync + Send>(&mut self, callback: F) {
        self.callback.push_front(Box::new(callback));
    }

    pub fn interact(&self, event: T) -> ActionResult {
        let mut result = ActionResult::Success;
        for callback in self.callback.iter() {
            match callback(event.clone()) {
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
