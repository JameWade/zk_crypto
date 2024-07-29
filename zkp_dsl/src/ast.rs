use std::fmt;

#[derive(Debug,PartialEq)]
pub enum Expr{
    Identifier(String),
    Number(i64),
    BinaryOp{
        left:Box<Expr>,
        operator:BinaryOperator,
        right:Box<Expr>,
    },
}

#[derive(Debug,PartialEq,Clone)]
pub enum BinaryOperator{
    Add,
    Subtract,
    Multiply,
    Divide,
    DoubleEquals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
}
#[derive(Debug,PartialEq)]
pub enum Statement{
    Input {
        is_private: bool,
        name: String,
        typ: String,
    },
    Constraint {
        name: String,
        expr: Expr,
    },
    Assert(Expr),
}
#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Identifier(name) => write!(f, "{}", name),
            Expr::Number(value) => write!(f, "{}", value),
            Expr::BinaryOp { left, operator, right } => write!(f, "({} {} {})", left, operator, right),
        }
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::DoubleEquals => write!(f, "=="),
            BinaryOperator::NotEquals => write!(f, "!="),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::LessThanEquals => write!(f, "<="),
            BinaryOperator::GreaterThanEquals => write!(f, ">="),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Input { is_private, name, typ } => {
                write!(f, "input {} {}: {}", if *is_private { "private" } else { "public" }, name, typ)
            },
            Statement::Constraint { name, expr } => write!(f, "constraint {} {{ {} }}", name, expr),
            Statement::Assert(expr) => write!(f, "assert {}", expr),
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }
        Ok(())
    }
}