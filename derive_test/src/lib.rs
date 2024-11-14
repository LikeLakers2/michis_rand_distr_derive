use michis_rand_distr_derive::StandardDistribution;

#[derive(StandardDistribution)]
pub struct SupportsStructsWithNamedFields {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[derive(StandardDistribution)]
pub struct SupportsStructNewType(pub f32);

#[derive(StandardDistribution)]
pub struct SupportsStructUnit;

#[derive(StandardDistribution)]
pub struct SupportsStructsWithTupleFields(pub f32, pub f32, pub f32);

#[derive(StandardDistribution)]
pub struct SupportsStructsWithTypeParams<F> {
	pub inner: F,
	pub other_inner: f32,
}
