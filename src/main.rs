use clap::Parser;

fn main() {
    let args = Args::parse();
    let path = args.path.unwrap();

    match git2::Repository::discover(path) {
        Ok(repository) => {
            let current_branch = repository.head().expect("Failed to get current branch");
            let current_commit = current_branch.target().expect("No commit in current branch");

            println!("Current branch: {:?}", current_branch.name().expect("Failed to get branch name"));
            println!("Current commit: {:?}", current_commit);


        }
        Err(e) => {
            println!("Error, you suck bro {}", e);
        }
    }
}

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Squeek Commit; squashes your commits and saves the first message")]
struct Args {
    #[arg(short, long, default_value = ".", help = "The path to the git repository eg /home/user/repo")]
    path: Option<String>,
}
