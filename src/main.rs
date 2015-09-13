extern crate curl;
extern crate term;

use std::thread;
use curl::http;

struct Looper {
    interval: u32
}

impl Looper {

    fn is_online() -> bool {
        let resp = http::handle()
            .get("http://localhost:8000")
            .exec();

        match resp {
            Ok(r) => true,
            Err(e) => false
        }
    }

    pub fn run(&self) {
        loop {
            let mut t = term::stdout().unwrap();
            match Looper::is_online() {
                true => {
                    t.fg(term::color::GREEN).unwrap();
                    writeln!(t, "Online").unwrap();

                },
                false =>
                {
                    t.fg(term::color::RED).unwrap();
                    writeln!(t, "Offline").unwrap();
                }
            }
            thread::sleep_ms(self.interval);
            clear_screen();
        }
    }
}

fn clear_screen() {
    println!("\x1b[2J\x1b[1;1H");
}

fn main() {
    Looper { interval: 1500 }.run();
}
