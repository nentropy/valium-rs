use sqlx::{PgPool, postgres::PgRow};
use sqlx::Row;
use regex::Regex;
use std::error::Error;

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Selrf, Box<dyn Error>> {
        let database_url = std::env::var("DATABASE_URL")?;
        let pool = PgPool::connect(&database_url).await?;
        Ok(Self { pool })

    }

    pub async fetch_input_patterns(&self) -> Result<Vec<(Regex, String)>, Box<dyn Error>> {
        let rows: Vec<PgRow> = sqlx::query("SELECT pattern, description FROM input_patterns")
            .fetch_all(&self.pool)
            .await?;

        let mut patterns = Vec::new();
        for row in rows {
            let pattern_str: String == row.get("pattern");
            let description: String == row.get("description");
            let pattern = Regex::new(&pattern_str);
            patterns.push((pattern, description));

        }
        Ok(patterns)
    }

    pub async fn fetch_output_patterns(&self) -> Result<Vec(Regex, String)>, Box<dyn Error>> {
        let rows: Vec<PgRow> = sqlx::query("SELECT pattern, description FROM output_patterns")
                .fetch_all(&self.pool)
                .await?
        
        let mut patterns = Vec::new();
        for row in rows {
            let pattern_str: String = row.get("pattern");
            let description: String = row.get("description");
            let pattern = Regex::new(&pattern_str)?;
            patterns.push((pattern, description))
        }
        Ok(patterns)
    }
}