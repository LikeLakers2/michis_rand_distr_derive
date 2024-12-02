use michis_rand_distr_derive::StandardDistribution;

#[derive(StandardDistribution)]
pub enum EnumIntegerWeights {
	#[standard_distribution(weight = 1)]
	Variant1,
	#[standard_distribution(weight = 3)]
	Variant2(f32),
	#[standard_distribution(weight = 2)]
	Variant3 { f: f32 },
}

#[derive(StandardDistribution)]
pub enum EnumFloatWeights {
	#[standard_distribution(weight = 1.0)]
	Variant1,
	#[standard_distribution(weight = 3.0)]
	Variant2(f32),
	#[standard_distribution(weight = 2.0)]
	Variant3 { f: f32 },
}

#[derive(StandardDistribution)]
pub enum EnumVariantSkip {
	Variant1,
	#[standard_distribution(skip)]
	Variant2,
	Variant3,
}

#[derive(StandardDistribution)]
pub struct FieldSkip {
	_field1: f32,
	_field2: f64,
	#[standard_distribution(skip)]
	_field3: String,
}

#[derive(StandardDistribution)]
pub enum AllButOneVariantSkip {
	Variant1,
	#[standard_distribution(skip)]
	Variant2,
	#[standard_distribution(skip)]
	Variant3,
}
