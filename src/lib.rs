use syn::parse_macro_input;

mod standard;
mod uniform;

#[proc_macro_derive(StandardDistr)]
pub fn derive_standard_distribution(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input);
	self::standard::derive(input).into()
}

#[proc_macro_derive(UniformDistr)]
pub fn derive_uniform_distribution(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input);
	self::uniform::derive(input).into()
}
