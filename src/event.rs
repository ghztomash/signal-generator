use color_eyre::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

/// Terminal events that can be handled by the application.
#[derive(Debug, Clone, Copy)]
pub enum Event {
    /// Terminal tick event.
    Tick,
    /// Terminal key event.
    Key(KeyEvent),
    /// Terminal mouse event.
    Mouse(MouseEvent),
    /// Terminal resize event.
    Resize(u16, u16),
}

/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    /// Event channel sender.
    sender: mpsc::Sender<Event>,
    /// Event channel receiver.
    receiver: mpsc::Receiver<Event>,
    /// Event handler thread.
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Create a new event handler.
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    // poll for tick rate duration, if no events, sent tick event.
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    // poll for events
                    if event::poll(timeout).expect("unable to poll for event") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(key) => {
                                if key.kind == event::KeyEventKind::Press {
                                    sender.send(Event::Key(key))
                                } else {
                                    Ok(()) // ignore key release events
                                }
                            }
                            CrosstermEvent::Mouse(mouse) => sender.send(Event::Mouse(mouse)),
                            CrosstermEvent::Resize(width, height) => {
                                sender.send(Event::Resize(width, height))
                            }
                            _ => unimplemented!(),
                        }
                        .expect("unable to send event");
                    }

                    // send tick event
                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("unable to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receive the next event
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}
