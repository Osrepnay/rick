use rand::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut rng = rand::thread_rng();
    loop {
        let random_num: u64 = (rng.gen::<f64>() * 360.0) as u64;
        if random_num == 0 {
            webbrowser::open("https://www.youtube.com/watch?v=dQw4w9WgXcQ").unwrap();
        }
        thread::sleep(Duration::from_secs(1));
    }
}
