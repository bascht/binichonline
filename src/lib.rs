pub mod ui {
    extern crate term;
    use net;

    pub fn update(state: net::State) {
        match state {
            net::State::Online  => online(),
            net::State::Offline => offline()
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
    use std::env;
    pub enum State { Online, Offline }
    pub fn is_online() -> State {

        // Going full on rust here. Thanks @skade! :D
        let env_var = env::var_os("ONLINE_CHECK_HOST");
        let check_target = env_var.as_ref()
            .map(|val| val.to_str().unwrap())
            .unwrap_or("https://duckduckgo.com/");

        let resp = http::handle()
            .get(check_target)
            .exec();

        match resp {
            Ok(r) => State::Online,
            Err(e) => State::Offline
        }
    }


}
