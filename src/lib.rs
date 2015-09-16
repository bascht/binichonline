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
    extern crate url;
    use self::curl::http;
    use std::env;
    use self::
    url::Url;

    pub enum State { Online, Offline }
    pub fn is_online() -> State {

        // Going full on rust here. Thanks @skade! :D
        let env_var = env::var_os("ONLINE_CHECK_HOST");

        let url = match env_var {
            Some(ref val) => val.to_str().unwrap(),
            None => "http://duckduckgo.com/"
        };

        let check_target = match Url::parse(url) {
            Ok(v) => v,
            Err(e) => panic!("Sorry, URL {:?} is broken because of a {:?} error.", url, e)
        };

        let resp = http::handle()
            .get(&check_target)
            .exec();

        match resp {
            Ok(r) => State::Online,
            Err(e) => State::Offline
        }
    }


}
