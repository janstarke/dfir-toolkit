use anyhow::Result;
use clap::Parser;
use evtx2bodyfile_app::Evtx2BodyfileApp;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

mod bf_data;
mod evtx2bodyfile_app;
mod evtx_file;
#[macro_use]
mod macros;

fn main() -> Result<()> {
    if std::env::args().any(|a| &a == "--markdown-help") {
        clap_markdown::print_help_markdown::<Evtx2BodyfileApp>();
        return Ok(());
    }
    let cli = Evtx2BodyfileApp::parse();

    TermLogger::init(
        cli.verbose().log_level_filter(),
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    cli.handle_evtx_files()
}
