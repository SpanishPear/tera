use std::io::{self, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

// we want this to be public to main.rs
// hence pub
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    // clippy says unused self
    // removing self as per https://rust-lang.github.io/rust-clippy/master/index.html#unused_self
    // results in errors :( 
    pub fn run(&mut self) {
        
        let _stdout = stdout().into_raw_mode().unwrap();
       
        loop {
            if let Err(error) = self.process_keypresses() {
                die(error);
            }
            if self.should_quit {
                break;
            }
        }
    }

    fn process_keypresses(&mut self) -> Result<(), std::io::Error> {
        
        let pressed_key = read_key()?;
        match pressed_key  {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    // this is essentially an init function 
    // for the struct
    // with default values (but none for now)
    pub fn default() -> Self {
        Self {
            should_quit: false,
        }
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
        
    }
}


fn die(e: std::io::Error) {
   std::panic::panic_any(e);
}

