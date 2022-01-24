use clap::{App, Arg, ArgMatches};

pub struct Cli<'a> {
    app: App<'a>,
}

impl<'a> Cli<'a> {
    pub fn new() -> Self {
        let app = App::new("mahasiswa-finder")
            .setting(clap::AppSettings::ArgRequiredElseHelp)
            .version("0.0.1")
            .author("globalpolaris, nomadilog@gmail.com")
            .arg(
                Arg::new("name")
                    .help("Name of the student")
                    .long("name")
                    .short('n')
                    .takes_value(true)
                    .required(true),
            );
        Self { app }
    }

    pub fn get_arg(self) -> ArgMatches {
        self.app.get_matches()
    }
}
