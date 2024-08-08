use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

pub fn start_spinner(spins: u32, delay: u64) {
    let spinner_chars = ['|', '/', '-', '\\'];
    let mut idx = 0;

    for _ in 0..spins {
        print!("\r{}", spinner_chars[idx]);
        io::stdout().flush().unwrap();
        idx = (idx + 1) % spinner_chars.len();
        sleep(Duration::from_millis(delay));
    }

    print!("\rDone!        \n");
}
