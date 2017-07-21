extern crate binichonline;

use std::thread;
use std::time::Duration;
use binichonline::ui;
use binichonline::net;

struct Looper {
    interval: std::time::Duration
}

impl Looper {
    pub fn run(&self) {
        loop {
            ui::update(net::is_online());
            thread::sleep(self.interval);
            ui::clear_screen();
        }
    }
}

fn main() {
    Looper { interval: Duration::from_millis(1500) }.run();
}
