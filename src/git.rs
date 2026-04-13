use git2::{BranchType, Oid};

pub struct GitClient {
    repository: git2::Repository,
    path: String,
}

impl GitClient {
    pub fn new(path: &str) -> Self {
        GitClient {
            repository: git2::Repository::open(path).unwrap(),
            path: path.to_string(),
        }
    }

    // TODO: If current branch is main, return error. Else, take previous commit
    pub fn get_diff_with_main(&self) -> anyhow::Result<String> {
        let remote = self.repository.find_remote("origin")?;
        let head = self.repository.head()?;
        let default_branch_name = remote.default_branch()?.as_str().unwrap().to_string();

        if head.shorthand().unwrap().to_string() == default_branch_name {
            anyhow::bail!("Current branch is the default branch. Same-branch review is not yet supported.");
        }

        let local_tree = head.peel_to_commit()?.tree()?;

        let default_branch = self.repository.find_branch(default_branch_name.as_str(), BranchType::Local)?;
        let default_tree = default_branch.get().peel_to_commit()?.tree()?;

        let diff = self.repository.diff_tree_to_tree(Some(&default_tree), Some(&local_tree), None)?;

        let mut diff_aggr = String::new();

        let _ = diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
            let origin = line.origin();
            let content = std::str::from_utf8(line.content()).unwrap_or("failure");

            diff_aggr.push_str(&match origin {
                '+' => format!("+ {}", content),
                '-' => format!("- {}", content),
                _ => format!(" {}", content),
            });
            true
        });

        Ok(diff_aggr)
    }
}
