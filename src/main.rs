mod util;
mod opts;
mod lists;
mod analyze;

use crate::opts::*;
use crate::lists::*;
use crate::analyze::*;
use clap::Clap;

fn main() {
    let opts = Opts::parse();
    match opts.subcmd {
	SubCommand::UpdateDnsLists(cmd) => update_dns_lists(&cmd.source),
	SubCommand::Analyze(cmd) => analyze(cmd)
    }
}
