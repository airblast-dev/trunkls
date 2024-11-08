mod init;
mod lsp;
mod utils;

use init::initialize_result;
use lsp::main_loop;

use std::{fs::File, io::stderr};

use clap::Parser;
use lsp_server::Connection;
use lsp_types::InitializeParams;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Default, Parser)]
struct Config {
    #[arg(short = 'o', long = "log-file")]
    log_file: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let (con, _th) = Connection::stdio();
    let (id, resp) = con.initialize_start()?;
    let resp: InitializeParams = serde_json::from_value(resp)?;
    let (text_fn, init_res) = initialize_result(&resp);
    con.initialize_finish(id, serde_json::to_value(init_res)?)?;

    let cfg = Config::parse();

    if let Some(lf) = cfg.log_file.as_deref() {
        log_to_file(lf)?;
    } else {
        log_to_stdout();
    }

    main_loop(text_fn, con)?;

    Ok(())
}

fn log_to_file<S: AsRef<str>>(path: S) -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(
            File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path.as_ref())?,
        )
        .with_ansi(false)
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

fn log_to_stdout() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
