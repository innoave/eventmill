#[test]
fn build_struct_all_attributes_specified() {
    let t = trybuild::TestCases::new();
    t.pass("tests/build_struct_all_attributes_specified.rs")
}

#[test]
fn build_struct_no_attribute_specified() {
    let t = trybuild::TestCases::new();
    t.pass("tests/build_struct_no_attribute_specified.rs")
}

#[test]
fn build_struct_event_type_attribute_omitted() {
    let t = trybuild::TestCases::new();
    t.pass("tests/build_struct_event_type_attribute_omitted.rs")
}

#[test]
fn build_enum_no_attribute_specified() {
    let t = trybuild::TestCases::new();
    t.pass("tests/build_enum_no_attribute_specified.rs")
}

#[test]
fn build_enum_event_type_attribute_omitted() {
    let t = trybuild::TestCases::new();
    t.pass("tests/build_enum_event_type_attribute_omitted.rs")
}

#[test]
fn build_enum_event_type_attribute_on_variants() {
    let t = trybuild::TestCases::new();
    t.pass("tests/build_enum_event_type_attribute_on_variants.rs")
}
