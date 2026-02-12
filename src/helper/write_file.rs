use directories::UserDirs;
use std::fs;
use std::path::PathBuf;

pub fn write_file(filename: &str, content: &str, path_list: &[&str]) -> std::io::Result<()> {
	let user_dirs = UserDirs::new()
		.ok_or_else(|| std::io::ErrorKind::NotFound)?;
	
	let mut full_path = PathBuf::from(user_dirs.home_dir());
	
	for direc in path_list {
		full_path.push(direc);
	}
	full_path.push(filename);
	
	if let Some(parent) = full_path.parent() {
		fs::create_dir_all(parent)?;
	}
	
	fs::write(&full_path, content)?;
	
	Ok(())
}
