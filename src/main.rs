extern crate binichonline;

use std::thread;
use binichonline::ui;
use binichonline::net;

struct Looper {
    interval: u32
}

impl Looper {
    pub fn run(&self) {
        loop {
            match net::is_online() {
                true  => ui::online(),
                false => ui::offline()
            }
            thread::sleep_ms(self.interval);
            ui::clear_screen();
        }
    }
}

fn main() {
    Looper { interval: 1500 }.run();
}
