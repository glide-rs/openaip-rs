use openaip::parse;

#[test]
fn it_works() {
    let str = include_str!("data/de_asp.xml");

    let result = parse(str);
    assert!(result.is_ok());

    let file = result.unwrap();
    assert!(file.airspaces.is_some());

    let airspaces = file.airspaces.unwrap();
    assert_eq!(airspaces.len(), 753);
    assert!(airspaces.iter().all(|item| item.is_ok()));
}
