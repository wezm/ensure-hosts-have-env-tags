extern crate failure;
extern crate ensure_hosts_have_env_tags as local;

use std::env;
use std::collections::HashMap;

use failure::Error;
use local::datadog::Client;

const ENVIRONMENTS: [&str; 6] = ["prod", "demo", "staging", "dev", "end2end", "presales"];

fn hostname_to_env(hostname: &str) -> Option<&str> {
    ENVIRONMENTS.iter().find(|&env| hostname.contains(env)).map(|env| *env)
}

fn run() -> Result<(), Error> {
    let api_key = env::var("DATADOG_API_KEY").expect("DATADOG_API_KEY must be set");
    let app_key = env::var("DATADOG_APP_KEY").expect("DATADOG_APP_KEY must be set");

    let datadog = Client::new(api_key, app_key);

    let results = datadog.search(None)?;
    println!("Hosts: {}", results.hosts.len());

    let hostmap = results.hosts.iter()
        .filter_map(|host| hostname_to_env(host).map(|env| (host, env)))
        .fold(HashMap::new(), |mut map, (host, env)| {
            map.entry(env).or_insert(vec![]).push(host);
            map
        });

    println!("Mapped hosts {} {:?}", hostmap.values().map(|arr| arr.len()).sum::<usize>(), hostmap);

    Ok(())
}

fn main() {
    match run() {
        Err(error) => {
            println!("{}, {}", error.cause(), error.backtrace());
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}
