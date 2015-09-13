pub mod ui {
    extern crate term;
    use net;

    pub fn update(state: Result<net::State, net::State>) {
        match state {
            Ok(net::State::Online)  => online(),
            Err(net::State::Offline) => offline(),
            _ => println!("WHAT?"),
        }
    }

    pub fn online() {
        let mut t = term::stdout().unwrap();
        t.fg(term::color::GREEN).unwrap();
        writeln!(t, "Online").unwrap();
        t.reset();
    }
    pub fn offline() {
        let mut t = term::stdout().unwrap();
        t.fg(term::color::RED).unwrap();
        writeln!(t, "Offline").unwrap();
        t.reset();
    }
    pub fn clear_screen() {
        println!("\x1b[2J\x1b[1;1H");
    }
}

pub mod net {
    extern crate curl;
    use self::curl::http;
    pub enum State { Online, Offline }
    pub fn is_online() -> Result<State, State> {
        let resp = http::handle()
            .get("http://localhost:8000")
            .exec();

        match resp {
            Ok(r) => Ok(State::Online),
            Err(e) => Err(State::Offline)
        }
    }


}
