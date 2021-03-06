mod subcommand;

use clap::{crate_authors, crate_description, crate_version, AppSettings, Clap};
use eyre::Result;
use subcommand::SubCommand;

#[derive(Debug, Clap)]
#[clap(
    version = crate_version!(),
    about = crate_description!(),
    author = crate_authors!(),
    global_setting = AppSettings::ColoredHelp,
)]
pub struct Opt {
    #[clap(flatten)]
    put_opt: subcommand::put::Opt,

    /// How verbose to log. The verbosity is error by default.
    #[clap(short = 'v', long = "verbose", parse(from_occurrences), global = true)]
    pub verbosity: u8,

    /// The subcommand to run. If none is specified, will run `trash put` by default
    #[clap(subcommand)]
    pub subcmd: Option<SubCommand>,
}

impl Opt {
    pub fn run_or_default(self) -> Result<()> {
        match self.subcmd {
            Some(subcmd) => subcmd.run()?,
            None => SubCommand::Put(self.put_opt).run()?,
        }
        Ok(())
    }
}
