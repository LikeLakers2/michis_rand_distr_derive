use darling::FromDeriveInput;
use proc_macro::TokenStream as TokenStream1;
use quote::ToTokens;
use syn::{parse_macro_input, Data, DeriveInput};

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

trait VecOfVariantsExt {
	fn generate_enum_sample_code(&self, enum_name: &syn::Ident) -> Vec<syn::Stmt>;
}
trait FieldsExt {
	fn to_struct_expression(&self, struct_or_enum_path: syn::Path) -> syn::ExprStruct;
}
