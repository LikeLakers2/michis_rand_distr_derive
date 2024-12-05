use michis_rand_distr_derive::StandardDistribution;
fn main() {}

#[derive(StandardDistribution)]
pub enum RegularEnum {
	UnitVariant,
	TupleVariant(f32, f32, f32),
	NamedVariant { x: f32, y: f32, z: f32 },
}

#[derive(StandardDistribution)]
pub enum EnumWithTypeParams<F> {
	UnitVariant,
	TupleVariant(F, f32),
	NamedVariant { inner: F, other_inner: f32 },
}
