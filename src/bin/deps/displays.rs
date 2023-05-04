use colored::Colorize;
use islamic_os_lib::adhansys::{
    qiblah::get_qiblah_direction,
    times::{get_current_prayer_details, get_today_prayer_times},
};
use serde_json::{Value};

use super::config::{create_new_config, is_config_available};

pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

pub fn prompt_for_config() {
    let config = is_config_available();

    if !config {
        println!(
            "{}",
            "It seems like you haven't configured IslamicOS Services Manager yet. Let me handle that for you..."
                .blue()
                .bold()
        );

        create_new_config();
        println!();
        println!(
            "{} {} {}",
            "All Good, Alhamdulillah! It's now highly recommended that you configure the program before using it. Please go to the"
                .blue()
                .bold(),
            "Settings".black().on_cyan(),
            "menu from this CLI and configure the program as your needs."
                .blue()
                .bold()
        );

        println!();

        println!("{}", "Enjoy! As-salamu alaykum.".green().bold());

        let _ = inquire::Text::new(
            format!("{}", "Press any key to continue...".yellow().bold()).as_str(),
        )
        .prompt();
    }
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

pub async fn get_location_from_ip() -> Result<Location, Box<dyn std::error::Error>> {
    let res = reqwest::get("https://ipwho.is/").await.unwrap();
    let json_str = res.text().await.unwrap_or_else(|e| {
        println!("Error: {}", e);
        std::process::exit(1);
    });

    let json_parsed: Value = serde_json::from_str(&json_str).unwrap_or_else(|e| {
        println!("Error: {}", e);
        std::process::exit(1);
    });

    let json_obj = json_parsed.as_object().unwrap();

    // println!(
    //     "Latitude: {}, Longitude: {}",
    //     json_obj.get("latitude").unwrap(),
    //     json_obj.get("longitude").unwrap()
    // );

    Ok(Location {
        latitude: json_obj.get("latitude").unwrap().as_f64().unwrap(),
        longitude: json_obj.get("longitude").unwrap().as_f64().unwrap(),
    })
}
