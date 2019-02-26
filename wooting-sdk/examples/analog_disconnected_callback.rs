use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use wooting_sdk::{analog, Key};

fn main() {
    println!("Waiting until keyboard is connected...");
    loop {
        // Only continue if keyboard is connected.
        if analog::is_wooting_keyboard_connected() {
            break;
        }
    }
    println!("Connected...");

    println!("Setting callback...");
    analog::set_disconnected_callback(|| {
        println!("Callback triggered...");
        println!("Finished!");
        exit(0);
    });

    println!("Waiting until disconnect...");
    loop {
        // Trigger a read so that a disconnect will be noticed. This is a limitation of that API.
        let _ = analog::read_analog_key(Key::Escape);
        sleep(Duration::from_millis(1000));
    }
}
