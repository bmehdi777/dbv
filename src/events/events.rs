use crate::events::key::Keys;
use crossterm::event::{self, Event};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub struct EventsHandling {
    tx: Sender<EventThread>,
    rx: Receiver<EventThread>,
}

pub enum EventThread {
    Tick, // nothing happened
    Event(Keys),
}

impl EventsHandling {
    pub fn new() -> Self {
        let (tx, rx) = channel::<EventThread>();
        Self { tx, rx }
    }

    pub fn start(self) -> Self {
        let etx = self.tx.clone();
        thread::spawn(move || loop {
            if event::poll(std::time::Duration::from_millis(16))
                .expect("An error occured while polling event.")
            {
                match event::read().expect("An error occured while reading event.") {
                    Event::Key(key) => {
                        let k: Keys = key.into();
                        etx.send(EventThread::Event(k))
                            .expect("An error occured while sending event.");
                    }
                    Event::Mouse(_) => {
                        // todo one day ?
                    }
                    _ => {
                        etx.send(EventThread::Tick)
                            .expect("An error occured while sending tick.");
                    }
                }
            }
            etx.send(EventThread::Tick)
                .expect("An error occured while sending tick.");
        });
        self
    }

    pub fn next(&self) -> Result<EventThread, std::sync::mpsc::RecvError> {
        self.rx.recv()
    }

    pub fn send_key(&self, key: Keys) -> anyhow::Result<()> {
        self.tx.send(EventThread::Event(key))?;
        Ok(())

    }
}
