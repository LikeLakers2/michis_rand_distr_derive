use syn::parse_macro_input;

mod standard;
mod uniform;

#[proc_macro_derive(Standard)]
pub fn derive_standard_distribution(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input);
	self::standard::derive(input).into()
}

#[proc_macro_derive(Uniform)]
pub fn derive_uniform_distribution(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input);
	self::uniform::derive(input).into()
}
