use darling::FromDeriveInput;
use proc_macro::TokenStream as TokenStream1;
use quote::ToTokens;
use syn::{parse_macro_input, Data, DeriveInput};

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
/// **Note**: If both the `skip` attribute and a `weight` are applied to an enum variant, the `skip`
/// attribute will take precedence, and the variant will be given a weight of zero.
///
/// [`WeightedIndex::new`]: https://docs.rs/rand/0.8.5/rand/distributions/struct.WeightedIndex.html#method.new
#[proc_macro_derive(StandardDistribution, attributes(standard_distribution))]
pub fn derive_standard_distribution(input_item: TokenStream1) -> TokenStream1 {
	let derive_input = parse_macro_input!(input_item as DeriveInput);

	// Match any erroneous conditions that can't be caught by darling, before passing it to darling.
	if let Data::Enum(enum_data) = &derive_input.data {
		if enum_data.variants.is_empty() {
			panic!("Cannot derive StandardDistribution for enums with zero variants");
		}
	}

	// Let's pass it to darling.
	let derive_data_result =
		self::standard_distribution::DeriveData::from_derive_input(&derive_input);

	// Finally, generate the output (whether that be an impl or a error)
	let output = match derive_data_result {
		Ok(mut v) => {
			v.prepare();
			v.into_token_stream()
		}
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
