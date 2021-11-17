use serde::{Deserialize, Serialize};
// use serde_json::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct GreenPass {
    ver: String,
}

#[cfg(test)]
mod tests {
    use crate::GreenPass;

    #[test]
    fn validate_json_version() {
        let data = r#"
        {
            "ver": "1.3.0",
            "nam": {
              "fn": "Smith-Jones",
              "fnt": "SMITH<JONES",
              "gn": "Charles Edward",
              "gnt": "CHARLES<EDWARD"
            },
            "dob": "1964-01-01",
            "v": [
              {
                "tg": "840539006",
                "vp": "1119349007",
                "mp": "EU/1/20/1507",
                "ma": "ORG-100031184",
                "dn": 1,
                "sd": 2,
                "dt": "2021-06-11",
                "co": "NL",
                "is": "Ministry of Health Welfare and Sport",
                "ci": "URN:UVCI:01:NL:DADFCC47C7334E45A906DB12FD859FB7#1"
              }
            ]
          }"#;

        let gp: GreenPass = serde_json::from_str(data).unwrap();
        assert_eq!(gp.ver, "1.3.0");
    }
}
