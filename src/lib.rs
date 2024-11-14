use darling::FromDeriveInput;
use proc_macro::TokenStream as TokenStream1;
use quote::ToTokens;
use syn::parse_macro_input;

mod sample_uniform;
mod standard_distribution;
mod uniform_sampler;

/// Derive macro generating `impl Distribution<T> for Standard` on the struct or enum this is
/// placed upon.
///
/// # Notes
/// When this derive is placed on an enum, the resulting code will choose a random variant, then
/// randomly generate the fields.
#[proc_macro_derive(StandardDistribution, attributes(standard_distribution))]
pub fn derive_standard_distribution(input_item: TokenStream1) -> TokenStream1 {
	let derive_input = parse_macro_input!(input_item);
	let derive_data_result =
		self::standard_distribution::DeriveData::from_derive_input(&derive_input);
	let output = match derive_data_result {
		Ok(v) => v.into_token_stream(),
		Err(e) => e.write_errors(),
	};
	output.into()
}

#[proc_macro_derive(SampleUniform)]
pub fn derive_sample_uniform(_input_item: TokenStream1) -> TokenStream1 {
	todo!()
}

#[proc_macro_derive(UniformSampler)]
pub fn derive_uniform_sampler(_input_item: TokenStream1) -> TokenStream1 {
	todo!()
}
