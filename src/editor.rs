use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

fn die(e: std::io::Error) {
    panic!("error: {}", e);
}

impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{:?} \r", c as u8);
                        } else {
                            println!("{:?} ({})\r", c as u8, c);
                        }
                    }
                    Key::Ctrl('q') => break,
                    _ => println!("{:?} \r", key),
                },
                Err(err) => die(err),
            }
        }
    }

    pub fn default() -> Self {
        Self {}
    }

    fn process_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => panic!("Program end"),
            _ => (),
        }
        Ok(())
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock.keys().next() {
            return key;
        }
    }
}
