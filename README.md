openaip-rs
==============================================================================

[OpenAIP](https://www.openaip.net/) file parser for [Rust](https://www.rust-lang.org/).


Features
------------------------------------------------------------------------------

- Reading OpenAIP airspace files

not implemented yet:

- Writing OpenAIP airspace files
- Reading/Writing OpenAIP airport files
- Reading/Writing OpenAIP hotspot files
- Reading/Writing OpenAIP navaid files


Usage
------------------------------------------------------------------------------

Read the [full documentation](https://docs.rs/openaip) at docs.rs


### Example

```rust
use openaip::parse;

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
        <POLYGON>9.1911 48.4911, 9.3727 48.5655, 9.4222 48.5747, 9.1911 48.4911</POLYGON>
      </GEOMETRY>
    </ASP>
  </AIRSPACES>
</OPENAIP>
"##;

let result = parse(data.as_bytes());
assert!(result.is_ok());
```


License
------------------------------------------------------------------------------
openaip-rs is licensed under the [Apache 2.0 License](LICENSE).
