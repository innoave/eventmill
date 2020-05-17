#[test]
fn failing_builds() {
    let t = trybuild::TestCases::new();
    //
    // #[derive(EventType)]
    //
    t.compile_fail("tests/failing_builds/event_struct_unknown_identifier_as_event_source.rs");
    //
    // #[derive(AggregateType)]
    //
    t.compile_fail("tests/failing_builds/aggregate_struct_id_field_not_found.rs");
}
