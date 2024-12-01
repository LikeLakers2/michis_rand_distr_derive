use darling::{ast::Fields, util::Flag, FromVariant};
use syn::{parse_quote, Expr, Ident};

use super::{derive_field::DeriveField, FieldsExt};

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
	weight: Option<Expr>,
}

impl DeriveVariant {
	pub(crate) fn make_struct_expression(&self, enum_name: &Ident) -> Expr {
		let variant_ident = &self.ident;
		let path = parse_quote! { #enum_name :: #variant_ident };
		self.fields.to_struct_expression(path)
	}
}
