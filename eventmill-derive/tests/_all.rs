#[test]
fn struct_all_attributes_specified() {
    let t = trybuild::TestCases::new();
    t.pass("tests/struct_all_attributes_specified.rs")
}

#[test]
fn struct_no_attribute_specified() {
    let t = trybuild::TestCases::new();
    t.pass("tests/struct_no_attribute_specified.rs")
}

#[test]
fn struct_event_type_attribute_omitted() {
    let t = trybuild::TestCases::new();
    t.pass("tests/struct_event_type_attribute_omitted.rs")
}

#[test]
fn enum_no_attribute_specified() {
    let t = trybuild::TestCases::new();
    t.pass("tests/enum_no_attribute_specified.rs")
}

#[test]
fn enum_event_type_attribute_omitted() {
    let t = trybuild::TestCases::new();
    t.pass("tests/enum_event_type_attribute_omitted.rs")
}
