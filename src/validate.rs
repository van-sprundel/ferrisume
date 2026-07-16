use crate::domain::Resume;

const SCHEMA: &str = include_str!("core/resources/jsonresume.schema.json");

pub fn validate_command(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let contents =
        std::fs::read_to_string(input).map_err(|e| format!("Couldn't read {input}: {e}"))?;

    let resume: serde_json::Value =
        serde_json::from_str(&contents).map_err(|e| format!("{input} is not valid JSON: {e}"))?;

    let schema: serde_json::Value = serde_json::from_str(SCHEMA)?;
    let validator = jsonschema::validator_for(&schema)?;

    let errors: Vec<_> = validator.iter_errors(&resume).collect();
    if !errors.is_empty() {
        eprintln!("❌ {input} does not match the JSON Resume schema:\n");
        for error in &errors {
            let location = if error.instance_path().to_string().is_empty() {
                "(root)".to_string()
            } else {
                error.instance_path().to_string()
            };
            eprintln!("  {location}: {error}");
        }
        return Err(format!(
            "{} validation error{} found",
            errors.len(),
            if errors.len() == 1 { "" } else { "s" }
        )
        .into());
    }

    // The schema allows more than the renderer knows about; make sure the
    // file also deserializes into our resume model so export/watch will work.
    serde_json::from_str::<Resume>(&contents)
        .map_err(|e| format!("{input} matches the schema but couldn't be parsed: {e}"))?;

    println!("✅ {input} is a valid JSON Resume");
    Ok(())
}
