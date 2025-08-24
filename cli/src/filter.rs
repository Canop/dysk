use {
    crate::col_expr::*,
    bet::*,
    lfs_core::*,
    std::str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoolOperator {
    And,
    Or,
    Not,
}

#[derive(Debug, Default, Clone)]
pub struct Filter {
    expr: BeTree<BoolOperator, ColExpr>,
}

impl Filter {
    #[allow(clippy::match_like_matches_macro)]
    pub fn eval(
        &self,
        mount: &Mount,
    ) -> Result<bool, EvalExprError> {
        self.expr
            .eval_faillible(
                // leaf evaluation
                |col_expr| col_expr.eval(mount),
                // bool operation
                |op, a, b| match (op, b) {
                    (BoolOperator::And, Some(b)) => Ok(a & b),
                    (BoolOperator::Or, Some(b)) => Ok(a | b),
                    (BoolOperator::Not, None) => Ok(!a),
                    _ => {
                        unreachable!()
                    }
                },
                // when to short-circuit
                |op, a| match (op, a) {
                    (BoolOperator::And, false) => true,
                    (BoolOperator::Or, true) => true,
                    _ => false,
                },
            )
            .map(|b| b.unwrap_or(true))
    }
    pub fn filter<'m>(
        &self,
        mounts: &'m [Mount],
    ) -> Result<Vec<&'m Mount>, EvalExprError> {
        let mut filtered = Vec::new();
        for mount in mounts {
            if self.eval(mount)? {
                filtered.push(mount);
            }
        }
        Ok(filtered)
    }
}

impl FromStr for Filter {
    type Err = ParseExprError;
    fn from_str(input: &str) -> Result<Self, ParseExprError> {
        // we start by reading the global structure
        let mut expr: BeTree<BoolOperator, String> = BeTree::new();
        for c in input.chars() {
            match c {
                '&' => expr.push_operator(BoolOperator::And),
                '|' => expr.push_operator(BoolOperator::Or),
                '!' => expr.push_operator(BoolOperator::Not),
                ' ' => {}
                '(' => expr.open_par(),
                ')' => expr.close_par(),
                _ => expr.mutate_or_create_atom(String::new).push(c),
            }
        }

        // then we parse each leaf
        let expr = expr.try_map_atoms(|raw| raw.parse())?;

        Ok(Self { expr })
    }
}
