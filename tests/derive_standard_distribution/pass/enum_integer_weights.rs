use michis_rand_distr_derive::StandardDistribution;
fn main() {}

#[derive(StandardDistribution)]
pub enum EnumIntegerWeights {
	#[standard_distribution(weight = 1)]
	Variant1,
	#[standard_distribution(weight = 3)]
	Variant2(f32),
	#[standard_distribution(weight = 2)]
	Variant3 { f: f32 },
}
