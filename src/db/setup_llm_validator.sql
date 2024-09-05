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
('Code Injection', '(\b(eval|exec|compile|os\.system)\b)', 'Detects possible code injection attempts'),
('LDAP Injection', '(\b(|\&|\||objectClass|cn|ou|dc|admin)\b)', 'Detects LDAP injection attempts which manipulate LDAP queries'),
('Email Injection', '(%0A|%0D|\n|\r|bcc:|cc:|to:)', 'Detects email header injection attacks, commonly used in email spamming'),
('XML Injection', '(<\?xml|\<\!\[CDATA\[|\<\!DOCTYPE|\<script\b)', 'Detects XML injection, which can be used to manipulate XML parsers'),
('File Path Injection', '(\.\./|\.\.\)|(\b/etc/\b)|(\b/cmd.exe\b))', 'Detects potential directory traversal or file path injection'),
('HTML Injection', '(<(img|iframe|a|object|embed)\b[^>]*(src|href)="[^"]*")', 'Detects HTML injection, often used to embed malicious links or resources'),
('Null Byte Injection', '(%00)', 'Detects null byte injection attempts, which may be used to terminate strings prematurely and bypass filters'),
('CRLF Injection', '(%0D%0A|\r\n)', 'Detects Carriage Return Line Feed (CRLF) injection, often used for HTTP header manipulation or log poisoning'),
('Shell Injection', '(\$\(.*?\)|`.*?`)', 'Detects shell injection where attackers try to execute commands in subshells'),
('SSI Injection', '(<\!--#\s*(exec|include|echo|config|set|if)\s*-->)', 'Detects Server-Side Includes (SSI) injection attacks, which exploit server configuration vulnerabilities'),
('SQL Boolean Injection', '(\b(AND|OR)\b\s*[\d\w]+=)', 'Detects Boolean-based SQL injection attempts by looking for logic-based SQL conditions'),
('JWT Injection', '(\b(JWT|HS256|HS512)\b)', 'Detects potential JWT manipulation attempts in tokens'),
('Hexadecimal Injection', '(%[0-9A-Fa-f]{2})', 'Detects hexadecimal encoding typically used in URL encoding to bypass filters'),
('Unicode Encoding Injection', '(\\u[0-9A-Fa-f]{4})', 'Detects Unicode-encoded characters used in injection attempts'),
('JavaScript Obfuscation', '(eval\(|Function\(|setTimeout\(|setInterval\()', 'Detects obfuscated JavaScript that could indicate malicious behavior or script injection'),
('CSRF Attack', '(<input type="hidden" name="csrf_token" value="[^"]+">)', 'Detects CSRF tokens in forms to ensure that CSRF protection is in place'),
('Remote File Inclusion', '((http|https|ftp)://.*?(\.php|\.asp|\.jsp|\.pl))')

-- Insert initial output validation patterns
INSERT INTO output_validation_patterns (pattern_name, regex_pattern, description) VALUES
('Sensitive Data', '(\b(password|ssn|credit_card|api_key|secret|token)\b)', 'Detects sensitive data like passwords, SSNs, or API keys in the output'),
('Internal Info Leak', '(\b(server_name|host|database|username|env|config)\b)', 'Detects internal server information, environment variables, or configurations leaking through output'),
('Prompt Injection', '(\b(prompt|input|instruction|generate|complete)\b)', 'Detects prompt injection keywords or malicious prompt alterations in AI output'),
('Error Message Disclosure', '(\b(SyntaxError|TypeError|NullPointerException|Exception|stacktrace|error)\b)', 'Detects error message disclosures that could reveal system internals or debug information'),
('PII Data', '(\b(email|phone|address|social_security_number|dob|birthdate|ssn)\b)', 'Detects personally identifiable information (PII) in the output such as emails, phone numbers, or addresses'),
('Financial Information', '(\b(credit_card|account_number|bank_account|routing_number|iban)\b)', 'Detects financial information leakage such as credit card or bank account details'),
('JWT Disclosure', '(\b(eyJ[0-9a-zA-Z-_]{10,}\.[0-9a-zA-Z-_]{10,}\.[0-9a-zA-Z-_]{10,})\b)', 'Detects potential JSON Web Token (JWT) disclosures in the output'),
('Authentication Tokens', '(\b(Bearer\s+[a-zA-Z0-9-_]+\.[a-zA-Z0-9-_]+\.[a-zA-Z0-9-_]+)\b)', 'Detects potential leakage of Bearer authentication tokens'),
('IP Address Leak', '(\b([0-9]{1,3}\.){3}[0-9]{1,3}\b)', 'Detects IPv4 addresses in the output that may expose server or user locations'),
('Internal URL Disclosure', '(\b(http|https)://(localhost|127\.0\.0\.1|192\.168\.[0-9]{1,3}\.[0-9]{1,3}|10\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3})\b)', 'Detects internal URLs or IP addresses that could expose internal infrastructure details'),
('Stack Trace Disclosure', '(Traceback \(most recent call last\)|at .* in .*:\d+)', 'Detects stack traces which may leak details about internal software structure'),
('Encryption Keys', '(\b(-----BEGIN\s+(RSA|DSA|EC|PGP|ENCRYPTED) PRIVATE KEY-----|-----BEGIN\s+CERTIFICATE-----)\b)', 'Detects private keys or certificates being leaked in the output'),
('File Path Disclosure', '([a-zA-Z]:\\|~\/|\/var\/|\/etc\/|\/home\/)', 'Detects file path disclosures, especially those exposing sensitive directories like `/etc/`, `/var/`, or Windows drives'),
('UUID/GUID Detection', '([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})', 'Detects UUID or GUID in the output, which may leak internal identifiers'),
('HTML/JavaScript Injection', '(<(script|iframe|object|embed|img|a)[^>]*>)', 'Detects HTML/JavaScript injections in the output that could indicate XSS attacks'),
('Base64 Encoded Data', '([A-Za-z0-9+/]{20,}={0,2})', 'Detects Base64-encoded data which may indicate sensitive information being encoded but still leaking through output'),
('Internal API Keys', '(\b[A-Za-z0-9-_]{30,}\b)', 'Detects potential internal API keys or access tokens being leaked in the output'),
('Social Media Tokens', '(\b([A-Za-z0-9]{39,})\b)', 'Detects tokens from social media platforms (like Facebook, Twitter, etc.) in the output'),
('JSON Structure', '(\{[^}]*:.*?\})', 'Detects possible JSON object structures in output, useful for identifying data leaks in JSON responses'),
('HTML Comment Disclosure', '(<\!--[^>]*-->)', 'Detects HTML comments that may contain sensitive information or internal debugging data'),
('Environment Variable Leak', '(\$[A-Z_][A-Z0-9_]*(?=\s*=))', 'Detects environment variables that may have been exposed in the output'),
('Version Information Disclosure', '(\b(version|v\d+\.\d+\.\d+)\b)', 'Detects software version information leaking, which may help attackers identify vulnerable software versions'),
('Phone Number Leakage', '(\b\+?[0-9]{1,3}[\s\-\.]?[0-9]{1,4}[\s\-\.]?[0-9]{1,4}[\s\-\.]?[0-9]{1,9}\b)', 'Detects phone numbers that could expose private user information'),
('External URL Exposure', '((https?|ftp):\/\/[^\s\/$.?#].[^\s]*)', 'Detects URLs that point to external domains, useful for ensuring no sensitive information is leaked to external servers'),
('CRLF Injection in Output', '(%0D%0A|\r\n|\n|\r)', 'Detects potential CRLF injection sequences in output that could be used to inject headers or manipulate logs');

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

-- New Validation
CREATE TABLE input_validations (
    id SERIAL PRIMARY KEY,
    text TEXT NOT NULL,
    prompt_injection BOOLEAN,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE output_validations (
    id SERIAL PRIMARY KEY,
    text TEXT NOT NULL,
    sentiment VARCHAR(50),
    prompt_injection BOOLEAN,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
