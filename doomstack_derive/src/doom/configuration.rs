use crate::doom::{Description, Property, Wrap};

use syn::{parse, Attribute, FieldsNamed};

pub(crate) struct Configuration {
    pub description: Description,
    pub wrap: Option<Wrap>,
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
        let mut config_wrap: Option<Wrap> = None;

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
                Property::Wrap { constructor } => {
                    if config_wrap.is_none() {
                        config_wrap = Some(Wrap { constructor });
                    } else {
                        panic!("multiple `wrap`s for the same item");
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

        match &config_wrap {
            Some(wrap) => {
                let fields = match fields {
                    Some(fields) => fields,
                    None => panic!("unexpected `wrap` on unit item"),
                };

                if fields.named.len() != 1 {
                    panic!("`wrap` can be called only on single-field items");
                }
            }
            None => {}
        }

        Configuration {
            description: config_description,
            wrap: config_wrap,
        }
    }
}
