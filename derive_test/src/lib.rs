use michis_rand_distr_derive::add_standard_distribution_support;

#[add_standard_distribution_support]
pub struct AStructWithNamedFields {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[add_standard_distribution_support]
pub struct AStructWithTupleFields(pub f32, pub f32, pub f32);

#[add_standard_distribution_support]
pub struct AStructWithTypeParams<F> {
	pub inner: F,
	pub other_inner: f32,
}
