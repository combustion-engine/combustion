use semver::{Version, VersionReq};
use toml::{Parser, Value};

lazy_static! {
    pub static ref COMBUSTION_PLUGIN_VERSION: Option<Version> = {
        get_crate_version().and_then(|version| {
            Version::parse(version.as_str()).ok()
        })
    };
}

pub fn get_crate_version() -> Option<String> {
    if let Some(cargo) = Parser::new(super::COMBUSTION_PLUGIN_CARGO_TOML).parse() {
        if let Some(package) = cargo.get("package").and_then(|p| p.as_table()) {
            if let Some(version) = package.get("version") {
                return match version {
                    &Value::String(ref s) => Some(s.clone()),
                    _ => None
                };
            }
        }
    }

    None
}

pub fn plugin_supported(version: &str) -> bool {
    if let Ok(req) = VersionReq::parse(version) {
        if let Some(ref version) = *COMBUSTION_PLUGIN_VERSION {
            return req.matches(version);
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_cargo() {
        println!("{:?}", COMBUSTION_PLUGIN_VERSION.as_ref().unwrap());
    }

    #[test]
    fn test_valid_plugin() {
        assert!(plugin_supported("0.1.0"));
        assert!(plugin_supported("^0.1.0"));
        assert!(plugin_supported("^0.1"));
        assert!(plugin_supported("*"));
    }

    #[test]
    #[should_panic]
    fn test_invalid_plugin() {
        //TODO: Update this when the crate version updates
        assert!(plugin_supported("0.2.0"));
    }
}