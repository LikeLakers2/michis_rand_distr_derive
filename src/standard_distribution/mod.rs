mod derive_field;
mod derive_variant;

use self::{derive_field::DeriveField, derive_variant::DeriveVariant};
use darling::{
	ast::Data,
	FromDeriveInput,
};
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{parse_quote, Arm, ExprStruct, Generics, Ident, ItemImpl, Path, Stmt, WherePredicate};

#[derive(FromDeriveInput)]
#[darling(attributes(standard_distribution), supports(struct_any, enum_any))]
pub struct DeriveData {
	ident: Ident,
	generics: Generics,
	// Yes, I'm aware that this is currently marked as only supporting structs. The Variant type
	// bound is specified here for later, when I'm ready to handle enums.
	data: Data<DeriveVariant, DeriveField>,
}

impl DeriveData {
	pub(crate) fn prepare(&mut self) {
		self.prepare_where_clause();
	}

	fn prepare_where_clause(&mut self) {
		let type_param_idents: Vec<WherePredicate> = self
			.generics
			.type_params()
			.map(|tp| {
				let ident = &tp.ident;
				parse_quote! {
					::rand::distributions::Standard: ::rand::distributions::Distribution< #ident >
				}
			})
			.collect();

		self.generics
			.make_where_clause()
			.predicates
			.extend(type_param_idents);
	}

	fn make_code(&self) -> Vec<Stmt> {
		let self_ident = &self.ident;
		match &self.data {
			Data::Enum(variants) => {
				let variant_count = variants.len();
				let arms = variants.iter().enumerate().map::<Arm, _>(|(i, variant)| {
					let struct_expr = variant.make_struct_expression(self_ident);
					parse_quote! {
						#i => {
							#struct_expr
						}
					}
				});

				parse_quote! {
					match rng.gen_range(0..#variant_count) {
						#(#arms),*
						_ => unreachable!(),
					}
				}
			}
			Data::Struct(fields) => {
				let path = parse_quote! { #self_ident };
				let struct_expression = fields.to_struct_expression(path);
				parse_quote! {
					#struct_expression
				}
			}
		}
	}

	fn make_code_enum(&self) -> Vec<Stmt> {
		todo!()
	}
}

impl ToTokens for DeriveData {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		let ident = &self.ident;
		let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

		let sample_code = self.make_code();

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
					#(#sample_code);*
				}
			}
		};
		distr_impl.to_tokens(tokens);
	}
}

trait VecOfVariantsExt {}
trait FieldsExt {
	fn to_struct_expression(&self, struct_or_enum_path: Path) -> ExprStruct;
}
