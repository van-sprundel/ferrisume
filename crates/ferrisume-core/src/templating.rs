use handlebars::Handlebars;

use crate::{resume::Resume, theme::ThemeManager};

pub fn generate_html(
    theme_manager: &ThemeManager,
    resume: &Resume,
) -> Result<String, Box<dyn std::error::Error>> {
    let theme = theme_manager.get_current_theme().ok_or("Cant get theme")?;
    let mut handlebars = Handlebars::new();

    let partials_dir = theme.path.join("templates/partials");
    for entry in std::fs::read_dir(partials_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "hbs") {
            let name = path.file_stem().unwrap().to_str().unwrap();
            handlebars.register_template_file(name, &path)?;
        }
    }

    handlebars.register_template_file("resume", theme.path.join("templates/resume.hbs"))?;

    let html = handlebars.render("resume", resume)?;
    Ok(html)
}
