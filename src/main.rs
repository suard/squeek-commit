use clap::Parser;
use git2::{Commit, IndexAddOption, Repository, ResetType, Sort};
use log::{debug, info};
use simple_logger::SimpleLogger;

fn main() -> Result<(), anyhow::Error> {
    SimpleLogger::new().init().expect("Could not initialize logger");
    
    let args = Args::parse();
    
    debug!("current folder: {}", &args.path);

    let repository = Repository::discover(args.path)?;
    let current_branch = repository.head()?;
    
    debug!("branch: {:?}", current_branch.shorthand());

    let number_of_commits_between = count_commits_between(&repository, &args.main, current_branch.shorthand().unwrap())?;
    
    debug!("Number of commits: {}", number_of_commits_between);

    let head_commit = repository.head()?.peel_to_commit()?;
    let target_commit = get_nth_parent(&head_commit, number_of_commits_between).unwrap();

    debug!("target commit: {}", target_commit.id());

    let parent_commit = get_nth_parent(&head_commit, number_of_commits_between - 1).unwrap();

    let commit_message = match args.message {
        None => {
            parent_commit.message().unwrap().to_string()
        }
        Some(msg) => {
            msg.to_string()
        }
    };
    
    debug!("commit message: {}", commit_message);

    if !args.dry {
        repository.reset(target_commit.as_object(), ResetType::Soft, None)?;

        let mut index = repository.index()?;
        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let branch_ref = current_branch.name().unwrap();

        let sig = repository.signature()?;
        let tree_id = index.write_tree()?;
        let tree = repository.find_tree(tree_id)?;
        repository.commit(Some(branch_ref), &sig, &sig, &commit_message, &tree, &[&target_commit])?;

        info!("Commit successful; Squashed {} with message \"{}\"", number_of_commits_between, commit_message);
    }
    
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

fn get_nth_parent<'a>(commit: &'a Commit, n: usize) -> Option<Commit<'a>> {
    let mut current_commit = commit.clone();
    for _ in 0..n {
        if let Ok(parent) = current_commit.parent(0) {
            current_commit = parent;
        } else {
            return None;
        }
    }
    Some(current_commit)
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
    #[arg(
        short = 'a',
        long,
        help = "Override the commit message of the squashed commit"
    )]
    message: Option<String>,
    #[arg(long, help = "Initiate dry run and not executing the squash")]
    dry: bool,
}
