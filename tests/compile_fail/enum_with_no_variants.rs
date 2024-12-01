use michis_rand_distr_derive::StandardDistribution;

#[derive(StandardDistribution)]
pub enum DoesNotSupportEnumWithNoVariants {}

fn main() {}
