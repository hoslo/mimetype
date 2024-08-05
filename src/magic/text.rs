use std::sync::LazyLock;

use serde_json::Value;

use crate::mime::MimeDetector;

use super::base::{trim_left_ws, BytesExt, CiPrefixDetector, SheBangDetector};

#[derive(Debug)]
struct Bom {
    bom: Vec<u8>,
    enc: &'static str,
}

static BOMS: LazyLock<Vec<Bom>> = LazyLock::new(|| {
    vec![
        Bom {
            bom: vec![0xEF, 0xBB, 0xBF],
            enc: "UTF-8",
        },
        Bom {
            bom: vec![0xFE, 0xFF],
            enc: "UTF-16BE",
        },
        Bom {
            bom: vec![0xFF, 0xFE],
            enc: "UTF-16LE",
        },
        Bom {
            bom: vec![0x00, 0x00, 0xFE, 0xFF],
            enc: "UTF-32BE",
        },
        Bom {
            bom: vec![0xFF, 0xFE, 0x00, 0x00],
            enc: "UTF-32LE",
        },
    ]
});
pub(crate) fn from_boom(content: &[u8]) -> Option<&str> {
    for bom in &*BOMS {
        if content.starts_with(&bom.bom) {
            return Some(bom.enc);
        }
    }
    None
}

// Text matches a plain text file.
//
// TODO: This function does not parse BOM-less UTF16 and UTF32 files. Not really
// sure it should. Linux file utility also requires a BOM for UTF16 and UTF32.
pub(crate) struct TextDetector;

impl MimeDetector for TextDetector {
    fn detect(&self, content: &[u8], _limit: usize) -> bool {
        // First look for BOM.
        if from_boom(content).is_some() {
            return true;
        }
        // Binary data bytes as defined here: https://mimesniff.spec.whatwg.org/#binary-data-byte
        for &b in content {
            if b <= 0x08 || b == 0x0B || (0x0E..=0x1A).contains(&b) || (0x1C..=0x1F).contains(&b) {
                return false;
            }
        }
        true
    }
}

// Php matches a PHP: Hypertext Preprocessor file.
pub(crate) struct PhpDetector;

impl MimeDetector for PhpDetector {
    fn detect(&self, content: &[u8], limit: usize) -> bool {
        let php_page_f = SheBangDetector {
            sigs: vec![b"/usr/local/bin/php", b"/usr/bin/php", b"/usr/bin/env php"],
        };
        if php_page_f.detect(content, limit) {
            return true;
        }
        let php_script_f = CiPrefixDetector {
            sigs: vec![b"<?PHP", b"<?\n", b"<?\r", b"<? "],
        };
        php_script_f.detect(content, limit)
    }
}

// JSON matches a JavaScript Object Notation file.
pub(crate) struct JsonDetector;

impl MimeDetector for JsonDetector {
    fn detect(&self, content: &[u8], _limit: usize) -> bool {
        let content = trim_left_ws(content);
        // #175 A single JSON string, number or bool is not considered JSON.
        // JSON objects and arrays are reported as JSON.
        if content.len() < 2 || (content[0] != b'[' && content[0] != b'{') {
            return false;
        }

        let parsed = serde_json::from_slice::<Value>(content);

        parsed.is_ok() && !content.is_empty()
    }
}

// GeoJSON matches a RFC 7946 GeoJSON file.
//
// GeoJSON detection implies searching for key:value pairs like: `"type": "Feature"`
// in the input.
// BUG(gabriel-vasile): The "type" key should be searched for in the root object.
pub(crate) struct GeoJsonDetector;

impl MimeDetector for GeoJsonDetector {
    fn detect(&self, content: &[u8], _limit: usize) -> bool {
        let mut content = trim_left_ws(content);
        if content.is_empty() {
            return false;
        }
        // GeoJSON is always a JSON object, not a JSON array or any other JSON value.
        if content[0] != b'{' {
            return false;
        }
        let s = br#""type""#;
        let (si, sl) = (content.index(s), s.len());
        let Some(si) = si else {
            return false;
        };
        // If the "type" string is the suffix of the input,
        // there is no need to search for the value of the key.
        if si + sl == content.len() {
            return false;
        }
        // Skip the "type" part.
        content = &content[si + sl..];
        // Skip any whitespace before the colon.
        content = trim_left_ws(content);
        // Check for colon.
        if content.is_empty() || content[0] != b':' {
            return false;
        }

        // Skip any whitespace after the colon.
        content = trim_left_ws(&content[1..]);

        let geo_jsontypes: Vec<&[u8]> = vec![
            br#""Feature""#,
            br#""FeatureCollection""#,
            br#""Point""#,
            br#""LineString""#,
            br#""Polygon""#,
            br#""MultiPoint""#,
            br#""MultiLineString""#,
            br#""MultiPolygon""#,
            br#""GeometryCollection""#,
        ];
        for t in geo_jsontypes {
            if content.starts_with(t) {
                return true;
            }
        }
        false
    }
}

// HAR matches a HAR Spec file.
// Spec: http://www.softwareishard.com/blog/har-12-spec/
pub(crate) struct HarDetector;

impl MimeDetector for HarDetector {
    fn detect(&self, content: &[u8], _limit: usize) -> bool {
        let s = br#""log""#;
        let (si, sl) = (content.index(s), s.len());

        let Some(si) = si else {
            println!("11111");
            return false;
        };
        println!("22222");
        // If the "log" string is the suffix of the input,
        // there is no need to search for the value of the key.
        if si + sl == content.len() {
            return false;
        }
        // Skip the "log" part.
        let mut content = &content[si + sl..];
        // Skip any whitespace before the colon.
        content = trim_left_ws(content);
        // Check for colon.
        if content.is_empty() || content[0] != b':' {
            return false;
        }
        // Skip any whitespace after the colon.
        content = trim_left_ws(&content[1..]);

        let har_jsontypes: Vec<&[u8]> = vec![br#""version""#, br#""creator""#, br#""entries""#];
        for t in har_jsontypes {
            let si = content.index(t);
            if si.is_some() {
                return true;
            }
        }

        false
    }
}

// Svg matches a SVG file.
pub(crate) struct SvgDetector;

impl MimeDetector for SvgDetector {
    fn detect(&self, content: &[u8], _limit: usize) -> bool {
        let s = b"<svg";
        return content.windows(s.len()).any(|window| window == s);
    }
}

// P7s matches an .p7s signature File (PEM, Base64).
pub(crate) struct P7sDetector;

impl MimeDetector for P7sDetector {
    fn detect(&self, content: &[u8], _limit: usize) -> bool {
        // Check for PEM Encoding.
        if content.starts_with(b"-----BEGIN PKCS7") {
            return true;
        }
        // Check if DER Encoding is long enough.
        if content.len() < 20 {
            return false;
        }
        // Magic Bytes for the signedData ASN.1 encoding.
        let start_header: Vec<&[u8]> = vec![
            &[0x30, 0x80],
            &[0x30, 0x81],
            &[0x30, 0x82],
            &[0x30, 0x83],
            &[0x30, 0x84],
        ];
        let signed_data_match = &[0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07];
        // Check if Header is correct. There are multiple valid headers.
        for (i, m) in start_header.iter().enumerate() {
            // If first bytes match, then check for ASN.1 Object Type.
            if content.starts_with(m) && content[i + 2..].starts_with(signed_data_match) {
                return true;
            }
        }

        false
    }
}
