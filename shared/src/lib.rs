use tera::Tera;

mod id_string;
mod error;

pub use id_string::{IdToString, impl_id_to_string};
pub use error::*;

pub fn add_templates(tera: &mut Tera) {
    tera.add_template_files(vec![
        ("./shared/templates/macros/forms.html", Some("macros/forms.html")),
        ("./shared/templates/macros/components.html", Some("macros/components.html")),
        ("./shared/templates/macros/icons.html", Some("macros/icons.html")),
    ]).expect("Failed to load shared macros");
}
