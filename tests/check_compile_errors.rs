#[test]
fn ui() {
	let test_cases = trybuild::TestCases::new();
	
	// Derive Fail tests are those that fail during the derive generation process.
	test_cases.compile_fail("tests/derive_fail/*.rs");
	
	// Compile Fail tests are those that derive successfully, but generate uncompilable code.
	test_cases.compile_fail("tests/compile_fail/*.rs");
}
