use std::{
    env,
    path::{Path, PathBuf},
};

pub fn exe_parent_path() -> PathBuf {
    match env::current_exe() {
        Ok(exe_path) => {
            let exe_current_dir: &Path = exe_path.parent().unwrap_or(exe_path.as_path());
            PathBuf::from(exe_current_dir)
        }
        Err(e) => panic!(
            "Unable to access directory where game was started.\n Error: {}",
            e
        ),
    }
}
