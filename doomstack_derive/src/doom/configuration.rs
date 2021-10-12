use crate::doom::{Description, Property};

use syn::{parse, Attribute, FieldsNamed, Ident, LitStr};

pub(crate) struct Configuration {
    pub description: Description,
}

impl Configuration {
    pub fn new(attrs: &[Attribute], fields: &Option<FieldsNamed>) -> Self {
        let properties =
            attrs
                .iter()
                .map(|attr| match parse::<Property>(attr.tokens.clone().into()) {
                    Ok(property) => property,
                    Err(error) => panic!("{:?}", error),
                });

        let mut config_description: Option<Description> = None;

        for property in properties {
            match property {
                Property::StaticDescription { description } => {
                    if config_description.is_none() {
                        config_description = Some(Description::Static { description });
                    } else {
                        panic!("multiple `description`s for the same item");
                    }
                }
                Property::DynamicDescription {
                    description,
                    arguments,
                } => {
                    if config_description.is_none() {
                        config_description = Some(Description::Dynamic {
                            description,
                            arguments,
                        });
                    } else {
                        panic!("multiple `description`s for the same item");
                    }
                }
            }
        }

        if config_description.is_none() {
            panic!("missing `description`");
        }

        let config_description = config_description.unwrap();

        match &config_description {
            Description::Dynamic { arguments, .. } => {
                let fields = match fields {
                    Some(fields) => fields,
                    None => panic!("unexpected description arguments on unit item"),
                };

                for argument in arguments.iter() {
                    match fields
                        .named
                        .iter()
                        .find(|&field| field.ident.as_ref().unwrap() == argument)
                    {
                        Some(_) => {}
                        None => panic!("argument not found in field list: {:?}", argument),
                    }
                }
            }
            Description::Static { .. } => {}
        }

        Configuration {
            description: config_description,
        }
    }
}
