
use proc_macro2::Ident;
use syn::{parse_macro_input, __private::TokenStream, DeriveInput, Generics, DataStruct, DataEnum};
use quote::{quote};

#[proc_macro_derive(BorshSize)]
pub fn derive_data_size(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        syn::Data::Struct(ds) => derive_for_struct(input.ident, input.generics, ds),
        syn::Data::Enum(de) => derive_for_enum(input.ident, input.generics, de),
        syn::Data::Union(_) => panic!("unions not supported"),
    }
}


fn derive_for_struct(name: Ident, generics: Generics, ds: DataStruct) -> TokenStream { 
    
    let mut size = proc_macro2::TokenStream::new();
    size.extend(quote!(0));

    for (_idx, field) in ds.fields.iter().enumerate() { 
        let name = &field.ident;
        size.extend(quote!(
            + &self.#name.calculate_borsh_size()
        ));
      
    }


    TokenStream::from(quote! {
        impl borsh_size::BorshSize for #name #generics {

            fn calculate_borsh_size(&self) -> usize {
                #size
            }

        }
    })
}

fn derive_for_enum(name: Ident, generics: Generics, ds: DataEnum) -> TokenStream { 

    let mut enum_variant_options = proc_macro2::TokenStream::new();
    for variant in ds.variants.into_iter() { 
        let variant_ident = variant.ident;
        let mut field_match = proc_macro2::TokenStream::new();
        let mut field_calc = proc_macro2::TokenStream::from(quote!(0));

        match variant.fields {
            syn::Fields::Named(fields) => {

                let mut field_match_parts = proc_macro2::TokenStream::new();
                for field in fields.named.into_iter() {
                    let field_name = field.ident;
                    
                    field_match_parts.extend(quote!(
                        #field_name,
                    ));

                    field_calc.extend(quote! {
                        + #field_name.calculate_borsh_size()
                    });
                }
                if !field_match_parts.is_empty() {
                    field_match.extend(quote! {
                        {#field_match_parts}
                    });
                }

            },

            syn::Fields::Unnamed(_) => todo!(),

            syn::Fields::Unit => todo!(),
        }

    
        enum_variant_options.extend(quote!(
            #name::#variant_ident #field_match => { #field_calc }
        ));   
    }


    TokenStream::from(quote! {
        impl borsh_size::BorshSize for #name #generics {

            fn calculate_borsh_size(&self) -> usize {
                match self {
                    #enum_variant_options
                }
            }

        }
    })
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}