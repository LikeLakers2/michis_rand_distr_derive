use michis_rand_distr_derive::StandardDistribution;
fn main() {}

#[derive(StandardDistribution)]
pub enum ErrorsIfEnumHasAllZeroWeights1 {
	#[standard_distribution(weight = 0)]
	Variant,
}

#[derive(StandardDistribution)]
pub enum ErrorsIfEnumHasAllZeroWeights2 {
	#[standard_distribution(weight = 0)]
	Variant1,
	#[standard_distribution(weight = 0)]
	Variant2,
}

#[derive(StandardDistribution)]
pub enum ErrorsIfEnumHasAllZeroWeights3 {
	#[standard_distribution(weight = 0.0)]
	Variant1,
	#[standard_distribution(weight = 0.0)]
	Variant2,
}
