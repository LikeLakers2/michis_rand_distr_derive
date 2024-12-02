use darling::{
	ast::Fields, util::Flag, Error as DarlingError, FromVariant, Result as DarlingResult,
};
use syn::{parse_quote, Arm, Expr, ExprStruct, Ident, Lit, Stmt};

use super::{FieldsExt as _, VecOfVariantsExt};

use super::derive_field::DeriveField;

impl VecOfVariantsExt for Vec<DeriveVariant> {
	fn generate_enum_sample_code(&self, enum_name: &Ident) -> DarlingResult<Vec<Stmt>> {
		let arms = self.iter().enumerate().map::<Arm, _>(|(i, variant)| {
			let struct_expr = variant.make_struct_expression(enum_name);
			parse_quote! {
				#i => {
					#struct_expr
				}
			}
		});

		let variant_chooser = self::generate_variant_chooser(self)?;

		Ok(parse_quote! {
			let chosen_variant = #variant_chooser;
			match chosen_variant {
				#(#arms),*
				_ => unreachable!(),
			}
		})
	}
}

fn generate_variant_chooser(variants: &[DeriveVariant]) -> DarlingResult<Expr> {
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
		return Ok(self::generate_variant_chooser_skips_only(variants));
	}

	// TODO: Bubble up a `darling::Error` if there are no choosable variants
	let variant_count = variants.len();
	Ok(parse_quote! {
		rng.gen_range(0..#variant_count)
	})
}

#[derive(PartialEq, Eq)]
enum WeightLitType {
	Int,
	Float,
	/// This weight has an invalid literal type. This is used because we want to bubble up as many
	/// errors as possible, to reduce the number of recompilations needed.
	Invalid,
}

impl WeightLitType {
	fn get_zero(&self) -> Lit {
		match self {
			Self::Int => parse_quote! { 0 },
			Self::Float => parse_quote! { 0.0 },
			Self::Invalid => parse_quote! { "invalid" },
		}
	}

	fn get_default(&self) -> Lit {
		match self {
			Self::Int => parse_quote! { 1 },
			Self::Float => parse_quote! { 1.0 },
			Self::Invalid => parse_quote! { "invalid" },
		}
	}
}

impl TryFrom<Lit> for WeightLitType {
	type Error = DarlingError;

	fn try_from(value: Lit) -> Result<Self, Self::Error> {
		match value {
			Lit::Int(_) => Ok(Self::Int),
			Lit::Float(_) => Ok(Self::Float),
			Lit::Str(_)
			| Lit::ByteStr(_)
			| Lit::CStr(_)
			| Lit::Byte(_)
			| Lit::Char(_)
			| Lit::Bool(_)
			| Lit::Verbatim(_) => {
				Err(Self::Error::custom("Weights must be a Int or Float literal").with_span(&value))
			}
			_ => Err(
				Self::Error::custom("Internal error: Unknown literal type provided")
					.with_span(&value),
			),
		}
	}
}

/// Finds the weight type of the first weight attribute.
///
/// We deliberately avoid checking if the weights are all of the same type. That will be done later
/// by the type-checker.
fn get_weight_type(variants: &[DeriveVariant]) -> DarlingResult<WeightLitType> {
	// Finds the first weight type specified.
	//
	// Note: The unwrap here will never panic - because it comes right after a filter to all Options
	// which have a `Some()` value.
	let first_weight_type = variants
		.iter()
		.map(|variant| &variant.weight)
		.filter(|&x| x.is_some())
		.cloned()
		.map(Option::unwrap)
		.next();

	if first_weight_type.is_none() {
		return Err(DarlingError::custom("Internal error: Attempted to get the weight type of an item that doesn't have any weights attached."));
	}

	// At this point, we know we have at least one weight type. This means it is safe to unwrap.
	//
	// Since we want a WeightLitType, we then attempt to convert it to one - and return the result,
	// whatever that might be.
	first_weight_type.unwrap().try_into()
}

fn generate_variant_chooser_weighted(variants: &[DeriveVariant]) -> DarlingResult<Expr> {
	let mut error_accumulator = DarlingError::accumulator();

	// We find the type of the first weight literal. This allows us to provide appropriately-typed
	// values when a variant is either marked as `skip`, or doesn't have a weight associated with
	// it.
	let default_weight_type = {
		let res = error_accumulator.handle(self::get_weight_type(variants));
		// Unfortunately, if the first weight is not a valid type, it could result in only showing
		// an error for that first weight.
		//
		// In those circumstances, we use `WeightLitType::Invalid`, which gives dummy values for the
		// default and zero values.
		res.unwrap_or(WeightLitType::Invalid)
	};

	// TODO: Bubble up a `darling::Error` if both `skip` and `weight` are specified

	let weights: Vec<_> = variants
		.iter()
		.map(|variant| {
			if variant.skip.is_present() {
				return default_weight_type.get_zero();
			}

			match variant.weight {
				None => default_weight_type.get_default(),
				Some(ref w) => {
					// Ensure that this Lit is a valid literal type, or accumulate an error if it
					// isn't.
					let lit_conversion =
						error_accumulator.handle(WeightLitType::try_from(w.clone()));
					match lit_conversion {
						// If the literal is an invalid type, we'll just toss the default value in.
						// This allows us to minimize the number of unnecessary errors, while still
						// ensuring we pass the type-checker.
						None => default_weight_type.get_default(),
						Some(_) => w.clone(),
					}
				}
			}
		})
		.collect();

	// TODO: Bubble up a `darling::Error` if all weights are zero

	error_accumulator.finish().map(|_| {
		parse_quote! {
			::rand::distributions::WeightedIndex::new(&[
				#(#weights),*
			]).unwrap().sample(rng)
		}
	})
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
