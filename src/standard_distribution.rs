use core::num::NonZeroUsize;

use darling::{
	ast::{Data, Fields},
	util::Flag,
	FromDeriveInput, FromField, FromVariant,
};
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{
	parse_quote, Arm, Expr, Generics, Ident, ItemImpl, Member, Path, Stmt, Type, WherePredicate,
};

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
				let struct_expression = self::fields_to_struct_expression(path, fields);
				parse_quote! {
					#struct_expression
				}
			}
		}
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

#[derive(FromVariant)]
#[darling(attributes(standard_distribution))]
pub struct DeriveVariant {
	ident: Ident,
	fields: Fields<DeriveField>,

	/// If specified, this variant will never be chosen when choosing a random variant.
	// TODO: Implement this
	_skip: Flag,
	/// If specified, sets the weight for this variant to be chosen. `1` is the base, `2` is twice
	/// as likely, and `5` is five times as likely.
	///
	/// If unspecified, the weight for this variant will be `1`.
	// TODO: Implement this
	// TODO: Probably change this to a float weight at some point?
	_weight: Option<NonZeroUsize>,
}

impl DeriveVariant {
	fn make_struct_expression(&self, enum_name: &Ident) -> Expr {
		let variant_ident = &self.ident;
		let path = parse_quote! { #enum_name :: #variant_ident };
		self::fields_to_struct_expression(path, &self.fields)
	}
}

#[derive(FromField)]
#[darling(attributes(standard_distribution))]
pub struct DeriveField {
	ident: Option<Ident>,
	ty: Type,

	/// If specified, this field will never be randomly generated. Instead, it will be generated
	/// using the field type's `Default` impl.
	// TODO: Implement this
	_skip: Flag,
}

impl DeriveField {
	fn make_rng_call(&self) -> Expr {
		let ty = &self.ty;
		parse_quote! {
			rng.gen::< #ty >()
		}
	}
}

fn fields_to_struct_expression(path: Path, fields: &Fields<DeriveField>) -> Expr {
	let field_names_iter = fields
		.iter()
		.enumerate()
		.map::<Member, _>(|(i, field)| field.ident.clone().map_or(i.into(), Into::into));
	let field_rng_calls = fields.iter().map(|field| field.make_rng_call());

	parse_quote! {
		#path {
			#(#field_names_iter : #field_rng_calls),*
		}
	}
}
