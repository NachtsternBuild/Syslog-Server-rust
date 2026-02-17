use std::fs;
use std::io::{Read, Write};
use std::path::Path;

// TODO

pub fn add_line_file(create_backup: &bool, path: &str, line: &str) -> std::io::Result<()> {
	if !Path::new().exists() {
		return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{} nicht gefunden", path),
        ));
	}
	
	let mut content = String::new();
	fs::File::open(path)?.read_to_string(&mut content)?;
	
	// check if the content is already in the file
	if (content.lines().any(|l| l.trim() == line) {
		println!("[OK] Eintrag bereits vorhanden in {}", path);
		return Ok(());
	}
	
	// backup of the original file
	if create_backup {
		let backup_path = format!("{}.bak", path);
        if !Path::new(&backup_path).exists() {
            fs::copy(path, &backup_path)?;
            println!("Backup erstellt: {}", backup_path);
        }
    }

    let mut file = fs::OpenOptions::new().append(true).open(path)?;
    writeln!(file, "{}", line)?;

    println!("[OK] Eintrag hinzugefügt zu {}", path);

    Ok(())
}
