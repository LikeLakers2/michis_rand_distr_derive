use darling::FromDeriveInput;
use proc_macro::TokenStream as TokenStream1;
use syn::parse_macro_input;

mod sample_uniform;
mod standard_distribution;
mod uniform_sampler;

#[proc_macro_derive(StandardDistribution)]
pub fn derive_standard_distribution(
	input_item: TokenStream1,
) -> TokenStream1 {
	let derive_input = parse_macro_input!(input_item);
	let derive_opts = match self::standard_distribution::DeriveData::from_derive_input(&derive_input) {
		Ok(v) => v,
		Err(e) => return e.write_errors().into(),
	};
	derive_opts.do_derive().into()
}

#[proc_macro_derive(SampleUniform)]
pub fn derive_sample_uniform(
	_input_item: TokenStream1,
) -> TokenStream1 {
	todo!()
}

#[proc_macro_derive(UniformSampler)]
pub fn derive_uniform_sampler(
	_input_item: TokenStream1,
) -> TokenStream1 {
	todo!()
}
