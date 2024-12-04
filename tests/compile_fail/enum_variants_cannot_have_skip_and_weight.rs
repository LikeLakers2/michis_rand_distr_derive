use michis_rand_distr_derive::StandardDistribution;
fn main() {}

#[derive(StandardDistribution)]
pub enum ErrorTest1 {
	#[standard_distribution(skip)]
	#[standard_distribution(weight = 1.0)]
	Variant1,
	#[standard_distribution(weight = 3.0)]
	Variant2,
}

#[derive(StandardDistribution)]
pub enum ErrorTest2 {
	#[standard_distribution(skip)]
	#[standard_distribution(weight = 1.0)]
	Variant1,
	#[standard_distribution(skip)]
	#[standard_distribution(weight = 0.0)]
	Variant2,
}
