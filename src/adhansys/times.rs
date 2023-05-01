use crate::adhansys::methods::{get_madhab, get_method};
use salah::prelude::*;
use std::process::exit;

pub struct TodayPrayerTimes {
    pub fajr: String,
    pub sunrise: String,
    pub dhuhr: String,
    pub asr: String,
    pub maghrib: String,
    pub isha: String,
    pub qiyam: String,
}

pub struct CurrentAndNextSalah {
    pub current: CurrentSalahDetails,
    pub next: String,
}

pub struct CurrentSalahDetails {
    pub name: String,
    // pub time_remaining: String,
    pub time_remaining: (u32, u32),
}

pub fn get_today_prayer_times() -> TodayPrayerTimes {
    let latitude = 23.7231;
    let longitude = 90.4086;
    let method = "Karachi";
    let madhab = "Hanafi";

    let city = Coordinates {
        latitude,
        longitude,
    };
    let date = Utc::today();
    let params = Configuration::with(get_method(method), get_madhab(madhab));
    let prayers = PrayerSchedule::new()
        .on(date)
        .for_location(city)
        .with_configuration(params)
        .calculate();

    match prayers {
        Ok(prayer) => TodayPrayerTimes {
            fajr: prayer
                .time(Prayer::Fajr)
                .with_timezone(&chrono::Local)
                .format("%-l:%M %p")
                .to_string(),
            sunrise: prayer
                .time(Prayer::Sunrise)
                .with_timezone(&chrono::Local)
                .format("%-l:%M %p")
                .to_string(),
            dhuhr: prayer
                .time(Prayer::Dhuhr)
                .with_timezone(&chrono::Local)
                .format("%-l:%M %p")
                .to_string(),
            asr: prayer
                .time(Prayer::Asr)
                .with_timezone(&chrono::Local)
                .format("%-l:%M %p")
                .to_string(),
            maghrib: prayer
                .time(Prayer::Maghrib)
                .with_timezone(&chrono::Local)
                .format("%-l:%M %p")
                .to_string(),
            isha: prayer
                .time(Prayer::Isha)
                .with_timezone(&chrono::Local)
                .format("%-l:%M %p")
                .to_string(),
            qiyam: prayer
                .time(Prayer::Qiyam)
                .with_timezone(&chrono::Local)
                .format("%-l:%M %p")
                .to_string(),
        },
        Err(error) => {
            println!("Could not calculate prayer times: {}", error);

            exit(1)
        }
    }
}

pub fn get_current_prayer_details() -> CurrentAndNextSalah {
    let latitude = 23.7231;
    let longitude = 90.4086;
    let method = "Karachi";
    let madhab = "Hanafi";

    let city = Coordinates::new(latitude, longitude);
    let date = Utc::today();
    let params = Configuration::with(get_method(method), get_madhab(madhab));
    let prayers = PrayerSchedule::new()
        .on(date)
        .for_location(city)
        .with_configuration(params)
        .calculate();

    match prayers {
        Ok(times) => {
            // let (hours, mins) = times.time_remaining();

            CurrentAndNextSalah {
                current: CurrentSalahDetails {
                    name: times.current().name(),
                    // time_remaining: hours.to_string() + ":" + &mins.to_string(),
                    time_remaining: times.time_remaining(),
                },
                next: times.next().name(),
            }
        }
        Err(_) => {
            eprint!("Error fetching PrayerTimes!");

            exit(1);
        }
    }
}
