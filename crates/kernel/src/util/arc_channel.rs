use hashbrown::HashMap;
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;
use std::thread::yield_now;
use std::time::{Duration, Instant};

pub struct ArcChannel<T> {
    receivers: Vec<Sender<T>>,
}

struct Sender<T> {
    parent: usize,
    id: usize,
    queue: Arc<Mutex<VecDeque<Arc<T>>>>,
}

#[derive(Clone)]
pub struct Receiver<T> {
    parent: usize,
    id: usize,
    queue: Arc<Mutex<VecDeque<Arc<T>>>>,
}

pub struct MultipleReceiver<T> {
    receivers: HashMap<u64, Receiver<T>>,
}

impl<T> Default for ArcChannel<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ArcChannel<T> {
    pub fn new() -> ArcChannel<T> {
        ArcChannel {
            receivers: Vec::new(),
        }
    }
    pub fn broadcast(&self, item: T) {
        let arc = Arc::new(item);
        for receiver in self.receivers.iter() {
            receiver.queue.lock().push_back(arc.clone());
        }
    }

    pub fn subscribe(&mut self) -> Receiver<T> {
        let queue = Arc::new(Mutex::new(VecDeque::new()));
        let id = self.receivers.len();
        let parent = self.receivers.as_ptr() as usize;
        let sender = Sender {
            parent,
            id,
            queue: queue.clone(),
        };
        self.receivers.push(sender);
        Receiver { parent, id, queue }
    }
    pub fn remove(&mut self, receiver: &Receiver<T>) -> bool {
        if receiver.parent != self.receivers.as_ptr() as usize {
            return false;
        }
        self.receivers.remove(receiver.id);
        true
    }
}

impl<T> Receiver<T> {
    pub fn try_receive(&self) -> Option<Arc<T>> {
        let mut queue = self.queue.lock();
        queue.pop_front()
    }

    pub fn receive(&self) -> Arc<T> {
        loop {
            if let Some(item) = self.try_receive() {
                return item;
            }
            yield_now();
        }
    }

    pub fn receive_timeout(&self, timeout: Duration) -> Option<Arc<T>> {
        let start = Instant::now();
        loop {
            if let Some(item) = self.try_receive() {
                return Some(item);
            }
            if start.elapsed() >= timeout {
                return None;
            }
        }
    }
}

impl<T> Default for MultipleReceiver<T> {
    fn default() -> Self {
        Self {
            receivers: HashMap::new(),
        }
    }
}

impl<T> MultipleReceiver<T> {
    pub fn new() -> MultipleReceiver<T> {
        MultipleReceiver {
            receivers: HashMap::new(),
        }
    }
    pub fn add(&mut self, pos: u64, receiver: Receiver<T>) -> usize {
        self.receivers.insert(pos, receiver);
        self.receivers.len() - 1
    }

    pub fn remove(&mut self, index: u64) -> Option<Receiver<T>> {
        self.receivers.remove(&index)
    }
    pub fn receive(&self) -> Arc<T> {
        loop {
            for receiver in self.receivers.iter() {
                if let Some(item) = receiver.1.try_receive() {
                    return item;
                }
            }
            yield_now();
        }
    }
    pub fn receive_timeout(&self, timeout: Duration) -> Option<Arc<T>> {
        let start = Instant::now();
        loop {
            for receiver in self.receivers.iter() {
                if let Some(item) = receiver.1.try_receive() {
                    return Some(item);
                }
            }
            if start.elapsed() >= timeout {
                return None;
            }
            yield_now();
        }
    }
    pub fn try_receive(&self) -> Option<Arc<T>> {
        for receiver in self.receivers.iter() {
            if let Some(item) = receiver.1.try_receive() {
                return Some(item);
            }
        }
        None
    }
}
