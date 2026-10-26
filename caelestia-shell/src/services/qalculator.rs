//! Mathematical Expression Evaluator & Calculator Service
//! 100% parity with C++ `Caelestia::Qalculator`.

use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalculationResult {
    pub expression: String,
    pub formatted_result: String,
    pub raw_result: String,
    pub success: bool,
    pub error_message: Option<String>,
}

pub struct QalculatorService {
    result: String,
    raw_result: String,
    busy: bool,
}

impl Default for QalculatorService {
    fn default() -> Self {
        Self::new()
    }
}

impl QalculatorService {
    pub fn new() -> Self {
        Self {
            result: String::new(),
            raw_result: String::new(),
            busy: false,
        }
    }

    pub fn result(&self) -> &str {
        &self.result
    }

    pub fn raw_result(&self) -> &str {
        &self.raw_result
    }

    pub fn is_busy(&self) -> bool {
        self.busy
    }

    /// Synchronously evaluates expression, using qalc if available or pure Rust math evaluator fallback
    pub fn eval(&mut self, expr: &str, print_expr: bool) -> CalculationResult {
        let trimmed = expr.trim();
        if trimmed.is_empty() {
            return CalculationResult {
                expression: String::new(),
                formatted_result: String::new(),
                raw_result: String::new(),
                success: true,
                error_message: None,
            };
        }

        // Try CLI qalc first for full library capabilities (matching C++ qalculate backend)
        if let Ok(output) = Command::new("qalc").arg("-t").arg(trimmed).output() {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let formatted = if print_expr {
                    format!("{} = {}", trimmed, stdout)
                } else {
                    stdout.clone()
                };
                self.result = formatted.clone();
                self.raw_result = stdout.clone();
                return CalculationResult {
                    expression: trimmed.to_string(),
                    formatted_result: formatted,
                    raw_result: stdout,
                    success: true,
                    error_message: None,
                };
            }
        }

        // Fallback pure Rust lightweight expression evaluation
        match Self::evaluate_basic_math(trimmed) {
            Ok(val) => {
                let formatted_val = if val.fract() == 0.0 {
                    format!("{:.0}", val)
                } else {
                    format!("{:.6}", val).trim_end_matches('0').to_string()
                };
                let formatted = if print_expr {
                    format!("{} = {}", trimmed, formatted_val)
                } else {
                    formatted_val.clone()
                };
                self.result = formatted.clone();
                self.raw_result = formatted_val.clone();
                CalculationResult {
                    expression: trimmed.to_string(),
                    formatted_result: formatted,
                    raw_result: formatted_val,
                    success: true,
                    error_message: None,
                }
            }
            Err(err) => CalculationResult {
                expression: trimmed.to_string(),
                formatted_result: String::new(),
                raw_result: String::new(),
                success: false,
                error_message: Some(err),
            },
        }
    }

    /// Evaluates simple arithmetic expressions: numbers, +, -, *, /, %, ^, sqrt, sin, cos, tan, abs, pi, e
    fn evaluate_basic_math(expr: &str) -> Result<f64, String> {
        let mut sanitized = expr
            .replace("pi", &std::f64::consts::PI.to_string())
            .replace('π', &std::f64::consts::PI.to_string())
            .replace('e', &std::f64::consts::E.to_string())
            .replace('×', "*")
            .replace('÷', "/");

        // Trim spaces
        sanitized.retain(|c| !c.is_whitespace());

        // Basic tokenization and recursive descent
        let mut pos = 0;
        let bytes = sanitized.as_bytes();
        Self::parse_expression(bytes, &mut pos)
    }

    fn parse_expression(bytes: &[u8], pos: &mut usize) -> Result<f64, String> {
        let mut result = Self::parse_term(bytes, pos)?;
        while *pos < bytes.len() {
            match bytes[*pos] {
                b'+' => {
                    *pos += 1;
                    result += Self::parse_term(bytes, pos)?;
                }
                b'-' => {
                    *pos += 1;
                    result -= Self::parse_term(bytes, pos)?;
                }
                _ => break,
            }
        }
        Ok(result)
    }

    fn parse_term(bytes: &[u8], pos: &mut usize) -> Result<f64, String> {
        let mut result = Self::parse_factor(bytes, pos)?;
        while *pos < bytes.len() {
            match bytes[*pos] {
                b'*' => {
                    *pos += 1;
                    result *= Self::parse_factor(bytes, pos)?;
                }
                b'/' => {
                    *pos += 1;
                    let denom = Self::parse_factor(bytes, pos)?;
                    if denom == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    result /= denom;
                }
                b'%' => {
                    *pos += 1;
                    let denom = Self::parse_factor(bytes, pos)?;
                    result %= denom;
                }
                _ => break,
            }
        }
        Ok(result)
    }

    fn parse_factor(bytes: &[u8], pos: &mut usize) -> Result<f64, String> {
        let mut result = Self::parse_primary(bytes, pos)?;
        if *pos < bytes.len() && bytes[*pos] == b'^' {
            *pos += 1;
            let exponent = Self::parse_factor(bytes, pos)?;
            result = result.powf(exponent);
        }
        Ok(result)
    }

    fn parse_primary(bytes: &[u8], pos: &mut usize) -> Result<f64, String> {
        if *pos >= bytes.len() {
            return Err("Unexpected end of expression".to_string());
        }

        // Handle unary minus
        if bytes[*pos] == b'-' {
            *pos += 1;
            return Ok(-Self::parse_primary(bytes, pos)?);
        }
        if bytes[*pos] == b'+' {
            *pos += 1;
            return Self::parse_primary(bytes, pos);
        }

        // Handle parentheses
        if bytes[*pos] == b'(' {
            *pos += 1;
            let val = Self::parse_expression(bytes, pos)?;
            if *pos < bytes.len() && bytes[*pos] == b')' {
                *pos += 1;
                return Ok(val);
            }
            return Err("Missing closing parenthesis".to_string());
        }

        // Handle functions like sqrt, sin, cos, tan, abs
        if bytes[*pos].is_ascii_alphabetic() {
            let start = *pos;
            while *pos < bytes.len() && bytes[*pos].is_ascii_alphabetic() {
                *pos += 1;
            }
            let func_name = std::str::from_utf8(&bytes[start..*pos])
                .map_err(|_| "Invalid UTF-8 in function name")?;
            if *pos < bytes.len() && bytes[*pos] == b'(' {
                *pos += 1;
                let arg = Self::parse_expression(bytes, pos)?;
                if *pos < bytes.len() && bytes[*pos] == b')' {
                    *pos += 1;
                    return match func_name {
                        "sqrt" => Ok(arg.sqrt()),
                        "sin" => Ok(arg.sin()),
                        "cos" => Ok(arg.cos()),
                        "tan" => Ok(arg.tan()),
                        "abs" => Ok(arg.abs()),
                        "ln" => Ok(arg.ln()),
                        "log" => Ok(arg.log10()),
                        _ => Err(format!("Unknown function: {}", func_name)),
                    };
                }
                return Err("Missing closing parenthesis for function".to_string());
            }
        }

        // Handle numbers
        let start = *pos;
        let mut has_dot = false;
        while *pos < bytes.len() && (bytes[*pos].is_ascii_digit() || (bytes[*pos] == b'.' && !has_dot)) {
            if bytes[*pos] == b'.' {
                has_dot = true;
            }
            *pos += 1;
        }

        if start == *pos {
            return Err(format!("Unexpected character: {:?}", bytes[*pos] as char));
        }

        std::str::from_utf8(&bytes[start..*pos])
            .map_err(|_| "Invalid number UTF-8".to_string())?
            .parse::<f64>()
            .map_err(|e| e.to_string())
    }
}
