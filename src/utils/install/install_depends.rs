use std::io::{self, Write}; // Für Terminal IO
use crate::helper::run_command::run_cmd;
use crate::helper::timer::timer;
use crate::helper::system::system_helper::refresh_system;
use crate::helper::config::config_desktop::config_desktop;
use crate::helper::config::config_boot::config_boot;

// Installiert die notwendigen Pakete und optionale Desktopumgebung
pub fn install_depends(with_desktop: bool) {
	refresh_system();
	let mut packages = vec!["openssh-client", "openssh-server", "rsyslog"];
	
	if with_desktop {
		packages.extend(["ubuntu-desktop", "gparted", "synaptic"]);
		config_desktop();
		config_boot("terminal");
	}
	
	let mut args = vec!["apt", "install"];
	args.extend(packages);
	run_cmd("sudo", &args);
	
	println!("[?] System jetzt neustarten? (j/n)");
	let mut ans = String::new();
	io::stdin().read_line(&mut ans).unwrap();
	if ans.trim().to_lowercase().unwrap() == "j" {
		println!("[INFO] System wird heruntergefahren...");
		timer(15);
		run_cmd("sudo", &["reboot"]);
	}
	println!("[INFO] Führen Sie sobald wie möglich einen Neustart durch.");
}   
