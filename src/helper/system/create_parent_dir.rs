use std::path::Path;
use std::fs;

// Funktion falls der Eltwernpfad nicht exsistiert diesen zu erstellen
pub fn create_parent_dir(path: &str) {
	let target_path = Path::new(path);
    
    if let Some(parent) = target_path.parent() {
        if !parent.exists() {
            // Das Programm stürzt mit deiner Nachricht ab, wenn ein Fehler auftritt
            fs::create_dir_all(parent).expect("[ERROR] Ordner konnte nicht erstellt werden");
        }
    }
}
