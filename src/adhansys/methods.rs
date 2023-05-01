use salah::prelude::*;

pub fn get_method(method: &str) -> Method {
    match method {
        "MWL" => Method::MuslimWorldLeague,
        "Egyptian" => Method::Egyptian,
        "Karachi" => Method::Karachi,
        "UmmAlQura" => Method::UmmAlQura,
        "Dubai" => Method::Dubai,
        "Qatar" => Method::Qatar,
        "Kuwait" => Method::Kuwait,
        "MoonsightingCommittee" => Method::MoonsightingCommittee,
        "Singapore" => Method::Singapore,
        "Turkey" => Method::Turkey,
        "Tehran" => Method::Tehran,
        "ISNA" => Method::NorthAmerica,
        "Other" => Method::Other,
        &_ => Method::Other,
    }
}

pub fn get_madhab(madhab: &str) -> Madhab {
    match madhab {
        "Hanafi" => Madhab::Hanafi,
        "Shafi" => Madhab::Shafi,
        &_ => Madhab::Hanafi,
    }
}
