use clap::{App, Arg, SubCommand};
use indoc::indoc;

#[tokio::main]
async fn main() {
    let matches = App::new("Maester Web Server")
        .version("0.1")
        .about(indoc!(
            "
               The brains of Maester. Hosts all the serverside logic and does all the things
               that you expect Maester to do."
        ))
        .subcommand(
            SubCommand::with_name("start")
                .about(indoc!("Start the webserver"))
                .arg(
                    Arg::with_name("endpoint")
                        .long("endpoint")
                        .short("e")
                        .takes_value(true)
                        .default_value("127.0.0.1:8000")
                        .help(indoc!("host:port combination to bind to")),
                ),
        )
        .get_matches();
}
