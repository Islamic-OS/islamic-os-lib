use std::env;

use super::{
    config::is_config_available,
    displays::{display_qiblah_direction, display_salah_details},
};

pub struct CLIChores {
    queries: Vec<String>,
    #[allow(dead_code)]
    flags: Vec<String>,
}

impl CLIChores {
    pub fn new() -> CLIChores {
        let mut args: Vec<String> = env::args().collect();
        args.remove(0);

        let mut queries: Vec<String> = Vec::new();
        let mut flags: Vec<String> = Vec::new();

        for arg in args {
            if arg.starts_with("--") {
                flags.push(arg);
            } else {
                queries.push(arg);
            }
        }

        CLIChores { queries, flags }
    }

    pub fn is_args_provided(&self) -> bool {
        self.queries.len() > 0
    }

    pub fn run(&mut self) {
        if !is_config_available() {
            panic!("Configuration not initialised yet! Please run only `islamic-cli` without any flags to automatically initialise it.")
        }

        let args = self.queries.clone();

        match args[0].as_str() {
            "salah" => {
                self.salah_mode(args);
            }
            _ => {
                println!("Invalid argument")
            }
        }
    }

    fn salah_mode(&self, next_args: Vec<String>) {
        let mut args = next_args.clone();
        args.remove(0);

        match args[0].as_str() {
            "schedule" => {
                display_salah_details();
            }
            "qiblah" => {
                display_qiblah_direction();
            }
            _ => {
                println!("Invalid argument")
            }
        }
    }
}
