use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub enum AssignOperator {
    Plus,
    Sub,
    Times,
    Divide,
    Modulo,
    Equal,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum BinaryOperator {
    Plus,
    Sub,
    Times,
    Exponent,
    Divide,
    Modulo,

    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,

    And,
    Or,
}

impl BinaryOperator {
    pub fn get_precedence(&self) -> u8 {
        match self {
            BinaryOperator::Exponent => 4,
            BinaryOperator::Times | BinaryOperator::Divide | BinaryOperator::Modulo => 3,
            BinaryOperator::Plus | BinaryOperator::Sub => 2,
            BinaryOperator::Equal
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterThanOrEqual
            | BinaryOperator::LessThan
            | BinaryOperator::LessThanOrEqual => 1,
            BinaryOperator::And | BinaryOperator::Or => 0,
        }
    }

    pub fn get_verb(&self) -> &'static str {
        match self {
            BinaryOperator::Plus => "add",
            BinaryOperator::Sub => "subtract",
            BinaryOperator::Times => "multiply",
            BinaryOperator::Exponent => "raise",
            BinaryOperator::Divide => "divide",
            BinaryOperator::Modulo => "take the modulo of",
            BinaryOperator::Equal => "check for equality of",
            BinaryOperator::GreaterThan => "check if",
            BinaryOperator::GreaterThanOrEqual => "check if",
            BinaryOperator::LessThan => "check if",
            BinaryOperator::LessThanOrEqual => "check if",
            BinaryOperator::And => "use 'and' operator (&&)",
            BinaryOperator::Or => "use 'or' operator (||)",
        }
    }
    pub fn get_proposition(&self) -> &'static str {
        match self {
            BinaryOperator::Plus => "to",
            BinaryOperator::Sub => "to",
            BinaryOperator::Times => "with",
            BinaryOperator::Divide => "with",
            BinaryOperator::Exponent => "to the power of",
            BinaryOperator::Modulo => "by",
            BinaryOperator::Equal => "and",
            BinaryOperator::GreaterThan => "is greater than",
            BinaryOperator::GreaterThanOrEqual => "is greater than or equal to",
            BinaryOperator::LessThan => "is less than",
            BinaryOperator::LessThanOrEqual => "is less than or equal to",
            BinaryOperator::And => "with",
            BinaryOperator::Or => "with",
        }
    }
}

impl fmt::Display for AssignOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssignOperator::Plus => write!(f, "+="),
            AssignOperator::Sub => write!(f, "-="),
            AssignOperator::Times => write!(f, "*="),
            AssignOperator::Divide => write!(f, "/="),
            AssignOperator::Modulo => write!(f, "%="),
            AssignOperator::Equal => write!(f, "="),
        }
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Plus => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Times => write!(f, "*"),
            BinaryOperator::Exponent => write!(f, "^"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Modulo => write!(f, "%"),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::GreaterThanOrEqual => write!(f, ">="),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::LessThanOrEqual => write!(f, "<="),
            BinaryOperator::And => write!(f, "&&"),
            BinaryOperator::Or => write!(f, "||"),
        }
    }
}
