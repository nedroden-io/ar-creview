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
        let tree = self.repository.find_tree(Oid::from_str("main")?)?;
        let diff = self.repository.diff_tree_to_workdir(Some(&tree), None)?;

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
