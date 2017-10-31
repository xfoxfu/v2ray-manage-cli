use clap::ArgMatches;
use config;

pub fn log(config: &mut config::Config, matches: &ArgMatches) {
    println!("Key: {}", matches.value_of("key").unwrap());
    match matches.value_of("set") {
        Some(value) => {
            let log = match config.log {
                Some(_) => config.log.as_mut().unwrap(),
                None => {
                    config.log = Some(config::Log {
                        ..Default::default()
                    });
                    config.log.as_mut().unwrap()
                }
            };
            match matches.value_of("key").unwrap() {
                "access" => log.access = Some(String::from(value)),
                "error" => log.error = Some(String::from(value)),
                "loglevel" => log.loglevel = Some(String::from(value)),
                value => panic!("invalid key {}", value),
            }
        }
        None => if matches.is_present("get") {
            let null = &String::from("null");
            println!(
                "{} = {}",
                matches.value_of("key").unwrap(),
                if config.log.is_none() {
                    &null
                } else {
                    match matches.value_of("key").unwrap() {
                        "access" => config
                            .log
                            .as_ref()
                            .unwrap()
                            .access
                            .as_ref()
                            .unwrap_or(&null),
                        "error" => config.log.as_ref().unwrap().error.as_ref().unwrap_or(&null),
                        "loglevel" => config
                            .log
                            .as_ref()
                            .unwrap()
                            .loglevel
                            .as_ref()
                            .unwrap_or(&null),
                        value => panic!("invalid key {}", value),
                    }
                }
            )
        } else {
            panic!("no operation provided")
        },
    }
}
