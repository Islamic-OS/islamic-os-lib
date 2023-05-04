use crate::adhansys::patched_methods::QiblahPatched;
use salah::prelude::*;

use super::config::get_config;

pub struct QiblahData {
    pub degrees: f64,
}

pub fn get_qiblah_direction() -> QiblahData {
    let mut config = get_config();

    let latitude_u8 = config.get(b"location_latitude").unwrap();
    let longitude_u8 = config.get(b"location_longitude").unwrap();

    let latitude = String::from_utf8(latitude_u8.to_vec())
        .unwrap()
        .parse::<f64>()
        .unwrap();
    let longitude = String::from_utf8(longitude_u8.to_vec())
        .unwrap()
        .parse::<f64>()
        .unwrap();

    let city = Coordinates::new(latitude, longitude);

    let qiblah = QiblahPatched::new(city);

    QiblahData {
        degrees: qiblah.0.clone(),
    }
}
