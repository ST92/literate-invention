#[derive(Default, Debug, Clone)]
pub(crate) struct PastWork {
    pub employer: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub position: String,
    pub bits: std::collections::HashMap<String, String>
}

#[derive(Default, Debug)]
pub(crate) struct PersonalInfo {
    pub name: String,
    pub born: u32,
    pub photo: Option<std::path::PathBuf>
}

#[derive(Default, Debug, Clone)]
pub(crate) struct ContactInfo {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub notes: Option<String>
}

#[derive(Default, Debug, Clone)]
pub(crate) struct LanguageInfo {
    pub language_name: String,
    pub level: String
}