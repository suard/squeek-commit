use clap::Parser;

fn main() {
    let args = Args::parse();

    match git2::Repository::discover(args.path) {
        Ok(repository) => {
            let current_branch = repository.head().expect("Failed to get current branch");
            let current_commit = current_branch
                .target()
                .expect("No commit in current branch");

            println!(
                "Current branch: {:?}",
                current_branch.name().expect("Failed to get branch name")
            );
            println!("Current commit: {:?}", current_commit);

            let main_branch = repository
                .find_branch(&args.main, git2::BranchType::Local)
                .expect("Failed to find master branch");
            let main_commit = main_branch
                .get()
                .peel_to_commit()
                .expect("Failed to get commit from master branch");

            println!(
                "Master branch: {:?}",
                main_branch.name().expect("Failed to get branch name")
            );
            println!("Master commit: {:?}", main_commit);
        }
        Err(e) => {
            println!("Error, Could not find any git repository: {}", e);
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    version = "1.0",
    about = "Squeek Commit; squashes your commits and saves the first message"
)]
struct Args {
    #[arg(
        short,
        long,
        default_value = ".",
        help = "The path to the git repository eg /home/user/repo"
    )]
    path: String,
    #[arg(
        short,
        long,
        default_value = "main",
        help = "The main branch name eg main, master etc"
    )]
    main: String,
}
