use michis_rand_distr_derive::StandardDistribution;
fn main() {}

#[derive(StandardDistribution)]
pub enum ErrorIfWeightArentNumbers1 {
	#[standard_distribution(weight = "1.0")]
	Variant1,
	#[standard_distribution(weight = 1.0)]
	Variant2,
	#[standard_distribution(weight = true)]
	Variant3,
	#[standard_distribution(weight = 5.0)]
	Variant4,
	#[standard_distribution(weight = "abcd")]
	Variant5,
}

#[derive(StandardDistribution)]
pub enum ErrorIfWeightArentNumbers2 {
	#[standard_distribution(weight = 1.0)]
	Variant1,
	#[standard_distribution(weight = "1.0")]
	Variant2,
	#[standard_distribution(weight = 5.0)]
	Variant3,
	#[standard_distribution(weight = true)]
	Variant4,
	#[standard_distribution(weight = 0.0)]
	Variant5,
}
