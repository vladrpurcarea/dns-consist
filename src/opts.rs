use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1", author = "vlad")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand
}

#[derive(Clap)]
pub enum SubCommand {
    UpdateDnsLists(UpdateDnsLists),
    Analyze(Analyze),
}

#[derive(Clap)]
pub struct UpdateDnsLists {
    #[clap(long, default_value="https://public-dns.info/nameservers-all.csv")]
    pub source: String
}

#[derive(Clap,Clone)]
pub struct Analyze {
    #[clap(short, long)]
    pub website: String,
    #[clap(long)]
    pub full: bool,
    #[clap(long)]
    pub threads: Option<usize>,
    #[clap(long)]
    pub max_servers: Option<usize>,
    #[clap(short, long)]
    pub output_file: Option<String>
}   
