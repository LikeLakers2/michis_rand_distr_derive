use darling::{ast::Fields, util::Flag, FromVariant};
use itertools::Itertools as _;
use syn::{parse_quote, Arm, Expr, ExprStruct, Ident, Lit, Stmt};

use crate::{FieldsExt as _, VecOfVariantsExt};

use super::derive_field::DeriveField;

impl VecOfVariantsExt for Vec<DeriveVariant> {
	fn generate_enum_sample_code(&self, enum_name: &Ident) -> Vec<Stmt> {
		let arms = self.iter().enumerate().map::<Arm, _>(|(i, variant)| {
			let struct_expr = variant.make_struct_expression(enum_name);
			parse_quote! {
				#i => {
					#struct_expr
				}
			}
		});

		let variant_chooser = self::generate_variant_chooser(self);

		parse_quote! {
			let chosen_variant = #variant_chooser;
			match chosen_variant {
				#(#arms),*
				_ => unreachable!(),
			}
		}
	}
}

fn generate_variant_chooser(variants: &[DeriveVariant]) -> Expr {
	// Possible scenarios:
	// * One or more weighted variants, and no skips (use `WeightedIndex`)
	// * One or more weighted variants, with skips (use `WeightedIndex`, with skips = 0.0)
	// * One or more skips, with no weighted variants (use `SliceRandom`)
	// * No weighted variants and no skips (use `rng.gen_range()`)
	// * All variants except one are skipped (generate that variant's index)
	// * SHOULD FAIL: All weighted variants equal 0
	// * SHOULD FAIL: All variants are skipped

	// I feel like I may have prematurely optimized... Although, these optimizations produce the
	// same result, just with Rust's compiler having to do less work.

	let has_weighted_variant = variants.iter().any(|variant| variant.weight.is_some());
	if has_weighted_variant {
		return self::generate_variant_chooser_weighted(variants);
	}

	let has_skip = variants.iter().any(|variant| variant.skip.is_present());
	if has_skip {
		return self::generate_variant_chooser_skips_only(variants);
	}

	// TODO: Bubble up a `darling::Error` if there are no choosable variants
	let variant_count = variants.len();
	parse_quote! {
		rng.gen_range(0..#variant_count)
	}
}

fn generate_variant_chooser_weighted(variants: &[DeriveVariant]) -> Expr {
	let first_weighted_variant = variants
		.iter()
		.find(|variant| variant.weight.is_some())
		.unwrap();
	let (zero_weight, default_weight): (Lit, Lit) = {
		// We can safely unwrap this because we just found this to have a weight.
		let weight = first_weighted_variant.weight.clone().unwrap();
		match weight {
			Lit::Int(_) => (parse_quote! { 0 }, parse_quote! { 1 }),
			Lit::Float(_) => (parse_quote! { 0.0 }, parse_quote! { 1.0 }),
			Lit::Str(_)
			| Lit::ByteStr(_)
			| Lit::CStr(_)
			| Lit::Byte(_)
			| Lit::Char(_)
			| Lit::Bool(_)
			| Lit::Verbatim(_) => panic!("Weights must be an Int or Float literal"),
			_ => panic!("Unknown literal type"),
			// TODO: The above panics should be a `darling::Error`
		}
	};

	// TODO: Bubble up a `darling::Error` if the weights aren't all the same type
	// TODO: Bubble up a `darling::Error` if both `skip` and `weight` are specified

	let weights = variants.iter().map(|variant| {
		if variant.skip.is_present() {
			zero_weight.clone()
		} else {
			variant.weight.clone().unwrap_or(default_weight.clone())
		}
	});

	// TODO: Bubble up a `darling::Error` if all weights are zero

	parse_quote! {
		::rand::distributions::WeightedIndex::new(&[
			#(#weights),*
		]).unwrap().sample(rng)
	}
}

fn generate_variant_chooser_skips_only(variants: &[DeriveVariant]) -> Expr {
	let choosable_variants: Vec<_> = variants
		.iter()
		.enumerate()
		.filter_map(|(i, variant)| (!variant.skip.is_present()).then_some(i))
		.collect();

	match choosable_variants.len() {
		0 => panic!("No choosable variants"),
		1 => {
			// We can provide a small optimization: If there is only one variant that isn't skipped,
			// then we can simply select that index.

			// We know there's items in this list, so we can safely unwrap.
			let single_variant = choosable_variants.first().unwrap();
			parse_quote! { #single_variant }
		}
		_ => {
			// NOTE: We must make an ExprBlock here, as SliceRandom has no way to call its functions
			// without a use statement.
			parse_quote! {
				{
					use ::rand::seq::SliceRandom;
					[#(#choosable_variants),*].choose(rng).unwrap()
				}
			}
		}
	}
}

/// When an enum derives StandardDistribution, the variant returned will be randomly chosen. The
/// below parameters affect this randomization.
#[derive(FromVariant)]
#[darling(attributes(standard_distribution))]
pub struct DeriveVariant {
	ident: Ident,
	fields: Fields<DeriveField>,

	/// If specified, this variant will never be chosen when choosing a random variant.
	skip: Flag,
	/// Sets the weight for this variant. A higher weight means this variant is more likely to be
	/// chosen. If unspecified, the weight for this variant will be `1.0`.
	///
	/// This parameter can be any type that implements `rand::distributions::uniform::SampleUniform`
	/// and [`PartialOrd`]. However, the parameter must be the same type across all usages of this
	/// parameter. For example, you cannot use `weight = 1.0` and `weight = 3` in the same enum.
	weight: Option<Lit>,
}

impl DeriveVariant {
	pub(crate) fn make_struct_expression(&self, enum_name: &Ident) -> ExprStruct {
		let variant_ident = &self.ident;
		let path = parse_quote! { #enum_name :: #variant_ident };
		self.fields.to_struct_expression(path)
	}
}
