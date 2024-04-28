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
        }
    }
}
