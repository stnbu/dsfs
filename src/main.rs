/// `dsfs`
///
/// The "missing" rust webdev utility server (pre-alpha)
///
/// If you cannot `cargo install dsfs`, I have failed. The quality to improve!
///
/// It re-uses existing tools and is dead-simple as a "feature".
///
/// ```
/// dsfs                         # serve current directory at 0.0.0.0:8000
/// dsfs --address 1.2.3.4 ./www # serve ./www at 1.2.3.4:8000
/// dsfs --address 1.2.3.4:9000  # serve current directory at 1.2.3.4:9000
/// dsfs ./www                   # serve ./www at 0.0.0.0:8000
/// ```
use clap::{App, Arg};
use rocket::config::{Config, Environment};
use rocket_contrib::serve::StaticFiles;

fn main() {
    let matches = App::new("Dead Simple File Server")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .arg(
            Arg::with_name("address")
                .short("a")
                .long("address")
                .value_name("IPV4_ADDR")
                .help("An IPv4 address of the form a.b.c.d:1234")
                .takes_value(true),
        )
        .arg(Arg::with_name("path").index(1))
        .get_matches();
    let address = matches.value_of("address").unwrap_or("0.0.0.0:8000");
    let path = matches.value_of("path").unwrap_or(".");

    let mut split = address.split(":");
    let ip = split.next().unwrap_or("0.0.0.0");
    let port = split.next().unwrap_or("8000");
    let port = port.parse().expect("not a number");

    let config = Config::build(Environment::Staging)
        .address(ip)
        .port(port)
        .finalize()
        .expect("Unable to finalize configuration.");

    rocket::custom(config)
        .mount("/", StaticFiles::from(path))
        .launch();
}
