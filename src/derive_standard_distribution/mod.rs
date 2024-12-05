mod derive_field;
mod derive_variant;

use self::{derive_field::DeriveField, derive_variant::DeriveVariant};
use darling::{ast::Data, Error as DarlingError, FromDeriveInput, Result as DarlingResult};
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{parse_quote, Generics, Ident, ItemImpl, Stmt, WherePredicate};

#[derive(FromDeriveInput)]
#[darling(
	attributes(standard_distribution),
	supports(struct_any, enum_any),
	and_then = Self::check_and_correct
)]
pub struct DeriveData {
	ident: Ident,
	generics: Generics,

	// Yes, I'm aware that this is currently marked as only supporting structs. The Variant type
	// bound is specified here for later, when I'm ready to handle enums.
	data: Data<DeriveVariant, DeriveField>,
}

impl DeriveData {
	fn check_and_correct(self) -> DarlingResult<Self> {
		let mut error_accumulator = DarlingError::accumulator();
		let Self {
			ident,
			mut generics,
			data,
		} = self;

		// Check for erroneous conditions with the DeriveData
		if let Data::Enum(enum_data) = &data {
			// We cannot create an instance of an enum if it has zero variants.
			if enum_data.is_empty() {
				error_accumulator.push(DarlingError::custom(
					"Cannot derive StandardDistribution for enums with zero variants",
				))
			}

			// We cannot create an enum without choosing a variant.
			let is_all_weights_zero = enum_data.iter().all(|variant| variant.is_skipped());
			if is_all_weights_zero {
				error_accumulator.push(DarlingError::custom(
					"There must be at least one non-skipped variant",
				))
			}
		}

		// Prepare the WhereClause
		let type_param_idents: Vec<WherePredicate> = generics
			.type_params()
			.map(|tp| {
				let ident = &tp.ident;
				parse_quote! {
					::rand::distributions::Standard: ::rand::distributions::Distribution< #ident >
				}
			})
			.collect();

		generics
			.make_where_clause()
			.predicates
			.extend(type_param_idents);

		// We are finished checking and correcting
		error_accumulator.finish_with(Self {
			ident,
			generics,
			data,
		})
	}

	fn make_code(&self) -> DarlingResult<Vec<Stmt>> {
		let self_ident = &self.ident;
		match &self.data {
			Data::Enum(variants) => {
				self::derive_variant::generate_enum_sample_code(variants, self_ident)
			}
			Data::Struct(fields) => {
				let path = parse_quote! { #self_ident };
				let struct_expression =
					self::derive_field::fields_to_struct_expression(fields, path);
				Ok(parse_quote! {
					#struct_expression
				})
			}
		}
	}
}

impl ToTokens for DeriveData {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		let ident = &self.ident;
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let sample_code_res = self.make_code();

		match sample_code_res {
			Ok(code) => {
				let distr_impl: ItemImpl = parse_quote! {
					impl #impl_generics ::rand::distributions::Distribution< #ident #type_generics >
					for ::rand::distributions::Standard
					#where_clause
					{
						// Yes, the type param name is silly. I just need to make sure it won't conflict.
						fn sample<RngProvidedToThisMethod: ::rand::Rng + ?Sized>(
							&self,
							rng: &mut RngProvidedToThisMethod,
						) -> #ident #type_generics {
							#(#code);*
						}
					}
				};
				distr_impl.to_tokens(tokens);
			}
			Err(e) => tokens.extend(e.write_errors()),
		}
	}
}
