use crate::parse::{expr_into_litstr, expr_into_path_into_ident};

use syn::parse;
use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, ExprCall, Ident, LitStr};

pub(crate) enum Property {
    StaticDescription {
        description: LitStr,
    },
    DynamicDescription {
        description: LitStr,
        arguments: Vec<Ident>,
    },
    Wrap {
        constructor: Ident,
    },
}

enum PropertyType {
    Description,
    Wrap,
}

impl Parse for Property {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let body;
        parenthesized!(body in input);

        let call = body.parse::<ExprCall>()?;

        match property_type(&call) {
            PropertyType::Description => parse_description(&call),
            PropertyType::Wrap => parse_wrap(&call),
        }
    }
}

fn property_type(call: &ExprCall) -> PropertyType {
    let ident = expr_into_path_into_ident(&*call.func);

    match ident.to_string().as_str() {
        "description" => PropertyType::Description,
        "wrap" => PropertyType::Wrap,
        _ => panic!("unexpected property: {:?}", ident),
    }
}

fn parse_description(call: &ExprCall) -> parse::Result<Property> {
    let mut args = call.args.iter();

    if args.len() < 1 {
        panic!("`description` expects at least one argument");
    }

    let description = expr_into_litstr(&args.next().unwrap()).clone();

    if args.len() == 0 {
        Ok(Property::StaticDescription { description })
    } else {
        let arguments = args
            .map(expr_into_path_into_ident)
            .cloned()
            .collect::<Vec<_>>();

        Ok(Property::DynamicDescription {
            description,
            arguments,
        })
    }
}

fn parse_wrap(call: &ExprCall) -> parse::Result<Property> {
    let args = &call.args;

    if args.len() != 1 {
        panic!("`wrap` expects exactly one argument");
    }

    let constructor = expr_into_path_into_ident(args.first().unwrap()).clone();

    Ok(Property::Wrap { constructor })
}
