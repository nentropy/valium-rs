use tokenizers::tokenizer::{Tokenizer, Encoding};
use rust_bert::pipelines::sentiment::SentimentModel;
use rust_bert::pipelines::sequence_classification::SequenceClassificationModel;
use std::env;

pub fn analyze_text(text: &str) -> Result<Encoding, Box<dyn std::error::Error>> {
    // Load a tokenizer (e.g., BERT tokenizer)
    let tokenizer = Tokenizer::from_pretrained("bert-base-uncased", None)?;
    
    // Encode the text input
    let encoding = tokenizer.encode(text, true)?;
    Ok(encoding)
}
// Sentiment and Toxicity
pub fn analyze_sentiment(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let model = SentimentModel::new(Default::default())?;
    
    // Predict sentiment on the text
    let sentiments = model.predict(&[text]);
    println!("Sentiment: {:?}", sentiments);

    Ok(())
}

//Prompt Injection
pub fn detect_prompt_injection(text: &str) -> bool {
    let prompt_patterns = vec![
        "ignore previous instructions",
        "pretend you are",
        "please execute",
        "run this code",
        "system: ",
    ];

    for pattern in prompt_patterns {
        if text.contains(pattern) {
            println!("Potential prompt injection detected: {}", pattern);
            return true;
        }
    }

    false
}

// If streaming enabled
pub fn run_analysis_live(text: &str) {
    if env::var("RUN_LIVE").unwrap_or("false".into()) == "true" {
        println!("Running live analysis...");
        analyze_text(text).unwrap();
        analyze_sentiment(text).unwrap();
    } else {
        println!("Running file-based analysis...");
    }
}

pub mod input_validator {
    use super::analyze_text;
    use super::detect_prompt_injection;

    pub fn validate_input(text: &str) {
        // Tokenization & basic analysis
        analyze_text(text).unwrap();
        
        // Detect prompt injection
        if detect_prompt_injection(text) {
            println!("Alert: Prompt Injection detected in input.");
        } else {
            println!("Input is safe.");
        }
    }
}

pub mod output_validator {
    use super::analyze_sentiment;
    use super::detect_prompt_injection;

    pub fn validate_output(text: &str) {
        // Sentiment & toxicity analysis
        analyze_sentiment(text).unwrap();
        
        // Detect prompt injection
        if detect_prompt_injection(text) {
            println!("Alert: Prompt Injection detected in output.");
        } else {
            println!("Output is safe.");
        }
    }
}

//Integrate DB
pub fn store_input_validation(db: &PgConnection, text: &str, prompt_injection: bool) {
    diesel::insert_into(input_validations::table)
        .values((
            input_validations::text.eq(text),
            input_validations::prompt_injection.eq(prompt_injection),
        ))
        .execute(db)
        .expect("Error saving input validation");
}