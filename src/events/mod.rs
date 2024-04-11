pub mod key;
pub mod events;

pub enum EventState{
    Consumed,
    ConfirmedText(String),
    Escaped,
    Wasted
}
