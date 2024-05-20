use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive a [`atlas_lib::ui::sidebar::MakeUi`] implementation for a struct using field attributes.
/// NOTE: Baby's first derive macro, looks bad but does the job.
#[proc_macro_derive(MakeUi, attributes(add, control, hint, name))]
pub fn make_ui_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    let fields = match ast.data {
        syn::Data::Struct(s) => s.fields,
        _ => unimplemented!(),
    };
    let fields = match fields {
        syn::Fields::Named(n) => n.named,
        _ => unimplemented!(),
    };

    let mut controls = vec![];
    let mut labels = vec![];
    let mut idents = vec![];
    let mut hints = vec![];
    let mut all_funs = vec![];
    for field in fields {
        let mut control: Option<TokenStream2> = None;
        let mut label: Option<TokenStream2> = None;
        let mut hint: Option<TokenStream2> = None;
        let mut funs = vec![];
        let ident = field.ident.unwrap();

        for attr in field.attrs {
            if attr.path().is_ident("name") {
                label = match attr.meta {
                    syn::Meta::List(l) => Some(l.tokens),
                    _ => panic!("Config fields UI label cannot be empty"),
                }
            } else if attr.path().is_ident("control") {
                control = match attr.meta {
                    syn::Meta::List(l) => Some(l.tokens),
                    _ => panic!("Config fields UI control cannot be empty"),
                }
            } else if attr.path().is_ident("add") {
                let fun = match attr.meta {
                    syn::Meta::List(l) => l.tokens,
                    _ => panic!("Config fields UI modifier methods cannot be empty"),
                };
                funs.push(fun);
            } else if attr.path().is_ident("hint") {
                hint = match attr.meta {
                    syn::Meta::List(l) => {
                        let msg = l.tokens;
                        Some(quote!(Option::<&str>::Some(#msg)))
                    }
                    _ => panic!("Config fields UI hint cannot be empty"),
                };
            }
        }
        if control.is_none() {
            continue;
        }
        controls.push(control.unwrap());
        labels.push(label.expect("Config field UI label must be provided"));
        idents.push(ident);
        all_funs.push(funs);
        hints.push(hint.unwrap_or_else(|| quote!(Option::<&str>::None)));
    }

    TokenStream::from(quote! {
        impl #impl_generics atlas_lib::ui::sidebar::MakeUi for #struct_name #type_generics #where_clause {
            fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
                #(
                    let result = atlas_lib::ui::sidebar::#controls::new(ui, #labels, &mut self.#idents)#(.#all_funs)*.show(#hints);
                    atlas_lib::ui::sidebar::#controls::post_show(result, &mut self.#idents);
                )*
            }
        }
    })
}

/// Derive a [`atlas_lib::ui::sidebar::MakeUi`] implementation for an enum using variant attributes.
/// NOTE: Baby's first derive macro, looks bad but does the job.
#[proc_macro_derive(MakeUiEnum, attributes(empty))]
pub fn make_ui_enum_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let enum_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    let variants = match ast.data {
        syn::Data::Enum(e) => e.variants,
        _ => unimplemented!(),
    };

    let mut idents = vec![];
    for variant in variants {
        let mut empty = false;
        for attr in variant.attrs {
            if attr.path().is_ident("empty") {
                empty = true;
            }
        }
        if !empty {
            idents.push(variant.ident)
        }
    }

    TokenStream::from(quote! {
        impl #impl_generics atlas_lib::ui::sidebar::MakeUi for #enum_name #type_generics #where_clause {
            fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) {
                match self {
                    #(Self::#idents(x) => x.make_ui(ui),)*
                    _ => {},
                }
            }
        }
    })
}

/// Derive a [`atlas_lib::ui::UiEditableEnum`] implementation for an enum.
/// NOTE: Baby's first derive macro, looks bad but does the job.
#[proc_macro_derive(UiEditableEnum, attributes(invisible))]
pub fn ui_editable_enum_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let enum_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    let variants = match ast.data {
        syn::Data::Enum(e) => e.variants,
        syn::Data::Struct(_) => unimplemented!(),
        syn::Data::Union(_) => unimplemented!(),
    };

    let mut len = variants.len();
    let mut idents = vec![];
    let mut matched = vec![];
    let mut default = vec![];
    let mut indices = vec![];
    let mut index: usize = 0;
    for variant in variants.into_iter() {
        let mut is_invisible = false;
        for attr in variant.attrs {
            if attr.path().is_ident("invisible") {
                is_invisible = true;
                break;
            }
        }
        if variant.fields.is_empty() {
            matched.push(TokenStream2::default());
            default.push(TokenStream2::default());
        } else {
            matched.push(quote!((_)));
            default.push(quote!((Default::default())));
        }
        idents.push(variant.ident);
        indices.push(if is_invisible { 9999 } else { index });
        if is_invisible {
            len -= 1;
        } else {
            index += 1;
        }
    }

    TokenStream::from(quote! {
        impl #impl_generics atlas_lib::ui::UiEditableEnum for #enum_name #type_generics #where_clause {
            const LEN: usize = #len;

            fn self_as_index(&self) -> usize {
                match self {
                    #(Self::#idents #matched => #indices,)*
                }
            }

            fn index_as_self(&self, idx: usize) -> Self {
                match idx {
                    #(#indices => Self::#idents #default,)*
                    _ => panic!(),
                }
            }

            fn index_to_str(idx: usize) -> &'static str {
                match idx {
                    #(#indices => stringify!(#idents),)*
                    _ => panic!(),
                }
            }
        }
    })
}
