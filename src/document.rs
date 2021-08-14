//-
//- The process of customizing the documents is as follows:
//-  - Start with a common template
//-  - Address requirements
//-  - Address nice-to-haves in motivational letter
//-  - Address what's prominent about company in motivational letter
//-  - Write something nice about yourself in the letter
//-  - Choose relevant pieces of work history (market appropriate)
//-  - Pick some soft-skill/personal accomplishment or two
//-  - Choose photo if any
//-  - Apply a generic footer or copy one requested in offer


///
/// I don't know about actors, but a workforce application consists of:
///  - An offer to respond to, it's requirements and nice-to-haves and info about the company behind it
///  - Personal info
///  - Contact info
///  - Select work history
///  - Relevant select work history highlights
///  - Accomplishments
///  - Appropriate footer
///  - A generic nice motivational letter template
///  - Highlights of my relevancy 
///

use crate::dataobjects::*;

#[derive(Debug)]
struct PastWorkFilter<'a> {
    past_work: &'a PastWork,
    select_bits: Vec<String>
}

#[derive(Default, Debug)]
struct Resume<'a> {
    personal_info: PersonalInfo,
    contact_info: ContactInfo,
    work_history: Vec<&'a PastWorkFilter<'a>>,
    language_info: Vec<LanguageInfo>,
    skills: Vec<String>,
    footer: Option<String>
}

#[derive(Debug, Clone)]
struct MLetter<'a> {
    resume: &'a Resume<'a>,
    template_name: String,
    recipient: String,
    why_me: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::{data, document::PastWorkFilter};

    #[test]
    fn construct() -> () {
        let past_work = super::PastWorkFilter{past_work: &data::past_work_lufthansa(), select_bits: vec![]};
        let past_work: &PastWorkFilter = Box::leak(Box::new(past_work));

        let work_history = vec![past_work];

        let mut resume = super::Resume {
            personal_info: data::personal_info_basic(),
            contact_info: data::contact_info_variants()[0].clone(),
            work_history: work_history,
            language_info: data::language_info(),
            skills: vec!["Rozwiązywanie problemów ponad 10 zespołów programistów".into()],
            footer: Some("RODO stopka".into()),
        };

        let mletter = super::MLetter {
            resume: &resume,
            template_name: "list1".into(),
            recipient: "Wirtualna Polska".into(),
            why_me: vec!["potrzebuję i wyszukuję wyzwania".into(), "potrzebuję pieniędzy".into()]
        };

    }
}