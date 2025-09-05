#[derive(Debug, PartialEq)]
pub enum CalculatorError {
    DivisionByZero,
    InvalidOperator,
}

pub struct Calculator;

impl Calculator {
    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    pub fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }

    pub fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }

    pub fn divide(a: f64, b: f64) -> Result<f64, CalculatorError> {
        if b == 0.0 {
            Err(CalculatorError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }

    pub fn calculate(a: f64, b: f64, operator: &str) -> Result<f64, CalculatorError> {
        match operator {
            "+" => Ok(Self::add(a, b)),
            "-" => Ok(Self::subtract(a, b)),
            "*" => Ok(Self::multiply(a, b)),
            "/" => Self::divide(a, b),
            _ => Err(CalculatorError::InvalidOperator),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Calculator::add(2.5, 3.5), 6.0);
        assert_eq!(Calculator::add(-1.0, 1.0), 0.0);
        assert_eq!(Calculator::add(0.0, 5.0), 5.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(Calculator::subtract(5.0, 3.0), 2.0);
        assert_eq!(Calculator::subtract(3.0, 5.0), -2.0);
        assert_eq!(Calculator::subtract(0.0, 5.0), -5.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(Calculator::multiply(2.0, 3.0), 6.0);
        assert_eq!(Calculator::multiply(-2.0, 3.0), -6.0);
        assert_eq!(Calculator::multiply(0.0, 5.0), 0.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(Calculator::divide(6.0, 3.0), Ok(2.0));
        assert_eq!(Calculator::divide(5.0, 2.0), Ok(2.5));
        assert_eq!(Calculator::divide(-6.0, 3.0), Ok(-2.0));
        
        // Test division by zero
        assert_eq!(Calculator::divide(5.0, 0.0), Err(CalculatorError::DivisionByZero));
    }

    #[test]
    fn test_calculate() {
        assert_eq!(Calculator::calculate(2.0, 3.0, "+"), Ok(5.0));
        assert_eq!(Calculator::calculate(5.0, 3.0, "-"), Ok(2.0));
        assert_eq!(Calculator::calculate(2.0, 3.0, "*"), Ok(6.0));
        assert_eq!(Calculator::calculate(6.0, 3.0, "/"), Ok(2.0));
        
        // Test invalid operator
        assert_eq!(Calculator::calculate(6.0, 3.0, "^"), Err(CalculatorError::InvalidOperator));
        
        // Test division by zero through calculate
        assert_eq!(Calculator::calculate(6.0, 0.0, "/"), Err(CalculatorError::DivisionByZero));
    }

    #[test]
    fn test_error_types() {
        assert_eq!(CalculatorError::DivisionByZero, CalculatorError::DivisionByZero);
        assert_eq!(CalculatorError::InvalidOperator, CalculatorError::InvalidOperator);
        assert_ne!(CalculatorError::DivisionByZero, CalculatorError::InvalidOperator);
    }
}