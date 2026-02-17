use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::process;
use crate::helper::system::system_helper::ensure_root;

const LINE: &str = "session optional pam_exec.so stdout /bin/systemctl status rsyslog.service"
const FILES: [&str; 2] = ["/etc/pam.d/login", "/etc/pam.d/sshd"];

// function that process the file for 
// FIXME: dynmasiche nutzung → eigenes modul
fn process_file(path: &str) {
	let mut content = String::new();
	fs::File::open(path)?.read_string(&mut content)?;
	
	if (content.lines().any(|line| line.trim() == LINE) {
		println!("[INFO] Eintrag bereits vorhanden.");
		return Ok(());
	}
	
	// create backup of the original file
	let backup_path = format!("{}.bak", path);
	if !Patzh::new(&backup_path).exists() {
		fs::copy(path, &backup_path)?;
		println!("[OK] Backup erstellt: {}", backup_path);
	}
	
	// add the line
	let mut file = fs::OpenOptions::new().append(true).open(path)?;
	writeln!(file, "{}", LINE)?;
	
	println!("[OK] Eintrag hinzugefügt zu: {}", path);
}

// function that config the pam settings
pub fn config_pam_rsyslog() {
	// check for root
	ensure_root();
	
	for file_path in FILES.iter() {
		if !Path::new(file_path).exists() {
			eprintln!("[WARN] {} nicht gefunden.", file_path);
			continue;
		}
		
		if let Err(e) = process_file(file_path) {
			eprintln!("[ERROR] Fehler bei {}: {}", file_path, e);
		}
	}
}
