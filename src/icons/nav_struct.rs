use yew::Properties;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub class_name: String,
    #[prop_or("20px".to_string())]
    pub width: String,
    #[prop_or("20px".to_string())]
    pub height: String,
    #[prop_or_default]
    pub color: String,
}