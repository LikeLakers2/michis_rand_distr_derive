use michis_rand_distr_derive::StandardDistribution;
fn main() {}

#[derive(StandardDistribution)]
pub enum ErrorTest1 {
	#[standard_distribution(weight = 0)]
	Variant,
}

#[derive(StandardDistribution)]
pub enum ErrorTest2 {
	#[standard_distribution(weight = 0)]
	Variant1,
	#[standard_distribution(weight = 0)]
	Variant2,
}

#[derive(StandardDistribution)]
pub enum ErrorTest3 {
	#[standard_distribution(weight = 0.0)]
	Variant1,
	#[standard_distribution(weight = 0.0)]
	Variant2,
}
