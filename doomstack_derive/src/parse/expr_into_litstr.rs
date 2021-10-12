use syn::{Expr, Lit, LitStr};

pub(crate) fn expr_into_litstr(expr: &Expr) -> &LitStr {
    match expr {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Str(lit) => lit,
            _ => panic!("unexpected literal: {:?}", lit),
        },
        _ => panic!("unexpected expression: {:?}", expr),
    }
}
