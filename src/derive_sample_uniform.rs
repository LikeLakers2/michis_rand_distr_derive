use darling::{FromDeriveInput, Result as DarlingResult};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, ToTokens};
use syn::{parse_quote, Generics, Ident, ItemImpl, Path};

#[derive(FromDeriveInput)]
#[darling(
	attributes(sample_uniform),
	and_then = Self::check_and_correct,
)]
pub struct DeriveData {
	ident: Ident,
	generics: Generics,

	sampler_path: Option<Path>,
}

impl DeriveData {
	fn check_and_correct(self) -> DarlingResult<Self> {
		let Self {
			ident,
			generics,
			mut sampler_path,
		} = self;

		if sampler_path.is_none() {
			let new_sampler_path_ident = format_ident!("{}UniformSampler", &ident);
			sampler_path = Some(parse_quote! { #new_sampler_path_ident });
		}

		Ok(Self {
			ident,
			generics,
			sampler_path,
		})
	}
}

impl ToTokens for DeriveData {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		let Self {
			ident,
			generics,
			sampler_path,
		} = self;

		let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

		// At this point, `sampler_path` should always be a `Some(_)` - but just in case, let's
		// assert.
		assert!(sampler_path.is_some());

		let code: ItemImpl = parse_quote! {
			impl #impl_generics ::rand::distributions::uniform::UniformSampler
			for #ident #type_generics
			#where_clause
			{
				type Sampler = #sampler_path;
			}
		};

		code.to_tokens(tokens);
	}
}
