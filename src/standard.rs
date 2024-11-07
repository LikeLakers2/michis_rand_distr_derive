use proc_macro2::TokenStream;
use quote::format_ident;
use syn::{
	parse_quote, Data, DeriveInput, Expr, ExprStruct, FieldValue, GenericParam, Generics, Type,
	WherePredicate,
};

pub fn derive(input: DeriveInput) -> TokenStream {
	let sample_method_code = self::generate_sample_method(&input);

	let struct_name = input.ident;
	let trait_bounds_added = self::add_trait_bounds(input.generics);
	let (impl_generics, ty_generics, where_clause) = trait_bounds_added.split_for_impl();

	parse_quote! {
		impl #impl_generics ::rand::distributions::Distribution<#struct_name #ty_generics> for ::rand::distributions::Standard #where_clause {
			fn sample<R: ::rand::Rng + ?Sized>(&self, rng: &mut R) -> #struct_name #ty_generics {
				#sample_method_code
			}
		}
	}
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
	let type_params_bounded: Vec<WherePredicate> = generics
		.params
		.iter()
		.filter_map(|param| {
			if let GenericParam::Type(type_param) = param {
				let ident = &type_param.ident;
				Some(parse_quote! {
					::rand::distributions::Standard: ::rand::distributions::Distribution< #ident >
				})
			} else {
				None
			}
		})
		.collect();

	generics
		.make_where_clause()
		.predicates
		.extend(type_params_bounded);

	generics
}

fn generate_sample_method(input: &DeriveInput) -> ExprStruct {
	let struct_ident = input.ident.clone();
	match &input.data {
		Data::Struct(data_struct) => {
			let field_gen =
				data_struct
					.fields
					.iter()
					.enumerate()
					.map::<FieldValue, _>(|(num, field)| {
						let ident = format_ident!(
							"{}",
							field
								.ident
								.clone()
								.map_or(num.to_string(), |name| name.to_string())
						);
						let ty = &field.ty;

						parse_quote! {
							#ident: rng.gen::<#ty>()
						}
					});

			parse_quote! {
				#struct_ident {
					#(#field_gen),*
				}
			}
		}
		Data::Enum(_data_enum) => unimplemented!(),
		Data::Union(_data_union) => unimplemented!(),
	}
}
