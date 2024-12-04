use darling::{
	ast::Fields,
	util::{Flag, SpannedValue},
	Error as DarlingError, FromVariant, Result as DarlingResult,
};
use syn::{parse_quote, Arm, Expr, ExprStruct, Ident, Lit, Stmt};

use super::{FieldsExt as _, VecOfVariantsExt};

use super::derive_field::DeriveField;

/// When an enum derives StandardDistribution, the variant returned will be randomly chosen. The
/// below parameters affect this randomization.
#[derive(FromVariant)]
#[darling(attributes(standard_distribution), and_then = Self::check_and_correct)]
pub struct DeriveVariant {
	ident: Ident,
	fields: Fields<DeriveField>,

	/// If specified, this variant will never be chosen when choosing a random variant.
	// NOTE: We are forced to wrap this flag in a SpannedValue. This is because there is no other
	// way to set/unset a `Flag` - something we do within `Self::check_and_correct` when
	// `self.weight` is a Lit with a zero-like value - without losing the span information.
	//
	// Unfortunately, SpannedValue also doesn't inherit the lack of need for `darling(default)`, so
	// we have to tag it as such.
	#[darling(default)]
	skip: SpannedValue<Flag>,
	/// Sets the weight for this variant. A higher weight means this variant is more likely to be
	/// chosen. If unspecified, the weight for this variant will be `1.0`.
	///
	/// This parameter can be any type that implements `rand::distributions::uniform::SampleUniform`
	/// and [`PartialOrd`]. However, the parameter must be the same type across all usages of this
	/// parameter. For example, you cannot use `weight = 1.0` and `weight = 3` in the same enum.
	weight: Option<Lit>,

	// --- Data populated during `Self::check_and_correct` --- //
	#[darling(skip)]
	weight_type: Option<WeightLitType>,
}

impl DeriveVariant {
	fn check_and_correct(self) -> DarlingResult<Self> {
		let mut error_accumulator = DarlingError::accumulator();
		let Self {
			ident,
			fields,
			mut skip,
			mut weight,
			mut weight_type,
		} = self;

		// It is an error if both `weight` and `skip` are present on a variant.
		if weight.is_some() && skip.is_present() {
			error_accumulator.push(
				DarlingError::custom("skip and weight may not be specified together")
					.with_span(&skip.span()),
			);
		}

		// If a weight literal is specified, we want to note down the weight type in a separate
		// variable. We can calculate this later, but the weight type gets used a lot - so it's
		// easier to do this now.
		//
		// If the weight literal is not of a valid type, we will instead propagate an error.
		if let Some(inner_lit) = weight.clone() {
			weight_type = error_accumulator.handle(WeightLitType::try_from(inner_lit));
		}

		// If the weight literal is a valid type, then we can check if the weight is a zero-like
		// value. If so, erase `weight` and set `skip`, as a zero-like value in weight has the same
		// effect.
		if let Some(wty) = &weight_type {
			let zero_lit_opt = weight.take_if(|inner| *inner == wty.get_zero());
			if let Some(zero_lit) = zero_lit_opt {
				skip = SpannedValue::new(Flag::present(), zero_lit.span());
			}
		}

		error_accumulator.finish_with(Self {
			ident,
			fields,
			skip,
			weight,
			weight_type,
		})
	}

	pub(crate) fn make_struct_expression(&self, enum_name: &Ident) -> ExprStruct {
		let variant_ident = &self.ident;
		let path = parse_quote! { #enum_name :: #variant_ident };
		self.fields.to_struct_expression(path)
	}

	pub fn is_skipped(&self) -> bool {
		self.skip.is_present()
	}
}

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

		let variant_chooser = self.generate_variant_chooser()?;

		Ok(parse_quote! {
			let chosen_variant = #variant_chooser;
			match chosen_variant {
				#(#arms),*
				_ => unreachable!(),
			}
		})
	}

	fn generate_variant_chooser(&self) -> DarlingResult<Expr> {
		// If we have any weighted variants, generate a variant chooser that uses weights.
		let first_weight_type = self.iter().filter_map(|variant| variant.weight_type).next();
		if let Some(default_weight_type) = first_weight_type {
			let weights: Vec<_> = self
				.iter()
				.map(|variant| {
					// If skip was specified (or if the user specified a zero-value weight), then we
					// make this variant's weight a zero-value of the default weight type.
					if variant.skip.is_present() {
						return default_weight_type.get_zero();
					}

					// If a weight type is set on this variant, then that means there's also a
					// weight set. In that case, we can safely unwrap the weight.
					//
					// Otherwise, use a default-value.
					match variant.weight_type {
						None => default_weight_type.get_default(),
						Some(_) => variant.weight.clone().unwrap(),
					}
				})
				.collect();

			let res = Ok(parse_quote! {
				::rand::distributions::WeightedIndex::new(&[
					#(#weights),*
				]).unwrap().sample(rng)
			});

			return res;
		}

		// Otherwise, choose our generator based on how many choosable variants there are.
		let choosable_variants: Vec<_> = self
			.iter()
			.enumerate()
			.filter_map(|(i, variant)| (!variant.skip.is_present()).then_some(i))
			.collect();
		match choosable_variants.len() {
			// If we have zero choosable variants, then we've messed up somewhere else (likely
			// within `DeriveVariant::check_and_correct`). In any case, we should panic here.
			0 => unreachable!("Internal error: Attempted to generate a variant chooser without any choosable variants"),

			// If we have only one choosable variant, then we can perform a small optimization: Just
			// choose that index!
			1 => {
				// We know there's items in this list, so we can safely unwrap.
				let single_variant = choosable_variants.first().unwrap();
				Ok(parse_quote! { #single_variant })
			}

			// If all variants are unskipped, then we don't need to generate anything more than a
			// single index.
			count if count == self.len() => {
				Ok(parse_quote! {
					rng.gen_range(0..#count)
				})
			}

			// Otherwise, we know our number of unskipped variants is not equal to our number of
			// variants. In this case, we choose from a list of unskipped variants.
			_ => {
				// NOTE: We must make an ExprBlock here, as SliceRandom has no way to call its
				// functions without a use statement.
				Ok(parse_quote! {
					{
						use ::rand::seq::SliceRandom;
						[#(#choosable_variants),*].choose(rng).unwrap()
					}
				})
			}
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WeightLitType {
	Int,
	Float,
}

impl WeightLitType {
	fn get_zero(&self) -> Lit {
		match self {
			Self::Int => parse_quote! { 0 },
			Self::Float => parse_quote! { 0.0 },
		}
	}

	fn get_default(&self) -> Lit {
		match self {
			Self::Int => parse_quote! { 1 },
			Self::Float => parse_quote! { 1.0 },
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
