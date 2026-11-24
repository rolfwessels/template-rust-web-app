//! List use cases and the outbound list port.

use futures_core::Stream;
use std::{pin::Pin, sync::Arc};

pub type ListSnapshot = Arc<Vec<String>>;
pub type ListUpdates = Pin<Box<dyn Stream<Item = ListSnapshot> + Send>>;

pub trait ListPort: Send + Sync {
    fn read(&self) -> ListSnapshot;
    fn append(&self, value: String);
    fn subscribe(&self) -> ListUpdates;
}

pub struct StringListApp {
    list: Box<dyn ListPort>,
    generate: Box<dyn Fn() -> String + Send + Sync>,
}

impl StringListApp {
    pub fn new(
        list: impl ListPort + 'static,
        generate: impl Fn() -> String + Send + Sync + 'static,
    ) -> Self {
        Self {
            list: Box::new(list),
            generate: Box::new(generate),
        }
    }

    pub fn read(&self) -> ListSnapshot {
        self.list.read()
    }

    pub fn add_random_string(&self) -> String {
        let value = (self.generate)();
        self.list.append(value.clone());
        value
    }

    pub fn subscribe(&self) -> ListUpdates {
        self.list.subscribe()
    }
}
