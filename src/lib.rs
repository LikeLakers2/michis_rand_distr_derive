use darling::{ast::NestedMeta, Error as DarlingError, FromMeta};
use proc_macro::TokenStream as TokenStream1;
use syn::parse_macro_input;

mod sample_uniform;
mod standard_distribution;
mod uniform_sampler;

#[proc_macro_attribute]
pub fn add_standard_distribution_support(
	attr_args_tok: TokenStream1,
	item_tok: TokenStream1,
) -> TokenStream1 {
	let item = parse_macro_input!(item_tok);
	let attr_args = match NestedMeta::parse_meta_list(attr_args_tok.into()) {
		Ok(v) => v,
		Err(e) => return DarlingError::from(e).write_errors().into(),
	};
	self::standard_distribution::derive(attr_args, item).into()
}

#[proc_macro_attribute]
pub fn add_uniform_distribution_support(
	_attr_args_tok: TokenStream1,
	_item_tok: TokenStream1,
) -> TokenStream1 {
	todo!()
}
