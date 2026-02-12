use std::path::Path;
use std::fs;

// Funktion falls der Eltwernpfad nicht exsistiert diesen zu erstellen
pub fn create_parent_dir(path: &str) {
	let target_path = Path::new(path);
	// elternpfad erstellen falls dieser nicht exsitiert
	if let Some(parent) = target_path.parent() {
		if !parent.exists() {
			fs::create_dir_all(parent)?;
		}
	}
}
