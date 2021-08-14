use std::{collections::HashMap, path::PathBuf, str::FromStr};

use crate::dataobjects::*;


pub(crate) fn personal_info_basic() -> PersonalInfo {
    PersonalInfo{ name: String::from("Marek Spociński"), born: 1992, photo: None}
}

pub(crate) fn personal_info_with_photo() -> PersonalInfo {
    PersonalInfo{ name: String::from("Marek Spociński"), born: 1992, photo: PathBuf::from_str("photo.jpeg").ok()}
}

pub(crate) fn contact_info_variants() -> Vec<ContactInfo> {
    vec![
        ContactInfo {email: Some("mareksp.92@gmail.com".into()), phone: Some("530945755".into()), notes: None},
        ContactInfo {email: Some("marsdprogrammer@gmail.com".into()), phone: Some("530945755".into()), notes: None},
        ContactInfo {email: Some("mareksp.92@gmail.com".into()), phone: None, notes: None},
    ]
}

pub(crate) fn language_info() -> Vec<LanguageInfo> {
    vec![
        LanguageInfo {language_name: "angielski".into(), level: "C1".into()}
    ]
}


pub(crate) fn past_work_lufthansa() -> PastWork {
    PastWork {
        employer: Some("Lufthansa Systems".into()),
        start: Some("04-2017".into()),
        end: Some("04-2018".into()),
        position: "Junior Software Engineer".into(),
        bits: HashMap::new(),
    }
}