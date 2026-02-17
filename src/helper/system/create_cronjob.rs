use crate::helper::run_command::run_cmd;

// function that create a cronjob
// FIXME: wie in config_pam_rsyslog
pub fn create_cronjob(cron: &str, path: &str) {
	println!("1. Tippen Sie im Terminal '1' um nano zu öffnen");
	println!("2. Ergänzen Sie am Ende der Datei folgende Zeile:");
	println!("{} {}", cron, path);
	run_cmd("crontab", &["-e"]);
}
