use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};

use crate::mime::MimeDetector;

use super::base::BytesExt;

pub(crate) struct Mp3Detector;

impl MimeDetector for Mp3Detector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        if content.len() < 3 {
            return false;
        }

        if content.starts_with(b"ID3") {
            // MP3s with an ID3v2 tag will start with "ID3"
            // ID3v1 tags, however appear at the end of the file.
            return true;
        }
        let Ok(v) = std::io::Cursor::new(&content[..2]).read_u16::<BigEndian>() else {
            return false;
        };
        match v & 0xFFFE {
            // 0xFFFA MPEG ADTS, layer III, v1
            // 0xFFF2 MPEG ADTS, layer III, v2
            // 0xFFE2 MPEG ADTS, layer III, v2.5
            0xFFFA | 0xFFF2 | 0xFFE2 => true,
            _ => false,
        }
    }
}

pub(crate) struct WavDetector;

impl MimeDetector for WavDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        if content.len() < 12 {
            return false;
        }
        if &content[..4] != b"RIFF" {
            return false;
        }
        if content[8..12] != [0x57, 0x41, 0x56, 0x45] {
            return false;
        }
        true
    }
}

pub(crate) struct AiffDetector;

impl MimeDetector for AiffDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        if content.len() < 12 {
            return false;
        }
        if content[..4] != [0x46, 0x4F, 0x52, 0x4D] {
            return false;
        }
        if content[8..12] != [0x41, 0x49, 0x46, 0x46] {
            return false;
        }
        true
    }
}

pub(crate) struct QcpDetector;

impl MimeDetector for QcpDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        if content.len() < 4 {
            return false;
        }
        if &content[..4] != b"RIFF" {
            return false;
        }
        if &content[8..12] != b"QLCM" {
            return false;
        }
        true
    }
}

// OggAudio matches an audio ogg file.
pub(crate) struct OggAudioDetector;

impl MimeDetector for OggAudioDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        content.len() >= 37
            && (content[28..].starts_with(b"\x7fFLAC")
                || content[28..].starts_with(b"\x01vorbis")
                || content[28..].starts_with(b"OpusHead")
                || content[28..].starts_with(b"Speex\x20\x20\x20"))
    }
}

// OggVideo matches a video ogg file.
pub(crate) struct OggVideoDetector;

impl MimeDetector for OggVideoDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        content.len() >= 37
            && (content[28..].starts_with(b"\x80theora")
                || content[28..].starts_with(b"fishead\x00")
                || content[28..].starts_with(b"\x01video\x00\x00\x00"))
    }
}

// Mpeg matches a Moving Picture Experts Group file.
pub(crate) struct MpegDetector;

impl MimeDetector for MpegDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        content.len() > 3
            && content.starts_with(&[0x00, 0x00, 0x01])
            && content[3] >= 0xB0
            && content[3] <= 0xBF
    }
}

// WebM matches a WebM file.
pub(crate) struct WebMDetector;

impl MimeDetector for WebMDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        is_matroska_file_type_matched(content, "webm".to_string())
    }
}

// Mkv matches a mkv file.
pub(crate) struct MkvDetector;

impl MimeDetector for MkvDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        is_matroska_file_type_matched(content, "matroska".to_string())
    }
}

// isMatroskaFileTypeMatched is used for webm and mkv file matching.
// It checks for .Eß£ sequence. If the sequence is found,
// then it means it is Matroska media container, including WebM.
// Then it verifies which of the file type it is representing by matching the
// file specific string.
fn is_matroska_file_type_matched(content: &[u8], fl_type: String) -> bool {
    if content.starts_with(b"\x1A\x45\xDF\xA3") {
        return is_file_type_name_present(content, fl_type);
    }
    false
}

fn is_file_type_name_present(content: &[u8], fl_type: String) -> bool {
    let (mut max_ind, len_in) = (4096, content.len());
    if len_in < max_ind {
        // restricting length to 4096
        max_ind = len_in
    }
    let Some(i) = content[..max_ind].index(b"\x42\x82") else {
        return false;
    };
    let mut ind = i;
    if ind > 0 && len_in > ind + 2 {
        ind += 2;

        // filetype name will be present exactly
        // n bytes after the match of the two bytes "\x42\x82"
        let n = vint_width(content[ind] as usize);
        if len_in > ind + n {
            return content[ind + n..].starts_with(fl_type.as_bytes());
        }
    }
    false
}

// vintWidth parses the variable-integer width in matroska containers
fn vint_width(v: usize) -> usize {
    let (mut mask, max, mut num) = (128, 8, 1);
    while num < max && v & mask == 0 {
        mask >>= 1;
        num += 1;
    }
    num
}

// Avi matches an Audio Video Interleaved file.
pub(crate) struct AviDetector;

impl MimeDetector for AviDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        content.len() > 16 && &content[..4] == b"RIFF" && &content[8..16] == b"AVI LIST"
    }
}

// Shp matches a shape format file.
// https://www.esri.com/library/whitepapers/pdfs/shapefile.pdf
pub(crate) struct ShpDetector;

impl MimeDetector for ShpDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        if content.len() < 112 {
            return false;
        }

        if !(BigEndian::read_u32(&content[0..4]) == 9994
            && BigEndian::read_u32(&content[4..8]) == 0
            && BigEndian::read_u32(&content[8..12]) == 0
            && BigEndian::read_u32(&content[12..16]) == 0
            && BigEndian::read_u32(&content[16..20]) == 0
            && BigEndian::read_u32(&content[20..24]) == 0
            && LittleEndian::read_u32(&content[28..32]) == 1000)
        {
            return false;
        }

        let shape_types = vec![
            0,  // Null shape
            1,  // Point
            3,  // Polyline
            5,  // Polygon
            8,  // MultiPoint
            11, // PointZ
            13, // PolylineZ
            15, // PolygonZ
            18, // MultiPointZ
            21, // PointM
            23, // PolylineM
            25, // PolygonM
            28, // MultiPointM
            31, // MultiPatch
        ];

        for st in shape_types {
            if st == LittleEndian::read_u32(&content[108..112]) {
                return true;
            }
        }

        false
    }
}
