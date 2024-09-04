-- Create the database for LLM Validator
CREATE DATABASE llm_validator;

-- Connect to the newly created database
\c llm_validator;

-- Create a table for storing input validation patterns
CREATE TABLE input_validation_patterns (
    id SERIAL PRIMARY KEY,
    pattern_name VARCHAR(255) NOT NULL,
    regex_pattern TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create a table for storing output validation patterns
CREATE TABLE output_validation_patterns (
    id SERIAL PRIMARY KEY,
    pattern_name VARCHAR(255) NOT NULL,
    regex_pattern TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Insert initial input validation patterns
INSERT INTO input_validation_patterns (pattern_name, regex_pattern, description) VALUES
('SQL Injection', '(\b(SELECT|INSERT|UPDATE|DELETE|DROP|UNION|ALTER|TRUNCATE|EXEC)\b)', 'Detects SQL injection keywords'),
('Command Injection', '(;|\||&|>|<)', 'Detects command injections using shell operators'),
('XSS Attack', '(<script\b[^>]*>(.*?)</script>)', 'Detects potential cross-site scripting (XSS) attacks'),
('Offensive Language', '(offensive_word_1|offensive_word_2)', 'Detects offensive language'),
('Code Injection', '(\b(eval|exec|compile|os\.system)\b)', 'Detects possible code injection attempts');

-- Insert initial output validation patterns
INSERT INTO output_validation_patterns (pattern_name, regex_pattern, description) VALUES
('Sensitive Data', '(\b(password|ssn|credit_card|api_key)\b)', 'Detects sensitive data in the output'),
('Internal Info Leak', '(\b(server_name|host|database|username)\b)', 'Detects internal server information leakage'),
('Prompt Injection', '(\b(prompt)\b)', 'Detects prompt injection keywords'),
('Error Message Disclosure', '(\b(SyntaxError|TypeError|NullPointerException)\b)', 'Detects error message disclosures in LLM output'),
('PII Data', '(\b(email|phone|address)\b)', 'Detects personal identifiable information in the output');

-- Add triggers for updating the timestamp on updates
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = NOW();
   RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for updating input validation patterns
CREATE TRIGGER update_input_pattern_timestamp
BEFORE UPDATE ON input_validation_patterns
FOR EACH ROW EXECUTE FUNCTION update_timestamp();

-- Trigger for updating output validation patterns
CREATE TRIGGER update_output_pattern_timestamp
BEFORE UPDATE ON output_validation_patterns
FOR EACH ROW EXECUTE FUNCTION update_timestamp();
