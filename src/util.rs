use std::{
    env,
    path::{Path, PathBuf},
};

pub fn exe_parent_path() -> PathBuf {
    let exe_path: PathBuf = env::current_exe().unwrap();
    let exe_current_dir: &Path = exe_path.parent().unwrap_or(exe_path.as_path());
    PathBuf::from(exe_current_dir)
}

#[cfg(test)]
mod tests {
    use crate::util::exe_parent_path;

    #[test]
    fn test_exe_parent_path_returns_value() {
        exe_parent_path();
    }
}
