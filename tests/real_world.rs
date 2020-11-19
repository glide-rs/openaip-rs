use openaip::parse;

#[test]
fn it_works() {
    let bytes = include_bytes!("data/de_asp.aip");

    let result = parse(bytes.as_ref());
    assert!(result.is_ok());

    let file = result.unwrap();
    assert!(file.airspaces.is_some());

    let airspaces = file.airspaces.unwrap();
    assert_eq!(airspaces.len(), 621);
    assert!(airspaces.iter().all(|item| item.is_ok()));
}
