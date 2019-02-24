use std::thread::sleep;
use std::time::Duration;

use wooting_sdk::{analog, Key};

fn main() {
    println!(
        "Keyboard connected? {}",
        analog::is_wooting_keyboard_connected()
    );
    println!("Reading keyboard state in 5 seconds...");
    sleep(Duration::from_millis(5000));
    println!("Reading...");
    println!(
        "Down arrow has analog value: {}",
        analog::read_analog_key(Key::DownArrow).unwrap(),
    );
    println!("Finished!");
}
