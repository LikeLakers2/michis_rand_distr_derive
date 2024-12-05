use michis_rand_distr_derive::StandardDistribution;
fn main() {}

#[derive(StandardDistribution)]
pub struct FieldSkip {
	_field1: f32,
	_field2: f64,
	#[standard_distribution(skip)]
	_field3: String,
}
