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

#[derive(StandardDistribution)]
pub enum SupportsEnum {
	UnitVariant,
	TupleVariant(f32, f32, f32),
	NamedVariant { x: f32, y: f32, z: f32 },
}

#[derive(StandardDistribution)]
pub enum SupportsEnumWithTypeParams<F> {
	UnitVariant,
	TupleVariant(F, f32),
	NamedVariant { inner: F, other_inner: f32 },
}
