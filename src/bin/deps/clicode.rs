use colored::Colorize;
use inquire::Select;
use islamic_os_lib::adhansys::{
    qiblah::get_qiblah_direction,
    times::{get_current_prayer_details, get_today_prayer_times},
};

#[derive(PartialEq, Clone, Copy)]
pub enum Routes {
    Home,
    SalahDetails,
    Settings,
}

pub fn display_salah_details() {
    let today_prayer_times = get_today_prayer_times();

    println!("{}", "Today's Salah Times".blue().bold());

    println!("{} {}", "Fajr:".black().on_cyan(), today_prayer_times.fajr);
    println!(
        "{} {}",
        "Dhuhr:".black().on_cyan(),
        today_prayer_times.dhuhr
    );
    println!("{} {}", "Asr:".black().on_cyan(), today_prayer_times.asr);
    println!(
        "{} {}",
        "Maghrib:".black().on_cyan(),
        today_prayer_times.maghrib
    );
    println!("{} {}", "Isha:".black().on_cyan(), today_prayer_times.isha);
    println!(
        "{} {}",
        "Qiyam:".black().on_cyan(),
        today_prayer_times.qiyam
    );

    println!();

    println!("{}", "Currently On-going and Next Salah".blue().bold());

    let current_salah = get_current_prayer_details();

    println!(
        "{} {}",
        "Current Salah:".black().on_cyan(),
        current_salah.current.name
    );

    // Display time remaining in this format: 1 hour and 30 minutes left
    let mut current_salah_time = String::new();

    if current_salah.current.time_remaining.0 > 0 {
        if current_salah.current.time_remaining.0 == 1 {
            current_salah_time
                .push_str(&format!("{} hour", current_salah.current.time_remaining.0));
        } else {
            current_salah_time
                .push_str(&format!("{} hours", current_salah.current.time_remaining.0));
        }
    }

    if current_salah.current.time_remaining.1 > 0 {
        if current_salah.current.time_remaining.0 > 0 {
            current_salah_time.push_str(" and ");
        }

        if current_salah.current.time_remaining.1 == 1 {
            current_salah_time.push_str(&format!(
                "{} minute",
                current_salah.current.time_remaining.1
            ));
        } else {
            current_salah_time.push_str(&format!(
                "{} minutes",
                current_salah.current.time_remaining.1
            ));
        }
    }

    if current_salah.current.time_remaining.0 == 0 && current_salah.current.time_remaining.1 == 0 {
        current_salah_time.push_str("Now");
    } else {
        current_salah_time.push_str(" left");
    }

    println!(
        "{} {}",
        "Time Remaining:".black().on_cyan(),
        current_salah_time
    );

    println!();

    println!("{} {}", "Next Salah:".black().on_cyan(), current_salah.next);
}

pub fn display_qiblah_direction() {
    let qibla = get_qiblah_direction();

    println!("{}", "Qiblah Direction".blue().bold());

    let mut direction = String::new();
    direction.push_str(&format!("{:.2}deg ", qibla.degrees));

    if qibla.degrees > 355.0 || qibla.degrees < 5.0 {
        direction.push_str(format!("{}", "North".blue().bold()).as_str());
    } else if qibla.degrees > 5.0 && qibla.degrees < 85.0 {
        direction.push_str(format!("{}", "North East".blue().bold()).as_str());
    } else if qibla.degrees > 85.0 && qibla.degrees < 95.0 {
        direction.push_str(format!("{}", "East".blue().bold()).as_str());
    } else if qibla.degrees > 95.0 && qibla.degrees < 175.0 {
        direction.push_str(format!("{}", "South East".blue().bold()).as_str());
    } else if qibla.degrees > 175.0 && qibla.degrees < 185.0 {
        direction.push_str(format!("{}", "South".blue().bold()).as_str());
    } else if qibla.degrees > 185.0 && qibla.degrees < 265.0 {
        direction.push_str(format!("{}", "South West".blue().bold()).as_str());
    } else if qibla.degrees > 265.0 && qibla.degrees < 275.0 {
        direction.push_str(format!("{}", "West".blue().bold()).as_str());
    } else if qibla.degrees > 275.0 && qibla.degrees < 355.0 {
        direction.push_str(format!("{}", "North West".blue().bold()).as_str());
    }

    println!("{} {}", "Direction Angle:".black().on_cyan(), direction);
    println!();
    println!(
        "{}",
        "[ Side Note: Use a compass to lookup the direction... ]"
            .black()
            .bold()
    );
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

    pub fn run(&mut self) {
        loop {
            self.greet();

            match self.curr_route {
                Routes::Home => self.route_home(),
                Routes::SalahDetails => self.route_salah_details(),
                Routes::Settings => todo!(),
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
}
