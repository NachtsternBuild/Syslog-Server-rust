use std::fs; // Für Dateisystem-Operationen 
use std::os::unix::fs::PermissionsExt; // Unix spezifisch (chmod)

pub fn config_cleanup_log_files() {
	let script_content = r#"!/bin/bash
# Skript um alte Log Dateien zu löschen
# Funktion um alte Logs zu löschen
find_old_log() {
	DIRECTORY=$1
	# Verzeicchnis exsistiert ?
	if [ ! -d \"$DIRECTORY\" ]; then
		echo \"[ERROR] Directory $DIRECTORY not found!\"
		exit 1
	}
	# Lösche alle Log-Dateien älter 90 Tage
	find \"$DIRECTORY\" -type f -name \"*.log\" -mtime +990 -exec rm -rf {} \;
}

# Main
# Datei definieren wo alle zu prüfende Verzeichnisse darin stehen
datei=\"/usr/local/share/logging/directory.conf\"
# schleife für alle dateien
while IFS= read -r element; do
	find_old_log $element
done < \"$datei\"
echo \"[INFO] All log files in $datei that are older than 90 days habe been deleted!\""#;
	let path = "/usr/local/bin/cleanup-log-files"
	// Skript erstellen
	script_permission(path, script_content);
	
	println!("\n\n[?] Pfad für neue directory.conf: ");
	let mut path_dir = String::new();
	io::stdin().read_line(&mut path_dir).unwrap();
	let path_dir = path_dir.trim();
	let target_path = "/usr/local/share/logging/directory.conf"
	
	// Datei an die Stelle verschieben
	move_file(path_dir, target_path);
}

// TODO
pub fn config_cleanup_log() {

}
