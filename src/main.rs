use clap::Parser;
use git2::ResetType;

fn main() -> Result<(), git2::Error> {
    let args = Args::parse();

    match git2::Repository::discover(args.path) {
        Ok(repository) => {
            let head = repository.head()?.resolve()?;
            let head_commit = head.peel_to_commit()?;

            let current_branch = repository.head()?;
            let current_commit = current_branch.target().unwrap();

            let main_branch = repository.find_branch(&args.main, git2::BranchType::Local)?;
            let main_commit = main_branch.get().peel_to_commit()?;

            let mut revwalk = repository.revwalk().expect("Failed to create revwalk");
            revwalk.push_range(&format!("{}..{}", main_commit.id(), current_commit))?;
            let commit_count = revwalk.count();

            if commit_count == 0 {
                println!("Branch is not ahead of master, nothing to squash.");
                return Ok(());
            }

            println!("Squashing {} commits...", commit_count);

            let mut revwalk = repository.revwalk()?;
            revwalk.push(head_commit.id())?;
            revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;

            let target_commit_id = revwalk
                .nth(commit_count - 1) // Get the commit before our extra commits
                .ok_or_else(|| git2::Error::from_str("Not enough commits to squash"))??;

            let target_commit = repository.find_commit(target_commit_id)?;
            repository.reset(target_commit.as_object(), ResetType::Soft, None)?;

            // Create a new commit with all the staged changes
            let sig = repository.signature()?;
            let tree_id = repository.index()?.write_tree()?;
            let tree = repository.find_tree(tree_id)?;

            if args.dry {
                repository.commit(
                    Some("HEAD"),      // Commit to HEAD
                    &sig,              // Author
                    &sig,              // Committer
                    "Squashed commit", // Commit message
                    &tree,             // Tree
                    &[&target_commit], // Parent commit
                )?;
            }

            println!("Successfully squashed {} commits into one.", commit_count);
        }
        Err(e) => {
            println!("Error, Could not find any git repository: {}", e);
        }
    }

    Ok(())
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
