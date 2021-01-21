use termion::event::Key;
use termion::input::TermRead;

use std::sync::mpsc;
use std::time::Duration;
use std::{io, thread};

pub enum Event<T> {
    Input(T),
    Continue,
}

pub struct Config {
    exit_key: Key,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: Key::Char('q'),
        }
    }
}

pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let tx2 = tx.clone();

        thread::spawn(move || {
            let stdin = io::stdin();
            for evt in stdin.keys() {
                if let Ok(key) = evt {
                    if let Err(err) = tx.send(Event::Input(key)) {
                        eprintln!("{}", err);
                        return;
                    }
                    if key == config.exit_key {
                        return;
                    }
                }
            }
        });

        thread::spawn(move || loop {
            if tx2.send(Event::Continue).is_err() {
                break;
            }
            thread::sleep(Duration::from_millis(150));
        });

        Events { rx }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
