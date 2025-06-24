use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormType {
    InputArea,
    InputText,
    InputDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub label: String,
    pub hint: String,
    pub form_type: FormType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub content: Field,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Footer {
    pub copyright: Field,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Block {
    Header(Header),
    Footer(Footer),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<Thing>,
    pub title: Field,
    pub blocks: Vec<Block>,
}

pub fn default_page_schema() -> Vec<Block> {
    vec![
        Block::Header(Header {
            content: Field {
                label: "Header".to_string(),
                hint: "".to_string(),
                form_type: FormType::InputArea,
            },
        }),
        Block::Footer(Footer {
            copyright: Field {
                label: "Copyright".to_string(),
                hint: "".to_string(),
                form_type: FormType::InputText,
            },
        }),
    ]
}

