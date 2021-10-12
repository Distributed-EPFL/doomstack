use syn::{Ident, LitStr};

pub(crate) enum Description {
    Static {
        description: LitStr,
    },
    Dynamic {
        description: LitStr,
        arguments: Vec<Ident>,
    },
}
