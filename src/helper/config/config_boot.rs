use crate::helper::run_command::run_cmd;

pub fn config_boot(mode: &str) {
	let target = if mode == "grf" {
		"graphical.target"
	}
	else {
		"multi-user.target"
	};
	run_cmd("sudo", &["systemctl", "set-default", target]);
}
