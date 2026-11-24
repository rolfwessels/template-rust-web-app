use app_core::{ListPort, ListSnapshot, ListUpdates};
use std::sync::Arc;
use tokio::sync::watch;
use tokio_stream::wrappers::WatchStream;

pub struct InMemoryList {
    snapshots: watch::Sender<ListSnapshot>,
}

impl InMemoryList {
    pub fn new(initial: Vec<String>) -> Self {
        Self {
            snapshots: watch::Sender::new(Arc::new(initial)),
        }
    }
}

impl ListPort for InMemoryList {
    fn read(&self) -> ListSnapshot {
        self.snapshots.borrow().clone()
    }

    fn append(&self, value: String) {
        self.snapshots
            .send_modify(|snapshot| Arc::make_mut(snapshot).push(value));
    }

    fn subscribe(&self) -> ListUpdates {
        Box::pin(WatchStream::new(self.snapshots.subscribe()))
    }
}
