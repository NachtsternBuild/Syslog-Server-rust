use std::fs; // Für Dateisystem-Operationen 
use std::os::unix::fs::PermissionsExt; // Unix spezifisch (chmod)
use crate::helper::system::create_parent_dir::create_parent_dir;

pub fn script_permission(path: &str, script_content: &str) {
	// Elternpfad überprüfen
	create_parent_dir(path);
	
	match fs::write(path, script_content) {
		Ok(_) => {
			// setzen der Datei Berechtigungen auf 755 (chmod a+x)
			let mut perms = fs::metadata(path).unwrap().permissions();
			perms.set_mode(0o755);
			fs::set_permissions(path, perms).unwrap();
			println!("[OK] Desktop Skript erstellt.");
		}
		Err(e) => eprintln!("[ERROR] Schreiben fehlgeschlagen: {}", e),
	}
}
