use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct AccordionValue {
    pub icon: String,
    pub title: String
}

#[derive(Deserialize, Debug)]
pub struct AccordionElement {
    pub name: String,
    pub values: Vec<AccordionValue>
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ProjectElement {
    pub id: i32,
    pub images: Vec<String>,
    pub name: String,
    pub description: String
}