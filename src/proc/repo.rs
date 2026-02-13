use std::path::{Path, PathBuf};

pub fn resolve_repo_root(start: &Path) -> Option<PathBuf> {
    let mut current = Some(start);
    while let Some(path) = current {
        if path.join(".git").exists() || path.join(".jcn-root").exists() {
            return Some(path.to_path_buf());
        }
        current = path.parent();
    }
    None
}
