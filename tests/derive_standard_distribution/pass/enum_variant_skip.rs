use michis_rand_distr_derive::StandardDistribution;
fn main() {}

#[derive(StandardDistribution)]
pub enum SingleVariantSkip {
	Variant1,
	#[standard_distribution(skip)]
	Variant2,
	Variant3,
}

#[derive(StandardDistribution)]
pub enum AllButOneVariantSkip {
	Variant1,
	#[standard_distribution(skip)]
	Variant2,
	#[standard_distribution(skip)]
	Variant3,
}
