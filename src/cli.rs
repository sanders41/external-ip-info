use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about = "Prints external ip address inforamtion")]
pub struct Cli {
    #[clap(short, long, help = "Clear cache")]
    pub clear_cache: bool,
}
