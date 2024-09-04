mod input_filters;
mod output_filters;
mod db;

use input_filters::*;
use output_filters::*;
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
struct ValidationError(String);

#[derive(Debug)]
struct InjectionError(String);


/// Asynchronously validates the input string to ensure it does not contain disallowed patterns.
/// 
/// # Arguments
/// 
///  `input` - A reference to the input string that needs to be validated.
/// 
/// # Returns
/// 
///  `Result<(), ValidationError>` - Represents the result of the validation process. Returns `Ok(())` if the input is valid, otherwise returns an error with a message indicating the disallowed token found.
async fn validate_input(input: &str) -> Result<(), ValidationError> {
    let disallowed_patterns = vec!["DROP TABLE", "DELETE", "UNION SELECT"];
    for pattern in disallowed_patterns {
        if input.contains(pattern) {
            return Err(ValidationError("Disallowed token found.".to_string()));
        }
    }
    Ok(())
}


/// Asynchronously validates the output string for sensitive data patterns.
/// 
/// # Arguments
/// 
///  `output` - A string slice containing the output to be validated.
/// 
/// # Returns
/// 
///  `Result<(), ValidationError>` - Represents the result of the validation operation. Returns `Ok(())` if no sensitive data is found, otherwise returns an error with a message.
/// 
/// # Examples
/// 
/// ```
/// use crate::validate_output;
/// 
/// let result = validate_output("123-45-6789");
/// assert!(result.is_err());
///
async fn validate_output(output: &str) -> Result<(), ValidationError> {
    let sensitive_data_patterns = vec![r"\d{16}", r"\d{3}-\d{2}-\d{4}"];
    for pattern in sensitive_data_patterns {
        let regex = Regex::new(pattern).unwrap();
        if regex.is_match(output) {
            return Err(ValidationError("Sensitive Data Found.".to_string()));
        }
    }
    Ok(())
}

//Concurrent worker thread to handle validation tasks:
async fn worker(receiver: Arc<Mutex<mpsc::Receiver<String>>>) {
    while let Some(message) = receiver.lock().await.recv().await {
        if let Err(e) = validate_input(&message).await {
            eprintln!("Input validation failed: {:?}", e);
        }
        if let Err(e) = validate_output(&message).await {
            eprintln!("Output validation failed: {:?}", e);
        }
        println!("Validation complete for: {}", message)
    }
} 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let input = "SELECT {} FROM {} WHERE id = {}", item, table, id;
    let output = "Your API key is abcdefg123456";
    let db = Database::new().await?;

    let input_filters = vec![
        InputFilter{
            name: String::from("SQL Injjection"),
            pattern: String::from(r"(\b(SELECT|INSERT|UPDATE|DELETE|DROP|UNION|ALTER|TRUNCATE|EXEC)\b)"), 
            description: String::from("Detects SQL injection keywords") 
        },
        InputFilter { 
            name: String::from("Command Injection"), 
            pattern: String::from(r"(;|\||&|>|<)"), 
            description: String::from("Detects command injections using shell operators") 
        },
    ]

    let output_filters = vec![
        OutputFilter { 
            name: String::from("Sensitive Data Leak"), 
            pattern: String::from(r"\b(api_key|password)\b"), 
            description: String::from("Detects sensitive information leaks") 
        },
        // Additional output filters
    ];

    // Concurrent Input Validation
    let input_validation_results = validate_input_concurrently(&input, input_filters.clone());
    println!("Input Validation Results: {:?}", input_validation_results);

    // Concurrent Output Validation
    let output_validation_results = validate_output_concurrently(&output, output_filters.clone());
    println!("Output Validation Results: {:?}", output_validation_results);

    // Optional: Run live input validation if RUN_LIVE=True
    live_input_validation("example_stream_data");


    if let Ok(run_live) = env::var("RUN_LIVE") {
        if run_live == "True" {
            println!("Running live validation...");
            // Implement live validation logic here, like handling streaming data
        } else {
            println!("Running file-based validation...");
            // Default file-based validation logic
        }
    }
}


    

    // Use RUN_LIVE environment variable to determine if we are validating live data or from a file
    let run_live = env::var("RUN_LIVE").unwrap_or("false".to_string());
    
    if run_live == "true" {
        // Example streaming data validation (simulated)
        let live_input = "Some input coming from live stream...";
        validate_input(live_input, &db).await?;
        
        let live_output = "Some output going out of the LLM...";
        validate_output(live_output, &db).await?;
    } else {
        // Example file-based validation (simulated)
        let file_input = std::fs::read_to_string("input.txt")?;
        validate_input(&file_input, &db).await?;
        
        let file_output = std::fs::read_to_string("output.txt")?;
        validate_output(&file_output, &db).await?;
    }

    Ok(())
}

   

//Simulating multiple validation requests
let messages = vec!["Test",
                    "DROP TABLE userss",
                    "1234-5678-9012-3456"];
for message in messages {
    sender.send(message.to_string()).await.unwrap();
}


