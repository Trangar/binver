extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use semver::Version;
use syn::{
    parse::{Parse, Parser},
    spanned::Spanned,
    Attribute, Data, DataEnum, DeriveInput, Error, Field, Fields, Ident, Variant,
};

#[proc_macro_derive(Serializable, attributes(since))]
pub fn derive_serializable(item: TokenStream) -> TokenStream {
    let input = match DeriveInput::parse.parse2(item.into()) {
        Ok(data) => data,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    let ident = input.ident;

    match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(fields) => derive_serializable_struct_fields(ident, fields.named),
            Fields::Unnamed(fields) => derive_serializable_struct_fields(ident, fields.unnamed),
            Fields::Unit => Error::new(s.struct_token.span, "Unit structs not supported")
                .into_compile_error()
                .into(),
        },
        Data::Enum(e) => derive_serializable_enum(ident, e),
        Data::Union(_) => Error::new(ident.span(), "Unions not supported")
            .into_compile_error()
            .into(),
    }
}

fn derive_serializable_struct_fields<'a>(
    ident: Ident,
    fields: impl IntoIterator<Item = Field>,
) -> TokenStream {
    let mut ser_impl = Vec::new();
    let mut de_impl = Vec::new();
    let mut idents = Vec::new();
    let mut highest_version = None;
    for f in fields {
        let ident = f.ident.unwrap();
        let ty = &f.ty;
        ser_impl.push(quote! { self.#ident.serialize(writer)?; });

        let Version {
            major,
            minor,
            patch,
            ..
        } = match parse_attribute(ident.span(), &f.attrs) {
            Ok(version) => version,
            Err(e) => return e.into_compile_error().into(),
        };

        highest_version = match (highest_version.take(), Version::new(major, minor, patch)) {
            (None, v) => Some(v),
            (Some(v1), v2) => Some(if v1 > v2 { v1 } else { v2})
        };

        de_impl.push(quote! {
            let #ident = if version < ::binver::Version::new(#major, #minor, #patch) {
                Default::default()
            } else {
                #ty::deserialize(reader)?
            };
        });
        idents.push(ident);
    }

    let highest_version = if let Some(Version { major, minor, patch, ..}) = highest_version {
        quote! { Some(binver::Version::new(#major, #minor, #patch)) }
    } else {
        quote! { None }
    };

    (quote! {
        impl binver::Serializable for #ident {
            fn serialize(&self, writer: &mut dyn binver::Writer) -> binver::WriteResult {
                #(#ser_impl)*
                Ok(())
            }
            fn deserialize(reader: &mut dyn binver::Reader) -> binver::ReadResult<Self> {
                let version = reader.version();
                #(#de_impl)*
                Ok(Self {
                    #(#idents, )*
                })
            }
            fn highest_version() -> Option<binver::Version> {
                #highest_version
            }
        }
    })
    .into()
}

fn derive_serializable_enum(ident: Ident, data: DataEnum) -> TokenStream {
    let mut ser_impl = Vec::new();
    let mut de_impl = Vec::new();
    let mut highest_version = None;

    // We need to validate the following, or we cannot uphold serialization guarantees:
    // 1. If one variant has a discriminant, they all have to have a discriminant

    let discriminant_count = data
        .variants
        .iter()
        .filter(|v| v.discriminant.is_some())
        .count();
    if discriminant_count != 0 && discriminant_count != data.variants.len() {
        return Error::new(
            ident.span(),
            "Enums must have either have ALL fixed values (Enum::Variant = 1), or none at all",
        )
        .into_compile_error()
        .into();
    }

    let mut last_version: Option<Version> = None;
    for (index, variant) in data.variants.into_iter().enumerate() {
        let ident = variant.ident.clone();
        let version = match parse_attribute(ident.span(), &variant.attrs) {
            Ok(version) => version,
            Err(e) => return e.into_compile_error().into(),
        };
        highest_version = match (highest_version.take(), version.clone()) {
            (None, v) => Some(v),
            (Some(v1), v2) => Some(if v1 > v2 { v1 } else { v2})
        };

        if let Some(last) = last_version.replace(version.clone()) {
            if last > version {
                return Error::new(
                    ident.span(),
                    format!("New versions must be added at the bottom. Version {:?} must be lower in the code than {:?}", last, version)
                ).into_compile_error().into();
            }
        }

        match EnumVariantSerDeResult::construct(index as u16, variant, version, &mut highest_version) {
            Ok(EnumVariantSerDeResult { ser, de }) => {
                ser_impl.push(ser);
                de_impl.push(de);
            }
            Err(e) => return e.into_compile_error().into(),
        }
    }

    let highest_version = if let Some(Version { major, minor, patch, ..}) = highest_version {
        quote! { Some(binver::Version::new(#major, #minor, #patch)) }
    } else {
        quote! { None }
    };

    (quote! {
        impl binver::Serializable for #ident {
            fn serialize(&self, writer: &mut dyn binver::Writer) -> binver::WriteResult {
                match self {
                    #(#ser_impl)*
                }
            }
            fn deserialize(reader: &mut dyn binver::Reader) -> binver::ReadResult<Self> {
                let version = reader.version();
                let variant = u16::deserialize(reader)?;
                Ok(match variant {
                    #(#de_impl)*
                    x => return Err(binver::ReadError::UnknownVariant(variant))
                })
            }

            fn highest_version() -> Option<binver::Version> {
                #highest_version
            }
        }
    })
    .into()
}

struct EnumVariantSerDeResult {
    pub ser: proc_macro2::TokenStream,
    pub de: proc_macro2::TokenStream,
}

impl EnumVariantSerDeResult {
    fn construct(index: u16, variant: Variant, version: Version, highest_version: &mut Option<Version>) -> Result<Self, Error> {
        let ident = variant.ident;
        let Version {
            major,
            minor,
            patch,
            ..
        } = version;

        match variant.fields {
            Fields::Named(fields) => {
                // Enum::Variant { a: ty, b: ty }
                let mut field_names = Vec::new();
                let mut field_serialize = Vec::new();
                let mut field_deserialize = Vec::new();

                for field in fields.named {
                    let ident = field.ident.unwrap();
                    let ty = field.ty;
                    let Version {
                        major,
                        minor,
                        patch,
                        ..
                    } = parse_attribute(ident.span(), &field.attrs)?;
                    *highest_version = match (highest_version.take(), Version::new(major, minor, patch)) {
                        (None, v) => Some(v),
                        (Some(v1), v2) => Some(if v1 > v2 { v1 } else { v2})
                    };
                    field_names.push(ident.clone());
                    field_serialize.push(quote! {
                        #ident.serialize(writer)?;
                    });
                    field_deserialize.push(quote! {
                        let #ident = if version < binver::Version::new(#major, #minor, #patch) {
                            Default::default()
                        } else {
                            #ty :: deserialize(reader)?
                        };
                    });
                }

                Ok(Self {
                    ser: quote! {
                        Self:: #ident { #(#field_names, )* } => {
                            #index.serialize(writer)?;
                            #(#field_serialize)*
                            Ok(())
                        },
                    },
                    de: quote! {
                        #index if version >= binver::Version::new(#major, #minor, #patch) => {
                            #(#field_deserialize)*
                            Self::#ident {
                                #(#field_names, )*
                            }
                        }
                    },
                })
            }
            Fields::Unnamed(_fields) => {
                // let field_names = fields.unnamed.iter().enumerate().map(|(i, _)| format!("arg_{}", i)).collect::<Vec<_>>();
                // let field_names = field_names.iter().map(|s| Ident::new(s, Span::call_site())).collect::<Vec<_>>();

                // Enum::Variant(ty, ty),
                unimplemented!("Fields::Unnamed");
            }
            Fields::Unit => {
                // Either:
                //   Enum::Variant
                //   Enum::Variant = 1
                if variant.discriminant.is_some() {
                    Ok(Self {
                        ser: quote! {
                            Self:: #ident => (Self::#ident as u16).serialize(writer),
                        },
                        de: quote! {
                            Self::#ident if version >= binver::Version::new(#major, #minor, #patch) => Self:: #ident,
                        },
                    })
                } else {
                    Ok(Self {
                        ser: quote! {
                            Self:: #ident => #index .serialize(writer),
                        },
                        de: (quote! {
                            #index if version >= binver::Version::new(#major, #minor, #patch) => Self:: #ident,
                        }),
                    })
                }
            }
        }
    }
}

fn parse_attribute(span: proc_macro2::Span, attr: &[Attribute]) -> Result<Version, Error> {
    if attr.len() != 1 {
        return Err(Error::new(
            span,
            "Expected exactly 1 attribute: `since(version)`",
        ));
    }
    let attr = attr.first().unwrap();
    match attr.path.segments.first() {
        Some(i) if i.ident.to_string() == String::from("since") => {}
        _ => {
            return Err(Error::new(
                span,
                "Expected exactly 1 attribute: `since(version)`",
            ));
        }
    }
    let content = attr.tokens.to_string();
    Version::parse(content.trim_start_matches('(').trim_end_matches(')'))
        .map_err(|e| Error::new(attr.tokens.span(), e))
}