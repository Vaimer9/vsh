/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/
use std::path::Path;

use git2::{Branch, Repository};
use std::env;

pub struct Git {
    repo: Option<Repository>,
}

pub struct BranchInfo {
    name: String,
    files_changed: usize,
}

impl BranchInfo {
    pub fn new(name: String, files_changed: usize) -> Self {
        Self {
            name,
            files_changed,
        }
    }

    /// Get a reference to the branch info's name.
    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get the branch info's files changed.
    pub fn get_files_changed(&self) -> usize {
        self.files_changed
    }
}

impl Git {
    /// Create a git instance on current directory
    pub fn new(repo: Option<Repository>) -> Self {
        Self { repo }
    }

    /// Check if the current dir(from env) is a git repository
    /// It also checks recursively if any parent directories are a git repository
    pub fn from_current_dir() -> Self {
        let path = env::current_dir().unwrap();
        Self {
            repo: Self::from_dir(&path),
        }
    }

    fn from_dir(path: &Path) -> Option<Repository> {
        let repo = Repository::open(path);
        match repo {
            Ok(r) => Some(r),
            Err(_) => {
                let parent = path.parent();
                match parent {
                    Some(x) => Self::from_dir(x),
                    None => None,
                }
            }
        }
    }

    ///Retrieve current branch by checking every branches.
    ///If one is the head then the name of that branch is returned.
    /// Also returns informations about deltas with the current dir
    ///Produce an error if there is no repository.
    pub fn get_current_branch_info(&self) -> Result<BranchInfo, ()> {
        let repo = self.repo.as_ref();
        if let Some(repo) = repo {
            let stats = repo
                .diff_index_to_workdir(None, None)
                .unwrap()
                .stats()
                .unwrap();
            for branch in repo.branches(None).unwrap() {
                let branch = branch.unwrap();
                if branch.0.is_head() {
                    return Ok(BranchInfo::new(
                        branch.0.name().unwrap().unwrap().to_string(),
                        stats.files_changed(),
                    ));
                }
            }
            return Err(());
        } else {
            Err(())
        }
    }
}
