use michis_rand_distr_derive::StandardDistribution;
fn main() {}

#[derive(StandardDistribution)]
pub struct StructWithNamedFields {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[derive(StandardDistribution)]
pub struct StructNewType(pub f32);

#[derive(StandardDistribution)]
pub struct StructUnit;

#[derive(StandardDistribution)]
pub struct StructsWithTupleFields(pub f32, pub f32, pub f32);

#[derive(StandardDistribution)]
pub struct StructsWithTypeParams<F> {
	pub inner: F,
	pub other_inner: f32,
}
