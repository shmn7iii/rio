use crate::event::sync::FairMutex;
use crate::event::RioEvent;
use std::collections::LinkedList;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct InnerData {
    list: LinkedList<RioEvent>,
    redraw: Vec<u8>,
    priority_list: Vec<RioEvent>,
}

pub struct Inner(InnerData);

impl Inner {
    /// Create a new, empty event listener list.
    pub fn new() -> Self {
        Self(InnerData {
            list: LinkedList::new(),
            redraw: Vec::new(),
            priority_list: Vec::new(),
        })
    }
}

pub struct Instance {
    pub inner: Inner,
}

impl Instance {
    pub fn new() -> Instance {
        Instance {
            inner: Inner::new(),
        }
    }
}

#[derive(Clone)]
pub struct Superloop {
    instance: Arc<FairMutex<Instance>>,
    // size: AtomicUsize,
}

impl Superloop {
    pub fn new() -> Superloop {
        Superloop {
            instance: Arc::new(FairMutex::new(Instance {
                inner: Inner::new(),
            })),
            // size: AtomicUsize::new(0),
        }
    }

    #[inline]
    pub fn event(&mut self) -> (Option<RioEvent>, bool) {
        let inner = &mut self.instance.lock().inner.0;
        // println!("{:?}", inner.list.len());

        let redraw = if !inner.redraw.is_empty() {
            inner.redraw.pop();
            true
        } else {
            false
        };

        let current_event = if !inner.priority_list.is_empty() {
            inner.priority_list.pop()
        } else {
            inner.list.pop_front()
        };

        (current_event, redraw)
    }

    #[inline]
    pub fn send_event(&mut self, event: RioEvent, _id: u16) {
        self.instance.lock().inner.0.list.push_back(event);
        // self.size.fetch_add(1, Ordering::SeqCst);
    }

    #[inline]
    pub fn send_event_with_high_priority(&mut self, event: RioEvent, _id: u16) {
        self.instance.lock().inner.0.priority_list.push(event);
        // self.size.fetch_add(1, Ordering::SeqCst);
    }

    #[inline]
    pub fn send_redraw(&mut self, _id: u16) {
        self.instance.lock().inner.0.redraw.push(0);
        // self.size.fetch_add(1, Ordering::SeqCst);
    }
}

impl std::fmt::Debug for Superloop {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Instance")
    }
}

impl core::panic::UnwindSafe for Superloop {}
impl core::panic::RefUnwindSafe for Superloop {}
