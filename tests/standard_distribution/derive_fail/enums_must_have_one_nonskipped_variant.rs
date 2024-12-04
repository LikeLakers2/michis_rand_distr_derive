use michis_rand_distr_derive::StandardDistribution;
fn main() {}

#[derive(StandardDistribution)]
pub enum ErrorTestSkip1 {
	#[standard_distribution(skip)]
	Variant,
}

#[derive(StandardDistribution)]
pub enum ErrorTestSkip2 {
	#[standard_distribution(skip)]
	Variant1,
	#[standard_distribution(skip)]
	Variant2,
}

#[derive(StandardDistribution)]
pub enum ErrorTestWeight1 {
	#[standard_distribution(weight = 0)]
	Variant,
}

#[derive(StandardDistribution)]
pub enum ErrorTestWeight2 {
	#[standard_distribution(weight = 0)]
	Variant1,
	#[standard_distribution(weight = 0)]
	Variant2,
}

#[derive(StandardDistribution)]
pub enum ErrorTestWeight3 {
	#[standard_distribution(weight = 0.0)]
	Variant1,
	#[standard_distribution(weight = 0.0)]
	Variant2,
}
