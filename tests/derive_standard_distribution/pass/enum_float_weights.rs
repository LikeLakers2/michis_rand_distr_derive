use michis_rand_distr_derive::StandardDistribution;
fn main() {}

#[derive(StandardDistribution)]
pub enum EnumFloatWeights {
	#[standard_distribution(weight = 1.0)]
	Variant1,
	#[standard_distribution(weight = 3.0)]
	Variant2(f32),
	#[standard_distribution(weight = 2.0)]
	Variant3 { f: f32 },
}
