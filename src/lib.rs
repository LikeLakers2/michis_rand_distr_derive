use darling::FromDeriveInput;
use proc_macro::TokenStream as TokenStream1;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

mod derive_sample_uniform;
mod derive_standard_distribution;
mod derive_uniform_sampler;
mod generate_uniform_sampler;

/// Generates a `impl Distribution<T> for Standard` for the input item.
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
/// **Parameter type:** Int or float
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
		self::derive_standard_distribution::DeriveData::from_derive_input(&derive_input);

	// Finally, generate the output (whether that be an impl or a error)
	let output = match derive_data_result {
		Ok(v) => v.into_token_stream(),
		Err(e) => e.write_errors(),
	};
	output.into()
}

/// Generates an `impl SampleUniform for T` for the input item. Optionally also generates a uniform
/// sampler based on the input item, if the user desires.
// parameters:
// * `#[sample_uniform(sampler_path = XSampler)]`
//   * Points this SampleUniform trait to a specific sampler, via its path.
//   * If unspecified, the sampler path will be `self::<T>UniformSampler`, where `<T>` is the input
//     item's name. This means that if you attach both `#[derive(SampleUniform)]` and
//     `#[generate_sampler_uniform]` to an item, it will already be linked.
#[proc_macro_derive(SampleUniform, attributes(sample_uniform))]
pub fn derive_sample_uniform(_input_item: TokenStream1) -> TokenStream1 {
	todo!()
}

// note: could we maybe have parameters that link specific fields in the uniform sampler, to the
// fields in the sampled type? i.e. `#[uniform_sampler(linked_field_name = "x")] x_gen: f32`
#[proc_macro_derive(UniformSampler, attributes(uniform_sampler))]
pub fn derive_uniform_sampler(_input_item: TokenStream1) -> TokenStream1 {
	todo!()
}

/// Generates a UniformSampler based on the input item.
///
/// The UniformSampler is placed in the same module as the input item.
// parameters:
// * **OPTIONAL:** `#[generate_uniform_sampler(name = "XSampler")]`
//   * Sets the name of the generated uniform sampler.
//   * If not specified, the name of the generated uniform sampler will be `<T>UniformSampler`,
//     where `<T>` is the input item's name.
#[proc_macro_attribute]
pub fn generate_uniform_sampler(
	_input_attrs: TokenStream1,
	_input_item: TokenStream1,
) -> TokenStream1 {
	todo!()
}
