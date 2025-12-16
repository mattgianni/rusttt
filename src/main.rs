use log::{debug, error, trace};
use ttt::cli::Cli;

fn init_logging(verbosity: u8) {
    use env_logger::Env;
    use log::LevelFilter;

    let level = match verbosity {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    env_logger::Builder::from_env(Env::default().default_filter_or(level.to_string()))
        .format_timestamp_secs()
        .init();
}

fn main() {
    let cli = <Cli as clap::Parser>::parse();
    init_logging(cli.verbose);

    trace!("trace logging enabled.");
    debug!("main called with {:?}.", cli);

    if let Err(e) = ttt::run(cli) {
        error!("{e}");
        std::process::exit(1);
    }
}
