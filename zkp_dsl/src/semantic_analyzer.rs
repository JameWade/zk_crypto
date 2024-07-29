use std::cmp::PartialEq;
use std::collections::HashMap;
use crate::ast::{Program, Statement, Expr, BinaryOperator};

#[derive(Debug, PartialEq,Clone)]
enum Type {
    Field,
    Boolean,
    // Add more types as needed
}

struct SymbolTable {
    variables: HashMap<String, Type>,
    constraints: HashMap<String, ()>,
}

impl SymbolTable {
    fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
            constraints: HashMap::new(),
        }
    }

    fn add_variable(&mut self, name: String, typ: Type) -> Result<(), String> {
        if self.variables.contains_key(&name) {
            Err(format!("Variable '{}' already declared", name))
        } else {
            self.variables.insert(name, typ);
            Ok(())
        }
    }

    fn add_constraint(&mut self, name: String) -> Result<(), String> {
        if self.constraints.contains_key(&name) {
            Err(format!("Constraint '{}' already defined", name))
        } else {
            self.constraints.insert(name, ());
            Ok(())
        }
    }

    fn get_variable_type(&self, name: &str) -> Option<&Type> {
        self.variables.get(name)
    }
}

fn analyze_expr(expr: &Expr, symbol_table: &SymbolTable) -> Result<Type, String> {
    match expr {
        Expr::Identifier(name) => {
            symbol_table.get_variable_type(name)
                .cloned()
                .ok_or_else(|| format!("Undeclared variable: {}", name))
        },
        Expr::Number(_) => Ok(Type::Field),
        Expr::BinaryOp { left, operator, right } => {
            let left_type = analyze_expr(left, symbol_table)?;
            let right_type = analyze_expr(right, symbol_table)?;

            match operator {
                BinaryOperator::Add | BinaryOperator::Subtract | BinaryOperator::Multiply | BinaryOperator::Divide => {
                    if left_type == Type::Field && right_type == Type::Field {
                        Ok(Type::Field)
                    } else {
                        Err("Arithmetic operations require Field type operands".to_string())
                    }
                },
                BinaryOperator::DoubleEquals | BinaryOperator::NotEquals |
                BinaryOperator::LessThan | BinaryOperator::GreaterThan |
                BinaryOperator::LessThanEquals | BinaryOperator::GreaterThanEquals => {
                    if left_type == right_type {
                        Ok(Type::Boolean)
                    } else {
                        Err(format!("Comparison operations require operands of the same type. Found {:?} and {:?}", left_type, right_type))
                    }
                },
            }
        },
    }
}

fn analyze_statement(statement: &Statement, symbol_table: &mut SymbolTable) -> Result<(), String> {
    match statement {
        Statement::Input { is_private: _, name, typ } => {
            let var_type = match typ.as_str() {
                "Field" => Type::Field,
                "Boolean" => Type::Boolean,
                _ => return Err(format!("Unsupported type: {}", typ)),
            };
            symbol_table.add_variable(name.clone(), var_type)?;
        },
        Statement::Constraint { name, expr } => {
            symbol_table.add_constraint(name.clone())?;
            let expr_type = debug_analyze_expr(expr, symbol_table)?;
            if expr_type != Type::Boolean {
                return Err(format!("Constraint expression must be of boolean type, found {:?}", expr_type));
            }
        },
        Statement::Assert(expr) => {
            let expr_type = analyze_expr(expr, symbol_table)?;
            if expr_type != Type::Boolean {
                return Err(format!("Assert expression must be of boolean type, found {:?}", expr_type));
            }
        },
    }
    Ok(())
}

pub fn analyze(program: &Program) -> Result<(), String> {
    let mut symbol_table = SymbolTable::new();

    for statement in &program.statements {
        analyze_statement(statement, &mut symbol_table)?;
    }

    Ok(())
}

// 添加用于调试的函数
pub fn debug_analyze_expr(expr: &Expr, symbol_table: &SymbolTable) -> Result<Type, String> {
    println!("Analyzing expression: {:?}", expr);
    let result = analyze_expr(expr, symbol_table);
    println!("Result: {:?}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    fn parse_and_analyze(input: &str) -> Result<(), String> {
        let mut parser = Parser::new(input);
        let program = parser.parse_program()?;
        println!("{}", program);
        analyze(&program)
    }

    #[test]
    fn test_valid_program() {
        let input = "
            input private x: Field;
            input y: Field;
            constraint c1 { x * y == 10 };
            assert x + y > 0;
        ";
        // assert x + y > 0;
        let mut parser = Parser::new(input);
        assert!(parse_and_analyze(input).is_ok());

    }

    #[test]
    fn test_undeclared_variable() {
        let input = "
            input x: Field
            constraint c1 { x * y == 10 }
        ";
        assert!(parse_and_analyze(input).is_err());
    }

    #[test]
    fn test_duplicate_constraint() {
        let input = "
            input x: Field
            constraint c1 { x == 10 }
            constraint c1 { x == 20 }
        ";
        assert!(parse_and_analyze(input).is_err());
    }

    #[test]
    fn test_type_mismatch() {
        let input = "
            input x: Field
            input y: Boolean
            constraint c1 { x + y == 10 }
        ";
        assert!(parse_and_analyze(input).is_err());
    }
}