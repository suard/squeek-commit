use clap::Parser;
use git2::{Commit, ResetType};

fn main() -> Result<(), git2::Error> {
    let args = Args::parse();

    println!("current folder: {}", &args.path);

    match git2::Repository::discover(args.path) {
        Ok(repository) => {
            let main_branch = repository.find_branch(&args.main, git2::BranchType::Local)?;
            let main_commit = main_branch.get().peel_to_commit()?;

            // println!("Latest commit main branch: {}", main_commit.id());

            let current_branch = repository.head()?;
            let current_commit = current_branch.target().unwrap();

            let mut revwalk = repository.revwalk().unwrap();
            revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;
            revwalk.push_range(&format!("{}..{}", main_commit.id(), current_commit)).unwrap();

            let first_commit = revwalk.last().unwrap().unwrap(); // here the number of commit
            let commit = repository.find_commit(first_commit)?;

            let commit_message = commit.message();
            println!("first commit message: {}", commit.message().unwrap());

            let mut revwalk2 = repository.revwalk()?;
            // revwalk.push_head()?;
            revwalk2.set_sorting(git2::Sort::TOPOLOGICAL)?;
            revwalk2.push_range(&format!("{}..{}", main_commit.id(), current_commit)).unwrap();


            let mut revwalk3 = repository.revwalk()?;
            // revwalk.push_head()?;
            revwalk3.set_sorting(git2::Sort::TOPOLOGICAL)?;
            revwalk3.push_range(&format!("{}..{}", main_commit.id(), current_commit)).unwrap();

            let n = revwalk3.count();
            println!("cnt: {}", n);

            let commit_id = revwalk2.flatten().nth(n - 1).ok_or_else(|| git2::Error::from_str("Niet genoeg commits"))?;
            let commit = repository.find_commit(commit_id)?;

            println!("commitId: {}", commit_id.to_string());

            repository.reset(commit.as_object(), ResetType::Soft, None)?;

            // **Nieuwe commit maken**
            let sig = repository.signature()?; // Huidige gebruiker als auteur
            let tree_id = repository.index()?.write_tree()?; // Huidige index als tree opslaan
            let tree = repository.find_tree(tree_id)?;

            // // Commit maken met de nieuwe state
            // repository.commit(Some("HEAD~2"), &sig, &sig, commit_message.unwrap(), &tree, &[&commit])?;
            repository.commit(None, &sig, &sig, commit_message.unwrap(), &tree, &[&commit])?;
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
