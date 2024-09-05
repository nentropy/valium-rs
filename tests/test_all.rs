pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn validates_live_data_when_run_live_is_true() {
        use std::env;
        use crate::db::Database;
        use crate::input_filters::validate_input;
        use crate::output_filters::validate_output;
        use tokio::sync::Mutex;
        use std::sync::Arc;
        
        // Set the RUN_LIVE environment variable to true
        env::set_var("RUN_LIVE", "true");
        
        // Mock the database
        let db = Arc::new(Mutex::new(Database::new().await.unwrap()));
        
        // Mock the input and output validation functions
        let live_input = "Some input coming from live stream...";
        let live_output = "Some output going out of the LLM...";
        
        validate_input(live_input, &db).await.unwrap();
        validate_output(live_output, &db).await.unwrap();
        
        // Clean up the environment variable
        env::remove_var("RUN_LIVE");

    }

        #[tokio::test]
        async fn validates_file_data_when_run_live_is_not_set() {
            use std::env;
            use crate::db::Database;
            use crate::input_filters::validate_input;
            use crate::output_filters::validate_output;
            use tokio::sync::Mutex;
            use std::sync::Arc;
            use std::fs;
            
            // Ensure the RUN_LIVE environment variable is not set
            env::remove_var("RUN_LIVE");
            
            // Mock the database
            let db = Arc::new(Mutex::new(Database::new().await.unwrap()));
            
            // Mock file input and output
            let file_input = "Mock file input data";
            let file_output = "Mock file output data";
            
            // Write mock data to files
            fs::write("input.txt", file_input).unwrap();
            fs::write("output.txt", file_output).unwrap();
            
            // Read from files and validate
            let input_data = fs::read_to_string("input.txt").unwrap();
            let output_data = fs::read_to_string("output.txt").unwrap();
            
            validate_input(&input_data, &db).await.unwrap();
            validate_output(&output_data, &db).await.unwrap();
            
            // Clean up mock files
            fs::remove_file("input.txt").unwrap();
            fs::remove_file("output.txt").unwrap();
        }

    #[tokio::test]
    async fn validates_output_without_sensitive_data() {
        use crate::validate_output;
        use crate::ValidationError;

        let result = validate_output("This is a safe string").await;
        assert!(result.is_ok());
}
    #[tokio::test]
    async fn handles_empty_string_input() {
        use crate::validate_output;
        use crate::ValidationError;

        let result = validate_output("").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn valid_input_returns_ok() {
        use crate::input_filters::validate_input;
        let input = "SELECT * FROM users";
        let result = validate_input(input).await;
        assert!(result.is_ok());
    }

        // Input string containing "DROP TABLE" returns ValidationError
    #[tokio::test]
    async fn input_with_drop_table_returns_error() {
        use crate::input_filters::validate_input;
        let input = "DROP TABLE users";
        let result = validate_input(input).await;
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.to_string(), "Disallowed token found.");
        }
    }

}