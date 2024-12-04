use darling::FromDeriveInput;
use proc_macro::TokenStream as TokenStream1;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

mod sample_uniform;
mod standard_distribution;
mod uniform_sampler;

/// Derive macro generating `impl Distribution<T> for Standard`, where `T` is the struct or enum
/// this is placed upon.
///
/// When placed on an enum, a random variant is chosen, then the fields on that variant are filled.
///
/// # Attribute Options
///
/// ## `#[standard_distribution(skip)]`
/// **Applicable to:** Enum variants, and fields within both enum variants and structs
///
/// * When attached to an **enum variant**, the variant will never be chosen. This has the same
///   effect as `#[standard_distribution(weight = 0)]`.
/// * When attached to a **struct field** or **enum variant field**, the field's value will be
///   generated from its [`Default`] implementation, instead of randomly.
///
/// ## `#[standard_distribution(weight = ...)]`
/// **Applicable to:** Enum variants
/// **Parameter type**: Int or float
///
/// When attached to an enum variant, this will set the weight for that variant. The higher a
/// variant's weight, the more likely it is to be chosen.
///
/// Due to the limitations of `WeightedIndex`, all weights must be of the same type - you cannot mix
/// floats and ints.
///
/// [`WeightedIndex::new`]: https://docs.rs/rand/0.8.5/rand/distributions/struct.WeightedIndex.html#method.new
#[proc_macro_derive(StandardDistribution, attributes(standard_distribution))]
pub fn derive_standard_distribution(input_item: TokenStream1) -> TokenStream1 {
	let derive_input = parse_macro_input!(input_item as DeriveInput);

	// Let's pass it to darling.
	let derive_data_result =
		self::standard_distribution::DeriveData::from_derive_input(&derive_input);

	// Finally, generate the output (whether that be an impl or a error)
	let output = match derive_data_result {
		Ok(v) => v.into_token_stream(),
		Err(e) => e.write_errors(),
	};
	output.into()
}

// parameters:
// * `#[sample_uniform(generate_uniform_sampler)]`
//   * generates a new `UniformSampler` in the same module, named `<T>UniformSampler` (replace `<T>`
//     with what this is attached to)
// * `#[sample_uniform(generate_uniform_sampler(name = XSampler))]`
//   * generates a new `UniformSampler` in the same module, with the given name
#[proc_macro_derive(SampleUniform)]
pub fn derive_sample_uniform(_input_item: TokenStream1) -> TokenStream1 {
	todo!()
}

// note: could we maybe have parameters that link specific fields in the uniform sampler, to the
// fields in the sampled type? i.e. `#[uniform_sampler(linked_field_name = "x")] x_gen: f32`
#[proc_macro_derive(UniformSampler)]
pub fn derive_uniform_sampler(_input_item: TokenStream1) -> TokenStream1 {
	todo!()
}
