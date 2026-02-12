use std::error::Error;
use crate::helper::write_file::write_file;
use crate::helper::system::server_ip::server_ip;

pub fn config_client() {
	let content = r#"# /etc/rsyslog.conf configuration file for rsyslog
#
# For more information install rsyslog-doc and see
# /usr/share/doc/rsyslog-doc/html/configuration/index.html
#
# Default logging rules can be found in /etc/rsyslog.d/50-default.conf

#################
#### MODULES ####
#################
module(load="imuxsock") # provides support for local system logging
#module(load="immark")  # provides --MARK-- message capability

# provides UDP syslog reception
#module(load="imudp")
#input(type="imudp" port="514")

# provides TCP syslog reception
#module(load="imtcp")
#input(type="imtcp" port="514")

# provides kernel logging support and enable non-kernel klog messages
module(load="imklog" permitnonkernelfacility="on")

###########################
#### GLOBAL DIRECTIVES ####
###########################

# Filter duplicated messages
$RepeatedMsgReduction on

#
# Set the default permissions for all log files.
#
$FileOwner syslog
$FileGroup adm
$FileCreateMode 0640
$DirCreateMode 0755
$Umask 0022
$PrivDropToUser syslog
$PrivDropToGroup syslog

#
# Where to place spool and state files
#
$WorkDirectory /var/spool/rsyslog

#
# Include all config files in /etc/rsyslog.d/
#
$IncludeConfig /etc/rsyslog.d/*.conf"#.to_string();

	match server_ip() {
		Ok(ip) => {
			content.push_str("\n*.* @@{}:514", ip);
			let create_file = write_file("rsyslog.conf", content, &["client-config"]);
			match create_file {
				Ok(p) => println!("[OK] Datei erstellt unter: {:?}", p),
				Err(e) => eprintln!("[ERROR] Fehler: {}", e),
			}		
			println!("{}", content);			
		}
		Err(e) => {
			eprintln!("[ERROR] Ein Fehler ist aufgetreten und wir sind uns nicht sicher wo dieser liegt.");
			std::process::exit(1);
		}
	}
}
