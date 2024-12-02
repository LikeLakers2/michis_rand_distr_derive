use darling::{ast::Fields, util::Flag, FromField};
use syn::{parse_quote, Expr, ExprStruct, Ident, Member, Path, Type};

use super::FieldsExt;

impl FieldsExt for Fields<DeriveField> {
	fn to_struct_expression(&self, struct_or_enum_path: Path) -> ExprStruct {
		let field_names_iter = self
			.iter()
			.enumerate()
			.map::<Member, _>(|(i, field)| field.ident.clone().map_or(i.into(), Into::into));
		let field_rng_calls = self.iter().map(|field| field.make_rng_call());

		parse_quote! {
			#struct_or_enum_path {
				#(#field_names_iter : #field_rng_calls),*
			}
		}
	}
}

#[derive(FromField, Debug)]
#[darling(attributes(standard_distribution))]
pub struct DeriveField {
	ident: Option<Ident>,
	ty: Type,

	/// If specified, this field will never be randomly generated. Instead, it will be generated
	/// using the field type's `Default` impl.
	skip: Flag,
}

impl DeriveField {
	pub(crate) fn make_rng_call(&self) -> Expr {
		if self.skip.is_present() {
			// TODO: Compile fail test if a field doesn't implement Default
			parse_quote! { Default::default() }
		} else {
			// TODO: Compile fail test if a field doesn't have `impl Distribution<T> for Standard`
			let ty = &self.ty;
			parse_quote! { rng.gen::< #ty >() }
		}
	}
}
