use std::io::{self, Write}; // Für Terminal IO

mod helper;
mod utils;

use crate::{
	helper::{
		run_command::run_cmd,
		timer::timer,
		config::config_client::config_client,
		system::{
			basic_commands::basic_commands,
			system_helper::{
				refresh_system,
				cleanup,
				status_syslog_tools,
			},	
		},
	},
	utils::{
		menu::{
			config_server::config_server,
			desktop_install_menu::desktop_install_menu,
			get_rsyslog_config::get_rsyslog_config,
			firewall_menu::firewall_menu,
			change_boot_menu::change_boot_menu,
			add_log_tools::add_log_tools,
		},
	},
};


// TODO: sudo weglassen → Programm muss mit root-rechten gestartet werden
// TODO: Kommentare hinzufügen	
fn main() {
	loop {
		println!("\nWas soll gemacht werden?");
		println!("-------------------------------------");
        println!("(k) Server konfigurieren"); 
        println!("(l) Client Konfiguration ausgeben"); 
        println!("-------------------------------------");
		println!("(u) Updates und Upgrades");
        println!("(c) Nach Updates Aufräumen");
        println!("(n) Neustarten");
        println!("(a) Abmelden");
        println!("(i) Kommandoübersicht"); 
        println!("-------------------------------------");
        println!("(d) Desktop hinzu installieren");
        println!("(t) Zusätzliche Log Tools"); 
        println!("(r) Neue Rsyslog Config nutzen");
        println!("(f) Firewall-Modus ändern");
        println!("(s) Status von System Tools ausgeben");
        println!("(b) Boot-Modus ändern");
        println!("-------------------------------------");
        println!("(v) Verlassen/Beenden");

        print!("\nEingabe: ");
        io::stdout().flush().unwrap(); // "Eingabe:" sofort anzeigen
        
        let mut input = String::new(); // String für input
        io::stdin().read_line(&mut input).unwrap(); // liest Zeile von stdin
        let choice = input.trim().to_lowercase(); // entfernt Leerzeoichen/Zeilenumbrüche
        
        // switch/case 
        match choice.as_str() {
        	"k" => config_server(),
        	"l" => config_client(), 
        	"u" => refresh_system(),
        	"c" => cleanup(),
        	"n" => {
        		timer(15);
        		run_cmd("sudo", &["reboot"]);
        	}
        	"a" => {
        		timer(15);
        		let args: &[&str] = &[]; // Expliziter Typ hilft dem Compiler
        		run_cmd("logout", args);
        	}
        	"i" => basic_commands(), 
        	"d" => desktop_install_menu(),
        	"t" => add_log_tools(), 
        	"r" => get_rsyslog_config(),
        	"f" => firewall_menu(),
        	"s" => status_syslog_tools(),
        	"b" => change_boot_menu(),
        	"v" => break, // schleife beenden
        	_ => {
        		println!("[ERROR] Unbekannte Eingabe!");
        	}     
        }
    }
}
