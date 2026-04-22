use serde::Serialize;

use crate::error::Result;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Toon,
}

pub fn emit<T: Serialize>(value: &T, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(value)?);
            Ok(())
        }
        OutputFormat::Toon => {
            println!("{}", serde_toon::to_string(value)?);
            Ok(())
        }
    }
}
