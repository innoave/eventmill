#[test]
fn all_tests() {
    let t = trybuild::TestCases::new();
    //
    // #[derive(EventType)]
    //
    t.pass("tests/event_struct_all_attributes.rs");
    t.pass("tests/event_struct_no_attributes.rs");
    t.pass("tests/event_struct_event_type_attribute_omitted.rs");
    t.pass("tests/event_enum_no_attributes.rs");
    t.pass("tests/event_enum_event_type_attribute_omitted.rs");
    t.pass("tests/event_enum_event_type_attribute_on_variants.rs");
    t.compile_fail("tests/event_struct_unknown_identifier_as_event_source.rs");
    t.pass("tests/event_struct_const_as_event_source.rs");
    t.pass("tests/event_enum_const_as_event_type_version.rs");
    //
    // #[derive(AggregateType)]
    //
    //    t.pass("tests/aggregate_struct_no_attributes.rs")
}
