use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    version = "1.0",
    about = "Squeek Commit; squashes your commits and saves the first message"
)]
pub struct Args {
    #[arg(
        short,
        long,
        default_value = ".",
        help = "The path to the git repository eg /home/user/repo"
    )]
    pub path: PathBuf,
    #[arg(
        short,
        long,
        default_value = "main",
        help = "The main branch name eg main, master etc"
    )]
    pub main: String,
    #[arg(
        short = 'a',
        long,
        help = "Override the commit message of the squashed commit"
    )]
    pub message: Option<String>,
    #[arg(long, help = "Initiate dry run and not executing the squash")]
    pub dry: bool,
}
