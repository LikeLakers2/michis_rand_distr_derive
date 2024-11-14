use core::num::NonZeroUsize;

use darling::{
	ast::{Data, Fields},
	util::Flag,
	FromDeriveInput, FromField, FromVariant,
};
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{Generics, Ident, Type};

#[derive(FromDeriveInput)]
#[darling(attributes(standard_distribution), supports(struct_any))]
pub struct DeriveData {
	ident: Ident,
	generics: Generics,
	// Yes, I'm aware that this is currently marked as only supporting structs. The Variant type
	// bound is specified here for later, when I'm ready to handle enums.
	data: Data<DeriveVariant, DeriveField>,
}

impl ToTokens for DeriveData {
	fn to_tokens(&self, tokens: &mut TokenStream2) {}
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
