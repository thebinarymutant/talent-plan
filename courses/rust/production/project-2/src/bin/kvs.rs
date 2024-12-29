use clap::{Arg, Command};
use kvs::KvStore;
use std::process::exit;

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            Command::new("set")
                .about("Set value of key string to a string")
                .arg(Arg::new("KEY").help("A string key").required(true))
                .arg(
                    Arg::new("VALUE")
                        .help("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("get")
                .about("Get the string value of a given string key")
                .arg(Arg::new("KEY").help("A string key").required(true)),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a given key")
                .arg(Arg::new("KEY").help("A string key").required(true)),
        )
        .get_matches();

    let mut store = KvStore::new();

    match matches.subcommand() {
        Some(("set", args)) => {
            let key = args.get_one::<String>("KEY").unwrap();
            let value = args.get_one::<String>("VALUE").unwrap();
            store.set(key.clone(), value.clone());
            exit(0);
        }
        Some(("get", args)) => {
            let key = args.get_one::<String>("KEY").unwrap();
            store.get(key.clone());
            exit(0);
        }
        Some(("rm", args)) => {
            let key = args.get_one::<String>("KEY").unwrap();
            store.remove(key.clone());
            exit(0);
        }
        _ => unreachable!(),
    }
}
