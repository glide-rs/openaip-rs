#[macro_use]
extern crate approx;

extern crate openaip;

use openaip::{
    parse,
    Airspace,
    AltitudeReference,
    AltitudeUnit,
    Category,
    Geometry,
};

#[test]
fn it_works() {
    let data: &'static str = r##"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<OPENAIP VERSION="d9192d6fa44fc5a0ecc3d84fd84d13e091df511c" DATAFORMAT="1.1">
<AIRSPACES>
<ASP CATEGORY="WAVE">
  <VERSION>d59ffb1bd865bc7307dbb3a191f4d00dfef5529f</VERSION>
  <ID>150668</ID>
  <COUNTRY>DE</COUNTRY>
  <NAME>Alb-Nord_1 128.950</NAME>
  <ALTLIMIT_TOP REFERENCE="STD">
    <ALT UNIT="FL">100</ALT>
  </ALTLIMIT_TOP>
  <ALTLIMIT_BOTTOM REFERENCE="MSL">
    <ALT UNIT="F">4500</ALT>
  </ALTLIMIT_BOTTOM>
  <GEOMETRY>
    <POLYGON>9.1911111111111 48.491111111111, 9.3727777777778 48.565555555556, 9.4222222222222 48.574722222222, 9.6236111111111 48.648333333333, 9.7472222222222 48.680833333333, 9.7462271986531 48.665486594604, 9.7441254667737 48.650187350037, 9.7410233702 48.634962603183, 9.7369276269873 48.619840899372, 9.7336111111111 48.609722222222, 9.6255555555556 48.5625, 9.1538888888889 48.448888888889, 9.1911111111111 48.491111111111</POLYGON>
  </GEOMETRY>
</ASP>
</AIRSPACES>
</OPENAIP>
"##;
    let result = parse(data.as_bytes());
    assert!(result.is_ok());

    let file = result.unwrap();
    assert!(file.airspaces.is_some());

    let ref airspaces = file.airspaces.as_ref().unwrap();
    assert_eq!(airspaces.len(), 1);

    let airspace: &Airspace = airspaces[0].as_ref().unwrap();
    assert_eq!(airspace.category, Category::Wave);
    assert_eq!(airspace.version, "d59ffb1bd865bc7307dbb3a191f4d00dfef5529f");
    assert_eq!(airspace.id, "150668");
    assert_eq!(airspace.country, "DE");
    assert_eq!(airspace.name, "Alb-Nord_1 128.950");
    assert_eq!(airspace.top.reference, AltitudeReference::STD);
    assert_eq!(airspace.top.unit, AltitudeUnit::FlightLevel);
    assert_eq!(airspace.top.value, 100);
    assert_eq!(airspace.bottom.reference, AltitudeReference::MSL);
    assert_eq!(airspace.bottom.unit, AltitudeUnit::Feet);
    assert_eq!(airspace.bottom.value, 4500);

    match airspace.geometry {
        Geometry::Polygon(ref polygon) => {
            assert_eq!(polygon.len(), 13);

            let first_point = polygon[0];
            assert_relative_eq!(first_point.longitude, 9.1911111111111);
            assert_relative_eq!(first_point.latitude, 48.491111111111);

            let second_point = polygon[1];
            assert_relative_eq!(second_point.longitude, 9.3727777777778);
            assert_relative_eq!(second_point.latitude, 48.565555555556);
        },
    }
}