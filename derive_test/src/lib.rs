use michis_rand_distr_derive::Standard;
use rand::distributions::Standard;

#[derive(Standard)]
pub struct AStructWithTupleFields(pub f32, pub f32, pub f32);

#[derive(Standard)]
pub struct AStructWithNamedFields {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[derive(Standard)]
pub struct AStructWithTypeParams<F> {
	pub inner: F,
	pub other_inner: f32,
}
