use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum AssignOperator {
    Plus,
    Sub,
    Times,
    Divide,
    Modulo,
    Equal,
}

#[derive(Debug, Clone, PartialEq)]
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
