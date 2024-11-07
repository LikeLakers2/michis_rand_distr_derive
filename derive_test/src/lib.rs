#[derive(michis_rand_distr_derive::StandardDistr)]
pub struct AStructWithBareTypes {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[derive(michis_rand_distr_derive::StandardDistr)]
pub struct AStructWithTypeParams<F> {
	pub inner: F,
	pub other_inner: f32,
}
