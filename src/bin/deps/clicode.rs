use colored::Colorize;
use inquire::{Select, Text};

use super::{
    config::{display_current_location, update_config_value},
    displays::{display_qiblah_direction, display_salah_details, prompt_for_config, get_location_from_ip},
};

#[derive(PartialEq, Clone, Copy)]
pub enum Routes {
    Home,
    SalahDetails,
    Settings,
    SettingsLocationSettings,
    SettingsSalahSettings,
    SettingsSalahMadhabSettings
}

pub struct CLIChores {
    history_stack: Vec<Routes>,
    curr_route: Routes,
    curr_executable: fn(),
    // output_list: Vec<fn()>,
}

impl CLIChores {
    pub fn new(history_stack: Vec<Routes>, curr_route: &mut Routes) -> CLIChores {
        CLIChores {
            history_stack,
            curr_route: *curr_route,
            curr_executable: || {},
            // output_list: Vec::new(),
        }
    }

    pub fn greet(&self) {
        clearscreen::clear().expect("failed to clear screen");

        println!("{}\n", "IslamicOS Multi-purpose CLI".green().bold());

        let art_lines = vec![
            r" ___  ________  ___       ________  _____ ______   ___  ________  ________  ________      ",
            r"|\  \|\   ____\|\  \     |\   __  \|\   _ \  _   \|\  \|\   ____\|\   __  \|\   ____\     ",
            r"\ \  \ \  \___|\ \  \    \ \  \|\  \ \  \\\__\ \  \ \  \ \  \___|\ \  \|\  \ \  \___|_    ",
            r" \ \  \ \_____  \ \  \    \ \   __  \ \  \\|__| \  \ \  \ \  \    \ \  \\\  \ \_____  \   ",
            r"  \ \  \|____|\  \ \  \____\ \  \ \  \ \  \    \ \  \ \  \ \  \____\ \  \\\  \|____|\  \  ",
            r"   \ \__\____\_\  \ \_______\ \__\ \__\ \__\    \ \__\ \__\ \_______\ \_______\____\_\  \ ",
            r"    \|__|\_________\|_______|\|__|\|__|\|__|     \|__|\|__|\|_______|\|_______|\_________\",
            r"        \|_________|                                                          \|_________|",
        ];

        for line in art_lines {
            println!("{}", line.cyan().bold());
        }

        // if self.output_list.len() > 0 {
        //     for output in &self.output_list {
        //         output();
        //         println!();
        //     }
        // }

        if self.curr_executable != || {} {
            println!();
            (self.curr_executable)();
            println!();
        }
    }

    pub async fn run(&mut self) {
        prompt_for_config();

        loop {
            self.greet();

            match self.curr_route {
                Routes::Home => self.route_home(),
                Routes::SalahDetails => self.route_salah_details(),
                Routes::Settings => self.route_settings(),
                Routes::SettingsSalahSettings => self.route_settings_salah_settings(),
                Routes::SettingsSalahMadhabSettings => self.route_settings_salah_madhab_settings(),
                Routes::SettingsLocationSettings => self.route_settings_location_settings().await,
            }
        }
    }

    fn navigate(&mut self, goto: &mut Routes) {
        self.history_stack.push(self.curr_route);
        self.curr_route = *goto;
    }

    fn back(&mut self) {
        self.curr_route = self.history_stack.pop().unwrap();
    }

    fn quit(&self) {
        println!("{}", "Bye!".blue().bold());

        std::process::exit(0)
    }

    // fn add_to_output_list(&mut self, executable: fn()) {
    //     self.output_list.push(executable);
    // }

    fn set_curr_executable(&mut self, executable: fn()) {
        self.curr_executable = executable;
    }

    fn route_home(&mut self) {
        let ans = Select::new(
            "How may I help you today?",
            vec!["Get Salah-related Information", "Settings", "Quit"],
        )
        .with_help_message("Please select your desired service.")
        .prompt();

        match ans {
            Ok("Get Salah-related Information") => self.navigate(&mut Routes::SalahDetails),
            Ok("Settings") => self.navigate(&mut Routes::Settings),
            Ok("Quit") => self.quit(),
            Ok(&_) => {}
            Err(e) => println!("Err: {}", e),
        }
    }

    fn route_salah_details(&mut self) {
        let ans = Select::new(
            "What information do you need related to Salah?",
            vec![
                "Show Today's Salah Schedule",
                "Show Qiblah Direction",
                "Back",
                "Quit to Home",
            ],
        )
        .with_help_message("Please select your desired service.")
        .prompt();

        match ans {
            Ok("Show Today's Salah Schedule") => self.set_curr_executable(display_salah_details),
            Ok("Show Qiblah Direction") => self.set_curr_executable(display_qiblah_direction),
            Ok("Back") => self.back(),
            Ok("Quit to Home") => self.navigate(&mut Routes::Home),
            Ok(&_) => {}
            Err(e) => println!("Err: {}", e),
        }
    }

    fn route_settings(&mut self) {
        let ans = Select::new(
            "What would you like to configure?",
            vec![
                "Location Settings",
                "Salah Configuration",
                "Back",
                "Quit to Home",
            ],
        )
        .with_help_message("Please select your desired service.")
        .prompt();

        match ans {
            Ok("Location Settings") => self.navigate(&mut Routes::SettingsLocationSettings),
            Ok("Salah Configuration") => self.navigate(&mut Routes::SettingsSalahSettings),
            Ok("Back") => self.back(),
            Ok("Quit to Home") => self.navigate(&mut Routes::Home),
            Ok(&_) => {}
            Err(e) => println!("Err: {}", e),
        }
    }

    async fn route_settings_location_settings(&mut self) {
        let ans = Select::new(
            "What would you like to configure?",
            vec![
                "Manually Enter Location",
                "Automatically Detect Location (Make sure you have an active internet connection, and you are not using a VPN)",
                "Back",
                "Quit to Home",
            ],
        )
        .with_help_message("Please select your desired service.")
        .prompt();

        match ans {
            Ok("Manually Enter Location") => {
                // Prompt for latitude and longitude
                let latitude = Text::new("Enter Latitude: ")
                    .with_help_message("Please enter the latitude of your location.")
                    .prompt();
                let longitude = Text::new("Enter Longitude: ")
                    .with_help_message("Please enter the longitude of your location.")
                    .prompt();

                update_config_value("location_latitude", latitude.unwrap().as_str());
                update_config_value("location_longitude", longitude.unwrap().as_str());

                self.set_curr_executable(display_current_location);
            }
            Ok("Automatically Detect Location (Make sure you have an active internet connection, and you are not using a VPN)") => {
                // Get location from IP address
                let location = get_location_from_ip().await.unwrap();

                

                update_config_value("location_latitude", format!("{:?}", location.latitude).as_str());
                update_config_value("location_longitude", format!("{:?}", location.longitude).as_str());

                self.set_curr_executable(display_current_location);
            }
            Ok("Back") => self.back(),
            Ok("Quit to Home") => self.navigate(&mut Routes::Home),
            Ok(&_) => {}
            Err(e) => println!("Err: {}", e),
        }
    }

    fn route_settings_salah_settings(&mut self) {
        let ans = Select::new(
            "What would you like to configure?",
            vec![
                "Madhab",
                "Calculation Method",
                "Back",
                "Quit to Home",
            ],
        )
        .with_help_message("Please select your desired service.")
        .prompt();

        match ans {
            Ok("Madhab") => self.navigate(&mut Routes::SettingsSalahMadhabSettings),
            Ok("Calculation Method") => todo!(),
            Ok("Back") => self.back(),
            Ok("Quit to Home") => self.navigate(&mut Routes::Home),
            Ok(&_) => {}
            Err(e) => println!("Err: {}", e),
        }
    }

    fn route_settings_salah_madhab_settings(&mut self) {
        let ans = Select::new(
            "Which madhab do you follow?",
            vec![
                "Hanafi",
                "Shafi",
                "Back",
                "Quit to Home",
            ],
        )
        .with_help_message("Please select your desired service.")
        .prompt();

        match ans {
            Ok("Hanafi") => update_config_value("salahcfg_madhab", "Hanafi"),
            Ok("Shafi") => update_config_value("salahcfg_madhab", "Shafi"),
            Ok("Back") => self.back(),
            Ok("Quit to Home") => self.navigate(&mut Routes::Home),
            Ok(&_) => {}
            Err(e) => println!("Err: {}", e),
        }
    }
}
