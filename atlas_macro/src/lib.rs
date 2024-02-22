use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MakeUi, attributes(add, control, hint, name))]
pub fn make_ui_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    let fields = match ast.data {
        syn::Data::Enum(_) => unimplemented!(),
        syn::Data::Struct(s) => s.fields,
        syn::Data::Union(_) => unimplemented!(),
    };
    let fields = match fields {
        syn::Fields::Named(n) => n.named,
        syn::Fields::Unnamed(_) => unimplemented!(),
        syn::Fields::Unit => unimplemented!(),
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
        impl #impl_generics atlas_lib::ui::MakeUi for #struct_name #type_generics #where_clause {
            fn make_ui(&mut self, ui: &mut bevy_egui::egui::Ui) -> Vec<usize> {
                let mut results = vec![];
                #(
                    results.push(atlas_lib::ui::#controls::new(ui, #labels, &mut self.#idents)#(.#all_funs)*.show(#hints));
                )*
                results
            }
        }
    })
}

#[proc_macro_derive(UiConfigurableEnum)]
pub fn ui_configurable_enum_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let enum_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    let variants = match ast.data {
        syn::Data::Enum(e) => e.variants,
        syn::Data::Struct(_) => unimplemented!(),
        syn::Data::Union(_) => unimplemented!(),
    };

    let len = variants.len();
    let mut idents = vec![];
    let indices: Vec<_> = (0..len).collect();
    for variant in variants.into_iter() {
        idents.push(variant.ident);
    }

    TokenStream::from(quote! {
        impl #impl_generics atlas_lib::ui::UiConfigurableEnum for #enum_name #type_generics #where_clause {
            const LEN: usize = #len;

            fn self_as_index(&self) -> usize {
                match self {
                    #(Self::#idents(_) => #indices,)*
                }
            }

            fn index_as_self(&self, idx: usize) -> Self {
                match idx {
                    #(#indices => Self::#idents(Default::default()),)*
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
