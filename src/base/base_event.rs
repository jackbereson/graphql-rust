use tokio::sync::broadcast::{self, Sender, Receiver};

// Event type to represent different events in the system
#[derive(Clone)]
pub enum EventType {
    Created,
    Updated,
    Deleted,
    Custom(String),
}

// Generic event struct
#[derive(Clone)]
pub struct Event<T> 
where 
    T: Clone + Send + Sync + 'static
{
    pub event_type: EventType,
    pub payload: T,
}

// EventEmitter to broadcast events
pub struct EventEmitter<T> 
where 
    T: Clone + Send + Sync + 'static
{
    sender: Sender<Event<T>>,
}

impl<T> EventEmitter<T> 
where 
    T: Clone + Send + Sync + 'static
{
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn emit(&self, event_type: EventType, payload: T) {
        let event = Event {
            event_type,
            payload,
        };
        
        // We don't care if there are no listeners
        let _ = self.sender.send(event);
    }
    
    pub fn subscribe(&self) -> Receiver<Event<T>> {
        self.sender.subscribe()
    }
}

// An event bus to manage multiple event types
pub struct EventBus {
    // This would be implemented with a map of type ids to event emitters
    // in a more complete implementation
}

impl EventBus {
    pub fn new() -> Self {
        Self {}
    }
    
    // For a complete implementation, you would have methods to register
    // emitters for different event types and to get those emitters
}
