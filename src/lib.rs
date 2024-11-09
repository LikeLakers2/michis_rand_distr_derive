use darling::{ast::NestedMeta, Error as DarlingError, FromMeta};
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{parse_macro_input, Item};

mod standard;
mod uniform;

#[proc_macro_attribute]
pub fn add_standard_distribution_support(
	attr_args_tok: TokenStream1,
	item_tok: TokenStream1,
) -> TokenStream1 {
	let item = parse_macro_input!(item_tok as Item);
	// We only support structs currently
	if !matches!(item, Item::Struct(_)) {
		return DarlingError::custom(
			"`#[add_standard_distribution_support]` is only supported for structs",
		)
		.write_errors()
		.into();
	}

	let resulting_impl = {
		let nested_meta = match NestedMeta::parse_meta_list(attr_args_tok.into()) {
			Ok(v) => v,
			Err(e) => return DarlingError::from(e).write_errors().into(),
		};
		match self::standard::Options::from_list(&nested_meta) {
			Ok(v) => v.create_distribution_impl(&item),
			Err(e) => return e.write_errors().into(),
		}
	};

	let mut ts = TokenStream2::new();
	item.to_tokens(&mut ts);
	resulting_impl.to_tokens(&mut ts);
	ts.into()
}

#[proc_macro_attribute]
pub fn add_uniform_distribution_support(
	_attr_args_tok: TokenStream1,
	_item_tok: TokenStream1,
) -> TokenStream1 {
	todo!()
}
