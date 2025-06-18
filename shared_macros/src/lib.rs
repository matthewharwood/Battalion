use tera::Tera;

pub fn add_templates(tera: &mut Tera) {
    tera.add_template_files(vec![
        ("./shared_macros/templates/macros/forms.html", Some("macros/forms.html")),
    ]).expect("Failed to load shared macros");
}
