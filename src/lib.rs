pub mod ui {
    extern crate term;

    pub fn online() {
        let mut t = term::stdout().unwrap();
        t.fg(term::color::GREEN).unwrap();
        writeln!(t, "Online").unwrap();

    }
    pub fn offline() {
        let mut t = term::stdout().unwrap();
        t.fg(term::color::RED).unwrap();
        writeln!(t, "Offline").unwrap();

    }
    pub fn clear_screen() {
        println!("\x1b[2J\x1b[1;1H");
    }
}

pub mod net {
    extern crate curl;

    use self::curl::http;

    pub fn is_online() -> bool {
        let resp = http::handle()
            .get("http://localhost:8000")
            .exec();

        match resp {
            Ok(r) => true,
            Err(e) => false
        }
    }


}
