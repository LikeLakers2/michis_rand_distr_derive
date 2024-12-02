use michis_rand_distr_derive::StandardDistribution;
fn main() {}

#[derive(StandardDistribution)]
pub enum ErrorTest1 {
	#[standard_distribution(skip)]
	Variant,
}

#[derive(StandardDistribution)]
pub enum ErrorTest2 {
	#[standard_distribution(skip)]
	Variant1,
	#[standard_distribution(skip)]
	Variant2,
}
