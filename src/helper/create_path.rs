use directories::UserDirs;
use std::fs;
use std::path::PathBuf;

pub fn create_path(filename: &str, path_list: &[&str]) -> std::io::Result<PathBuf> {
	let user_dirs = UserDirs::new()
		.ok_or_else(|| std::io::ErrorKind::notFound, "[ERROR] Home Verzichnis nicht gefunden"))?;
	
	let mut full_path = PathBuf::from(user_dirs.home_dir());
	
	for direc in path_list {
		full_path.push(direc);
	}
	full_path.push(filename);
	
	if let Some(parent) = full_path.parent() {
		fs::create_dir_all(parent)?;
	}
	Ok(full_path)
}
