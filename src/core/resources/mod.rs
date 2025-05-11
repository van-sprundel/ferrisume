use std::collections::HashMap;
use std::path::Path;

pub struct EmbeddedTheme {
    pub files: HashMap<&'static str, Vec<u8>>,
}

pub fn get_embedded_default_theme() -> EmbeddedTheme {
    let mut files = HashMap::new();

    // config file
    files.insert(
        "config.toml",
        include_bytes!("../../../themes/default/config.toml").to_vec(),
    );

    // templates
    files.insert(
        "templates/resume.hbs",
        include_bytes!("../../../themes/default/templates/resume.hbs").to_vec(),
    );
    files.insert(
        "templates/styles.css",
        include_bytes!("../../../themes/default/templates/styles.css").to_vec(),
    );

    // partials
    files.insert(
        "templates/partials/header.hbs",
        include_bytes!("../../../themes/default/templates/partials/header.hbs").to_vec(),
    );
    files.insert(
        "templates/partials/interests.hbs",
        include_bytes!("../../../themes/default/templates/partials/interests.hbs").to_vec(),
    );
    files.insert(
        "templates/partials/languages.hbs",
        include_bytes!("../../../themes/default/templates/partials/languages.hbs").to_vec(),
    );
    files.insert(
        "templates/partials/references.hbs",
        include_bytes!("../../../themes/default/templates/partials/references.hbs").to_vec(),
    );
    files.insert(
        "templates/partials/section.hbs",
        include_bytes!("../../../themes/default/templates/partials/section.hbs").to_vec(),
    );
    files.insert(
        "templates/partials/skills.hbs",
        include_bytes!("../../../themes/default/templates/partials/skills.hbs").to_vec(),
    );
    files.insert(
        "templates/partials/summary.hbs",
        include_bytes!("../../../themes/default/templates/partials/summary.hbs").to_vec(),
    );

    // fonts
    files.insert(
        "templates/fonts/lmroman10-bold.otf",
        include_bytes!("../../../themes/default/templates/fonts/lmroman10-bold.otf").to_vec(),
    );
    files.insert(
        "templates/fonts/lmroman10-italic.otf",
        include_bytes!("../../../themes/default/templates/fonts/lmroman10-italic.otf").to_vec(),
    );
    files.insert(
        "templates/fonts/lmroman10-regular.otf",
        include_bytes!("../../../themes/default/templates/fonts/lmroman10-regular.otf").to_vec(),
    );

    EmbeddedTheme { files }
}

pub fn extract_embedded_theme(theme: &EmbeddedTheme, destination: &Path) -> std::io::Result<()> {
    for (file_path, content) in &theme.files {
        let full_path = destination.join(file_path);

        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(full_path, content)?;
    }

    Ok(())
}
