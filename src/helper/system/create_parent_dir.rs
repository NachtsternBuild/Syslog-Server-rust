use std::path::Path;
use std::fs;
use crate::helper::run_command::run_cmd;

// Funktion falls der Eltwernpfad nicht exsistiert diesen zu erstellen
pub fn create_parent_dir(path: &str) {
	let target_path = Path::new(path);
	// elternpfad erstellen falls dieser nicht exsitiert
	if let Some(parent) = target_path.parent() {
		if !parent.exists() {
			if !run_cmd("sudo", &["mkdir", "-p", parent.to_str().unwrap()]) {
				return Err(format!("[ERROR] Konnte Verzeichnis {:?} nicht erstellen!", parent));
			}
		}
	}
}
