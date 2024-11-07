use proc_macro2::TokenStream;
use syn::{DeriveInput, Generics, ItemImpl, ItemStruct};

pub fn derive(input: DeriveInput) -> TokenStream {
	todo!()
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
	todo!()
}

fn generate_sample_uniform_impl(input: DeriveInput) -> ItemImpl {
	todo!()
}

fn generate_uniform_sampler(input: DeriveInput) -> ItemStruct {
	todo!()
}
