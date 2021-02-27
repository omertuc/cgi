#![recursion_limit = "128"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use syn::DeriveInput;

#[proc_macro_derive(VertexAttribPointers, attributes())]
pub fn vertex_attrib_pointers_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    let generics = &input.generics;
    let where_clause = &input.generics.where_clause;

    let fields_vertex_attrib_pointer = generate_vertex_attrib_pointer_calls(&input.data);

    let expanded = quote! {
        impl #ident #generics #where_clause {
            fn vertex_attrib_pointers(gl: &gl::Gl) {
                let stride = std::mem::size_of::<Self>();
                let mut offset = 0;
                let mut location = 0;

                #fields_vertex_attrib_pointer
            }
        }
    };

    expanded.into()
}

fn generate_vertex_attrib_pointer_calls(data: &syn::Data) -> proc_macro2::TokenStream {
    match data {
        &syn::Data::Struct(ref s) => {
            s.fields.iter().map(generate_struct_field_vertex_attrib_pointer_call).collect()
        }
        &syn::Data::Enum(ref _s) => {
            panic!("Enum vertex attrib pointers derivation unsupported")
        }
        &syn::Data::Union(ref _s) => {
            panic!("Union vertex attrib pointers derivation unsupported")
        }
    }
}

fn generate_struct_field_vertex_attrib_pointer_call(field: &syn::Field) -> proc_macro2::TokenStream {
    let field_ty = &field.ty;

    quote! {
        unsafe {
            #field_ty::vertex_attrib_pointer(gl, stride, location, offset);
        }

        offset += std::mem::size_of::<#field_ty>();
        location += 1;
    }
}

