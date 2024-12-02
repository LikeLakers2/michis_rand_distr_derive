use michis_rand_distr_derive::StandardDistribution;
fn main() {}

#[derive(StandardDistribution)]
pub enum ErrorTest1 {
	#[standard_distribution(weight = 1.0f32)]
	Variant1,
	#[standard_distribution(weight = 3.0f64)]
	Variant2,
}

#[derive(StandardDistribution)]
pub enum ErrorTest2 {
	#[standard_distribution(weight = 1.0)]
	Variant1,
	#[standard_distribution(weight = 3)]
	Variant2,
}
