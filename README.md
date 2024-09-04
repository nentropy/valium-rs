# LLM Validator 🔒⚡️

`LLM Validator` is a concurrent, multi-threaded Rust tool designed for securing interactions with Large Language Models (LLMs). This project ensures safety by validating both incoming and outgoing messages, detecting potential prompt injection attacks, and other malicious patterns. Built for scalability and speed, it uses multiple threads to handle real-time streaming data or file-based inputs, ensuring robust LLM protection in high-performance environments.

### Features
- **Concurrent Input Validation**: Identifies harmful patterns such as SQL injection, command injection, and offensive language in user inputs.
- **Concurrent Output Validation**: Monitors LLM responses for sensitive data leakage or internal system information.
- **Dynamic Pattern Management**: Fetches validation patterns dynamically from a PostgreSQL database.
- **Flexible Execution Modes**: Supports real-time validation from live streams or file-based input/output depending on the environment configuration (`RUN_LIVE`).
- **Highly Scalable**: Built using Rust's fearless concurrency and async I/O to handle multiple validation checks concurrently.

### Operations

- **Run in Live Mode**:
  - Perform real-time input and output validation:
    ```bash
    RUN_LIVE=true cargo run
    ```

- **File-Based Validation**:
  - Validate input and output using static files:
    ```bash
    RUN_LIVE=false cargo run
    ```

### Installation

1. Install Rust and Cargo.
2. Set up PostgreSQL and create necessary tables for pattern management.
3. Clone the repository and configure the `.env` file with the database URL and environment mode.

### Example Usage

- **Input Validation**:
  Validate incoming data for harmful patterns:
  ```rust
  let input_data = "User input here";
  validate_input(input_data, &db).await?;
  ```

- **Output Validation**:
  Validate outgoing LLM responses for sensitive data:
  ```rust
  let output_data = "LLM output here";
  validate_output(output_data, &db).await?;
  ```

### License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

### Contributing

Contributions are welcome! Please ensure code quality by following Rust's best practices and using `rustfmt` for formatting. We encourage adding new validation patterns and improving system efficiency.

### Improvements
  - **Extend Pattern Library**: Add more patterns for detection, including new types of injections and security vulnerabilities.
  - **Benchmarking & Optimizations**: Introduce benchmarking to measure performance under heavy loads.
  - **Machine Learning Integration**: Use ML models to detect patterns not easily captured by regex-based filters.
  - **Enhanced Data Handling**: Add support for additional data sources and formats in live validation mode.

---