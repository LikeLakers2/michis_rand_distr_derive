#[test]
fn ui() {
	let test_cases = trybuild::TestCases::new();

	// Derive Fail tests are those that fail during the derive generation process.
	test_cases.compile_fail("tests/standard_distribution/derive_fail/*.rs");

	// Compile Fail tests are those that derive successfully, but generate uncompilable code.
	test_cases.compile_fail("tests/standard_distribution/compile_fail/*.rs");
}
