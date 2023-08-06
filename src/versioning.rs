use git2::{ObjectType, Repository, Signature};

pub fn backup_path(path: &str, git_config: (&str, &str)) {
    let repo = match Repository::init(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to initialize repository: {}", e),
    };

    match repo.index() {
        Ok(mut index) => {
            index
                .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
                .expect("Failed to add files to index");

            index.write().expect("Failed to write index");
        }
        Err(e) => panic!("failed to open index: {}", e),
    }

    let signature = Signature::now(git_config.0, git_config.1).expect("Failed to create signature");

    let tree_id = match repo.index().unwrap().write_tree() {
        Ok(tree_id) => tree_id,
        Err(e) => panic!("failed to write tree: {}", e),
    };

    let tree = match repo.find_tree(tree_id) {
        Ok(tree) => tree,
        Err(e) => panic!("failed to find tree: {}", e),
    };

    let current_time = chrono::Local::now();
    let commit_message = format!("Backup at {}", current_time.format("%Y-%m-%d %H:%M:%S"));

    match repo.head() {
        Ok(head) => {
            let parent_commit = match head.peel(ObjectType::Commit) {
                Ok(commit) => commit.into_commit().unwrap(),
                Err(_) => return,
            };

            let current_commit_id = match repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                &commit_message,
                &tree,
                &[&parent_commit],
            ) {
                Ok(commit) => commit,
                Err(e) => panic!("failed to create commit: {}", e),
            };

            repo.find_commit(current_commit_id).unwrap();

            println!("Backup successful!");
        }
        Err(_) => {
            let _ = repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                &commit_message,
                &tree,
                &[],
            );
        }
    };
}
