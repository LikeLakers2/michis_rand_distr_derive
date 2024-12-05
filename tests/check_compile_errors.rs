#[test]
fn ui() {
	let test_cases = trybuild::TestCases::new();

	// Derive Fail tests are those that fail during the derive generation process.
	test_cases.compile_fail("tests/derive_standard_distribution/derive_fail/*.rs");

	// Compile Fail tests are those that derive successfully, but generate uncompilable code.
	test_cases.compile_fail("tests/derive_standard_distribution/compile_fail/*.rs");

	// Pass tests are those that derive successfully *and* compile successfully.
	test_cases.pass("tests/derive_standard_distribution/pass/*.rs");
}
