use syn::{Expr, Ident};

pub(crate) fn expr_into_path_into_ident(expr: &Expr) -> &Ident {
    match expr {
        Expr::Path(path) => {
            let segments = &path.path.segments;

            if segments.len() > 1 {
                panic!("unexpected multi-segment path: {:?}", path);
            }

            &segments.first().unwrap().ident
        }
        _ => panic!("unexpected expression: {:?}", expr),
    }
}
