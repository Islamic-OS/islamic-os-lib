use crate::adhansys::patched_methods::QiblahPatched;
use salah::prelude::*;

pub struct QiblahData {
    pub degrees: f64,
}

pub fn get_qiblah_direction() -> QiblahData {
    let latitude = 23.7231;
    let longitude = 90.4086;

    let city = Coordinates::new(latitude, longitude);

    let qiblah = QiblahPatched::new(city);

    QiblahData {
        degrees: qiblah.0.clone(),
    }
}
