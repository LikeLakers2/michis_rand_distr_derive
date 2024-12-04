use michis_rand_distr_derive::StandardDistribution;
fn main() {}

pub struct MyNonRandStruct;

#[derive(StandardDistribution)]
pub enum ErrorTest1 {
	Variant1 {
		field1: MyNonRandStruct,
		field2: MyNonRandStruct,
	},
	Variant2 {
		field: MyNonRandStruct,
	},
}

#[derive(StandardDistribution)]
pub struct ErrorTest2 {
	field1: MyNonRandStruct,
	field2: f32,
	field3: MyNonRandStruct,
}
