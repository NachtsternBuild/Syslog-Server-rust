use crate::helper::run_command::run_cmd;

// updates/upgrades
pub fn refresh_system() {
	run_cmd("sudo", &["apt", "update"]);
	run_cmd("sudo", &["apt", "upgrade"]);
	run_cmd("sudo", &["snap", "refresh"]);
}

// aufräumen
pub fn cleanup() {
	run_cmd("sudo", &["apt", "autoremove"]);
	run_cmd("sudo", &["apt", "autoclean"]);
}

// status systemtools
pub fn status_syslog_tools() {
	run_cmd("systemctl", &["status", "ssh", "--no-pager"]);
	run_cmd("systemctl", &["status", "rsyslog", "--no-pager"]);
}
