use crate::mime::MimeDetector;

pub(crate) struct Jpeg2kDetector {
    pub sig: &'static [u8],
}

impl MimeDetector for Jpeg2kDetector {
    fn detect(&self, content: &[u8], _limit: usize) -> bool {
        if content.len() < 24 {
            return false;
        }
        if content[4..8] != [0x6A, 0x50, 0x20, 0x20] && content[4..8] != [0x6A, 0x50, 0x32, 0x20] {
            return false;
        }
        &content[20..24] == self.sig
    }
}

// Webp matches a WebP file.
pub(crate) struct WebpDetector;

impl MimeDetector for WebpDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        content.len() > 12
            && &content[0..4] == b"RIFF"
            && content[8..12] == [0x57, 0x45, 0x42, 0x50]
    }
}

// Dwg matches a CAD drawing file.
pub(crate) struct DwgDetector;

impl MimeDetector for DwgDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        if content.len() < 6 || content[0] != 0x41 || content[1] != 0x43 {
            return false;
        }
        let dwg_versions = vec![
            &[0x31, 0x2E, 0x34, 0x30],
            &[0x31, 0x2E, 0x35, 0x30],
            &[0x32, 0x2E, 0x31, 0x30],
            &[0x31, 0x30, 0x30, 0x32],
            &[0x31, 0x30, 0x30, 0x33],
            &[0x31, 0x30, 0x30, 0x34],
            &[0x31, 0x30, 0x30, 0x36],
            &[0x31, 0x30, 0x30, 0x39],
            &[0x31, 0x30, 0x31, 0x32],
            &[0x31, 0x30, 0x31, 0x34],
            &[0x31, 0x30, 0x31, 0x35],
            &[0x31, 0x30, 0x31, 0x38],
            &[0x31, 0x30, 0x32, 0x31],
            &[0x31, 0x30, 0x32, 0x34],
            &[0x31, 0x30, 0x33, 0x32],
        ];

        for d in dwg_versions {
            if &content[2..6] == d {
                return true;
            }
        }
        false
    }
}

// Jxl matches JPEG XL image file.
pub(crate) struct JxlDetector;

impl MimeDetector for JxlDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        content.starts_with(&[0xFF, 0x0A])
            || content.starts_with(b"\x00\x00\x00\x0cJXL\x20\x0d\x0a\x87\x0a")
    }
}
