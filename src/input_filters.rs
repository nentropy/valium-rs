use regex::Regex;
use async_std::task;
use std::error::Error;
use log::*;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use dotenv::dotenv;
use std::env;
use crate::db::Database;
use std::sync::mpsc;
use std::thread;


/// # Validate Input
/// Struct to define an input filter
#[derive(Debug, Clone)]
pub struct InputFilter {
    pub name: String,
    pub pattern: String,
    pub description: String,
}

pub async fn validate_input_concurrently(input:&str, filters: Vec<InputFilter>, db: &Database) -> Vec<String> {
    let (tx, rx) = mpsc::channel();
    let patterns = db.fetch_input_patterns().await?;

    filters.into_par_iter().for_each_with(tx |s, filter| {
        let re = Regex::new(&filter.pattern).unwrap();
        if re.is_match(input) {
            s.send(format!("{} triggered", filter.name, filter.description)).unwrap()
        }
    });

    rx.iter().collect()

}

/// If environment variable is set to run live, validate streaming data.
pub fn live_input_validation(stream: &str) {
    if let Ok(run_live) = env::var("RUN_LIVE") {
        if run_live == "True" {
            // Run the stream validation logic
            println!("Running live input validation for stream: {}", stream);
            // Use async stream processing if needed.
        }
    }
}



 