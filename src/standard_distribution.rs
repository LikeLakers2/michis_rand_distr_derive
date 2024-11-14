use darling::FromDeriveInput;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Generics, Ident};

#[derive(Debug, FromDeriveInput)]
pub struct DeriveData {
	ident: Ident,
	generics: Generics,
}

impl DeriveData {
	pub fn do_derive(&self) -> TokenStream2 {
		TokenStream2::new()
	}

	/*
	pub fn create_distribution_impl(&self, item: &Item) -> ItemImpl {
		let sample_method_code = self.generate_sample_method(item);

		match item {
			Item::Struct(s) => {
				let struct_name = &s.ident;
				let trait_bounds_added = self.add_trait_bounds_to_generics(&s.generics);
				let (impl_generics, ty_generics, where_clause) =
					trait_bounds_added.split_for_impl();

				parse_quote! {
					impl #impl_generics ::rand::distributions::Distribution<#struct_name #ty_generics>
					for ::rand::distributions::Standard #where_clause {
						fn sample<R: ::rand::Rng + ?Sized>(&self, rng: &mut R) -> #struct_name #ty_generics {
							#sample_method_code
						}
					}
				}
			}
			Item::Enum(_e) => unimplemented!(),
			_ => unreachable!(),
		}
	}

	fn add_trait_bounds_to_generics(&self, generics: &Generics) -> Generics {
		let mut generics = generics.clone();

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

	fn generate_sample_method(&self, item: &Item) -> ExprStruct {
		match item {
			Item::Struct(s) => {
				let name = &s.ident;
				let field_values_iter = self.fields_to_field_values(&s.fields).into_iter();
				parse_quote! {
					#name {
						#(#field_values_iter),*
					}
				}
			}
			Item::Enum(_e) => unimplemented!(),
			_ => unreachable!(),
		}
	}

	fn fields_to_field_values(&self, fields: &Fields) -> impl IntoIterator<Item = FieldValue> {
		fields
			.iter()
			.enumerate()
			.map::<FieldValue, _>(|(num, field)| {
				let member_name: Member = field.ident.clone().map_or(num.into(), Into::into);
				let ty = &field.ty;

				parse_quote! {
					#member_name: rng.gen::<#ty>()
				}
			})
			.collect::<Vec<_>>()
	}
	*/
}
