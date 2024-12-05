use michis_rand_distr_derive::StandardDistribution;
fn main() {}

pub struct MyNonDefaultStruct;

#[derive(StandardDistribution)]
pub enum ErrorTest1 {
	Variant1 {
		#[standard_distribution(skip)]
		field1: MyNonDefaultStruct,
		#[standard_distribution(skip)]
		field2: MyNonDefaultStruct,
	},
	Variant2 {
		#[standard_distribution(skip)]
		field: MyNonDefaultStruct,
	},
}

#[derive(StandardDistribution)]
pub struct ErrorTest2 {
	#[standard_distribution(skip)]
	field1: MyNonDefaultStruct,
	field2: f32,
	#[standard_distribution(skip)]
	field3: MyNonDefaultStruct,
}
