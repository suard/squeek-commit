use clap::Parser;
use git2::{Commit, Oid, Repository, ResetType, Sort};


fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    println!("current folder: {}", &args.path);
    
    let repository = Repository::discover(args.path)?;
    let current_branch = repository.head()?;
    
    println!("branch: {}", current_branch.shorthand().unwrap());
    
    let number_of_commits_between = count_commits_between(&repository, &args.main, current_branch.shorthand().unwrap())?;

    println!("Number of commits: {}", number_of_commits_between);

    Ok(())
}

fn count_commits_between(repo: &Repository, base_branch: &str, target_branch: &str) -> Result<usize, git2::Error> {
    let base_branch = repo.find_branch(base_branch, git2::BranchType::Local)?;
    let base_commit = base_branch.get().peel_to_commit()?;

    let target_branch = repo.find_branch(target_branch, git2::BranchType::Local)?;
    let target_commit = target_branch.get().peel_to_commit()?;
    
    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(Sort::TOPOLOGICAL)?;
    revwalk.push_range(&format!("{}..{}", base_commit.id(), target_commit.id()))?;

    let count = revwalk.count();

    Ok(count)
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
    #[arg(long, help = "Initiate dry run and not executing the squash")]
    dry: bool,
}
