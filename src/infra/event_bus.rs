use crate::core::{EventBus, EventHandler};
use crate::domain::DomainEvent;
use parking_lot::RwLock;
use std::sync::Arc;

pub struct InMemoryEventBus {
    handlers: RwLock<Vec<Arc<dyn EventHandler>>>,
}

impl InMemoryEventBus {
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(Vec::new()),
        }
    }
}

impl Default for InMemoryEventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus for InMemoryEventBus {
    fn publish(&self, event: DomainEvent) {
        let handlers = self.handlers.read();
        for h in handlers.iter() {
            h.handle(&event);
        }
    }

    fn subscribe(&self, handler: Arc<dyn EventHandler>) {
        self.handlers.write().push(handler);
    }
}
