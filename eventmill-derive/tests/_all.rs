#[test]
fn all_attributes_specified() {
    let t = trybuild::TestCases::new();
    t.pass("tests/all_attributes_specified.rs")
}

#[test]
fn no_attribute_specified() {
    let t = trybuild::TestCases::new();
    t.pass("tests/no_attribute_specified.rs")
}

#[test]
fn event_type_attribute_omitted() {
    let t = trybuild::TestCases::new();
    t.pass("tests/event_type_attribute_omitted.rs")
}
