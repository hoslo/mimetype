use crate::mime::MimeDetector;

// QuickTime matches a QuickTime File Format file.
// https://www.loc.gov/preservation/digital/formats/fdd/fdd000052.shtml
// https://developer.apple.com/library/archive/documentation/QuickTime/QTFF/QTFFChap1/qtff1.html#//apple_ref/doc/uid/TP40000939-CH203-38190
// https://github.com/apache/tika/blob/0f5570691133c75ac4472c3340354a6c4080b104/tika-core/src/main/resources/org/apache/tika/mime/tika-mimetypes.xml#L7758-L7777
pub(crate) struct QuickTimeDetector;

impl MimeDetector for QuickTimeDetector {
    fn detect(&self, content: &[u8], _limit: usize) -> bool {
        if content.len() < 12 {
            return false;
        }
        // First 4 bytes represent the size of the atom as unsigned int.
        // Next 4 bytes are the type of the atom.
        // For `ftyp` atoms check if first byte in size is 0, otherwise, a text file
        // which happens to contain 'ftypqt  ' at index 4 will trigger a false positive.
        if &content[4..12] == b"ftypqt  " || &content[4..8] == b"ftypmoov" {
            return content[0] == 0x00;
        }
        let basic_atom_types = vec![
            b"moov\x00",
            b"mdat\x00",
            b"free\x00",
            b"skip\x00",
            b"pnot\x00",
        ];
        for a in basic_atom_types {
            if &content[4..9] == a {
                return true;
            }
        }
        &content[..8] == b"\x00\x00\x00\x08wide"
    }
}
