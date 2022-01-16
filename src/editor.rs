use crate::Terminal;
use std::io::Write;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect(""),
            cursor_position: Position { x: 0, y: 0 },
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Char('h')
            | Key::Char('j')
            | Key::Char('k')
            | Key::Char('l')
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => println!("{:?} \r", pressed_key),
        }
        Ok(())
    }
    fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;
        match key {
            Key::Char('k') => y = y.saturating_sub(1),
            Key::Char('j') => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Char('h') => x = x.saturating_sub(1),
            Key::Char('l') => {
                if x < width {
                    x = x.saturating_add(1)
                }
            }
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,

            _ => (),
        }
        self.cursor_position = Position { x, y }
    }
    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto editor -- version {}\r", VERSION);
        let len = welcome_message.len();
        let width = self.terminal.size().width as usize;
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", &welcome_message)
    }
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        Terminal::clear_current_line();
        for row in 0..height - 1 {
            if row == height / 3 {
                self.draw_welcome_message()
            } else {
                println!("~\r");
            }
        }
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("error: {}", e);
}
