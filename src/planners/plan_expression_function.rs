// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under AGPL License, Version 3.0.

use crate::planners::ExpressionPlan;

/// return a new expression l <op> r
fn binary_expr(l: ExpressionPlan, op: &str, r: ExpressionPlan) -> ExpressionPlan {
    ExpressionPlan::BinaryExpression {
        left: Box::new(l),
        op: op.to_string(),
        right: Box::new(r),
    }
}

pub fn add(left: ExpressionPlan, right: ExpressionPlan) -> ExpressionPlan {
    binary_expr(left, "+", right)
}

impl ExpressionPlan {
    /// Equal
    pub fn eq(&self, other: ExpressionPlan) -> ExpressionPlan {
        binary_expr(self.clone(), "=", other)
    }

    /// Not equal
    pub fn not_eq(&self, other: ExpressionPlan) -> ExpressionPlan {
        binary_expr(self.clone(), "!=", other)
    }

    /// Greater than
    pub fn gt(&self, other: ExpressionPlan) -> ExpressionPlan {
        binary_expr(self.clone(), ">", other)
    }

    /// Greater than or equal to
    pub fn gt_eq(&self, other: ExpressionPlan) -> ExpressionPlan {
        binary_expr(self.clone(), ">=", other)
    }

    /// Less than
    pub fn lt(&self, other: ExpressionPlan) -> ExpressionPlan {
        binary_expr(self.clone(), "<", other)
    }

    /// Less than or equal to
    pub fn lt_eq(&self, other: ExpressionPlan) -> ExpressionPlan {
        binary_expr(self.clone(), "<=", other)
    }
}
