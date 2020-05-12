#[test]
fn build_struct_all_attributes_specified() {
    let t = trybuild::TestCases::new();
    t.pass("tests/build_struct_all_attributes_specified.rs");
    t.pass("tests/build_struct_no_attribute_specified.rs");
    t.pass("tests/build_struct_event_type_attribute_omitted.rs");
    t.pass("tests/build_enum_no_attribute_specified.rs");
    t.pass("tests/build_enum_event_type_attribute_omitted.rs");
    t.pass("tests/build_enum_event_type_attribute_on_variants.rs");
    t.compile_fail("tests/fail_struct_unknown_identifier_as_event_source.rs");
    t.pass("tests/build_struct_const_as_event_source.rs");
    t.pass("tests/build_enum_const_as_event_type_version.rs");
}
