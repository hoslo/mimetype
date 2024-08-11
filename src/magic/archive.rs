use std::vec;

use byteorder::{ByteOrder, LittleEndian};

use crate::mime::MimeDetector;

use super::base::BytesExt;

// zip matches a zip archive.
pub(crate) struct ZipDetector;

impl MimeDetector for ZipDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        content.len() > 3
            && content[0] == 0x50
            && content[1] == 0x4B
            && (content[2] == 0x3 || content[2] == 0x5 || content[2] == 0x7)
            && (content[3] == 0x4 || content[3] == 0x6 || content[3] == 0x8)
    }
}

// Tar matches a (t)ape (ar)chive file.
// Tar files are divided into 512 bytes records. First record contains a 257
// bytes header padded with NUL.
pub(crate) struct TarDetector;

impl MimeDetector for TarDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        let mut content = content;

        if content.len() < 512 {
            return false;
        }
        content = &content[..512];

        // First 100 bytes of the header represent the file name.
        // Check if file looks like Gentoo GLEP binary package.
        let s = b"/gpkg-1\x00";
        if content[..100].index(s).is_some() {
            return true;
        }

        // Get the checksum recorded into the file.
        let recsum = tar_parse_octal(&content[148..156]);
        if recsum == -1 {
            return false;
        }
        let (sum1, sum2) = tar_chksum(content);
        recsum == sum1 || recsum == sum2
    }
}

// tarChksum computes the checksum for the header block b.
// The actual checksum is written to same b block after it has been calculated.
// Before calculation the bytes from b reserved for checksum have placeholder
// value of ASCII space 0x20.
// POSIX specifies a sum of the unsigned byte values, but the Sun tar used
// signed byte values. We compute and return both.
fn tar_chksum(b: &[u8]) -> (i64, i64) {
    let mut unsigned = 0i64;
    let mut signed = 0i64;
    for (i, c) in b.iter().enumerate() {
        let mut cc = *c;
        if (148..156).contains(&i) {
            cc = b' '
        }
        unsigned += cc as i64;
        signed += cc as i64;
    }
    (unsigned, signed)
}

// tarParseOctal converts octal string to decimal int.
fn tar_parse_octal(bs: &[u8]) -> i64 {
    // Because unused fields are filled with NULs, we need to skip leading NULs.
    // Fields may also be padded with spaces or NULs.
    // So we remove leading and trailing NULs and spaces to be sure.
    let bs = bs.trim_ascii();
    if bs.is_empty() {
        return -1;
    }

    let mut ret = 0;
    for &b in bs {
        if b == 0 {
            break;
        }
        if !(b'0'..=b'7').contains(&b) {
            return -1;
        }
        ret = (ret << 3) | (b - b'0') as i64
    }
    ret
}

// Xlsx matches a Microsoft Excel 2007 file.
pub(crate) struct XlsxDetector;

impl MimeDetector for XlsxDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        zip_contains(
            content,
            vec![
                b"xl/worksheets/",
                b"xl/drawings/",
                b"xl/theme/",
                b"xl/_rels/",
                b"xl/styles.xml",
                b"xl/workbook.xml",
                b"xl/sharedStrings.xml",
            ],
        )
    }
}

// Pub matches a Microsoft Publisher file.
pub(crate) struct PubDetector;

impl MimeDetector for PubDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        match_ole_clsid(
            content,
            &[
                0x01, 0x12, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x46,
            ],
        )
    }
}

// Ppt matches a Microsoft PowerPoint 97-2003 file or a PowerPoint 95 presentation.
pub(crate) struct PptDetector;

impl MimeDetector for PptDetector {
    fn detect(&self, content: &[u8], _limit: usize) -> bool {
        // Root CLSID test is the safest way to detect identify OLE, however, the format
        // often places the root CLSID at the end of the file.
        if match_ole_clsid(
            content,
            &[
                0x10, 0x8d, 0x81, 0x64, 0x9b, 0x4f, 0xcf, 0x11, 0x86, 0xea, 0x00, 0xaa, 0x00, 0xb9,
                0x29, 0xe8,
            ],
        ) || match_ole_clsid(
            content,
            &[
                0x70, 0xae, 0x7b, 0xea, 0x3b, 0xfb, 0xcd, 0x11, 0xa9, 0x03, 0x00, 0xaa, 0x00, 0x51,
                0x0e, 0xa3,
            ],
        ) {
            return true;
        }

        let lin = content.len();
        if lin < 520 {
            return false;
        }
        let ppt_sub_headers = vec![
            &[0xA0, 0x46, 0x1D, 0xF0],
            &[0x00, 0x6E, 0x1E, 0xF0],
            &[0x0F, 0x00, 0xE8, 0x03],
        ];
        for h in ppt_sub_headers {
            if content[512..].starts_with(h) {
                return true;
            }
        }

        if content[512..].starts_with(&[0xFD, 0xFF, 0xFF, 0xFF])
            && content[518] == 0x00
            && content[519] == 0x00
        {
            return true;
        }

        lin > 1152 && content[1152..4096.min(lin)].index(b"P\x00o\x00w\x00e\x00r\x00P\x00o\x00i\x00n\x00t\x00 D\x00o\x00c\x00u\x00m\x00e\x00n\x00t").is_some()
    }
}

// Doc matches a Microsoft Word 97-2003 file.
// See: https://github.com/decalage2/oletools/blob/412ee36ae45e70f42123e835871bac956d958461/oletools/common/clsid.py
pub(crate) struct DocDetector;

impl MimeDetector for DocDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        let clsids = vec![
            // Microsoft Word 97-2003 Document (Word.Document.8)
            &[
                0x06, 0x09, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x46,
            ],
            // Microsoft Word 6.0-7.0 Document (Word.Document.6)
            &[
                0x00, 0x09, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x46,
            ],
            // Microsoft Word Picture (Word.Picture.8)
            &[
                0x07, 0x09, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x46,
            ],
        ];

        for clsid in clsids {
            if match_ole_clsid(content, clsid) {
                return true;
            }
        }

        false
    }
}

// Docx matches a Microsoft Word 2007 file.
pub(crate) struct DocxDetector;

impl MimeDetector for DocxDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        zip_contains(
            content,
            vec![
                b"word/media/",
                b"word/_rels/document.xml.rels",
                b"word/document.xml",
                b"word/styles.xml",
                b"word/fontTable.xml",
                b"word/settings.xml",
                b"word/numbering.xml",
                b"word/header",
                b"word/footer",
            ],
        )
    }
}

// Pptx matches a Microsoft PowerPoint 2007 file.
pub(crate) struct PptxDetector;

impl MimeDetector for PptxDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        zip_contains(
            content,
            vec![
                b"ppt/slides/",
                b"ppt/media/",
                b"ppt/slideLayouts/",
                b"ppt/theme/",
                b"ppt/slideMasters/",
                b"ppt/tags/",
                b"ppt/notesMasters/",
                b"ppt/_rels/",
                b"ppt/handoutMasters/",
                b"ppt/notesSlides/",
                b"ppt/presentation.xml",
                b"ppt/tableStyles.xml",
                b"ppt/presProps.xml",
                b"ppt/viewProps.xml",
            ],
        )
    }
}

// Msi matches a Microsoft Windows Installer file.
// http://fileformats.archiveteam.org/wiki/Microsoft_Compound_File
pub(crate) struct MsiDetector;

impl MimeDetector for MsiDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        match_ole_clsid(
            content,
            &[
                0x84, 0x10, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x46,
            ],
        )
    }
}

// Msg matches a Microsoft Outlook email file.
pub(crate) struct MsgDetector;

impl MimeDetector for MsgDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        match_ole_clsid(
            content,
            &[
                0x0B, 0x0D, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x46,
            ],
        )
    }
}

// Aaf matches an Advanced Authoring Format file.
// See: https://pyaaf.readthedocs.io/en/latest/about.html
// See: https://en.wikipedia.org/wiki/Advanced_Authoring_Format
pub(crate) struct AafDetector;

impl MimeDetector for AafDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        if content.len() < 31 {
            return false;
        }
        content[8..].starts_with(&[0x41, 0x41, 0x46, 0x42, 0x0D, 0x00, 0x4F, 0x4D])
            && (content[30] == 0x09 || content[30] == 0x0C)
    }
}

// Xls matches a Microsoft Excel 97-2003 file.
pub(crate) struct XlsDetector;

impl MimeDetector for XlsDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        // Root CLSID test is the safest way to detect identify OLE, however, the format
        // often places the root CLSID at the end of the file.
        if match_ole_clsid(content, &[0x10, 0x08, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00])
            || match_ole_clsid(content, &[0x20, 0x08, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00])
        {
            return true;
        }

        let lin = content.len();
        if lin < 520 {
            return false;
        }
        let xls_sub_headers: Vec<&[u8]> = vec![
            &[0x09, 0x08, 0x10, 0x00, 0x00, 0x06, 0x05, 0x00],
            &[0xFD, 0xFF, 0xFF, 0xFF, 0x10],
            &[0xFD, 0xFF, 0xFF, 0xFF, 0x1F],
            &[0xFD, 0xFF, 0xFF, 0xFF, 0x22],
            &[0xFD, 0xFF, 0xFF, 0xFF, 0x23],
            &[0xFD, 0xFF, 0xFF, 0xFF, 0x28],
            &[0xFD, 0xFF, 0xFF, 0xFF, 0x29],
        ];
        for h in xls_sub_headers {
            if content[512..].starts_with(h) {
                return true;
            }
        }

        lin > 1152
            && content[1152..4096.min(lin)]
                .index(b"W\x00k\x00s\x00S\x00S\x00W\x00o\x00r\x00k\x00B\x00o\x00o\x00k")
                .is_some()
    }
}

fn match_ole_clsid(content: &[u8], clsid: &[u8]) -> bool {
    // Microsoft Compound files v3 have a sector length of 512, while v4 has 4096.
    // Change sector offset depending on file version.
    // https://www.loc.gov/preservation/digital/formats/fdd/fdd000392.shtml
    let mut sector_length = 512u32;
    if content.len() < sector_length as usize {
        return false;
    }
    if content[26] == 0x04 && content[27] == 0x00 {
        sector_length = 4096
    }
    // SecID of first sector of the directory stream.
    let first_sec_id = LittleEndian::read_u32(&content[48..52]);

    // Expected offset of CLSID for root storage object.
    let clsid_offset = sector_length * (1 + first_sec_id) + 80;

    if content.len() as u32 <= clsid_offset + 16 {
        return false;
    }

    content[clsid_offset as usize..].starts_with(clsid)
}

// Jar matches a Java archive file.
pub(crate) struct JarDetector;

impl MimeDetector for JarDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        zip_contains(content, vec![b"META-INF/MANIFEST.MF"])
    }
}

// zipTokenizer holds the source zip file and scanned index.
struct ZipTokenizer {
    content: Vec<u8>,
    index: usize,
}

impl ZipTokenizer {
    // next returns the next file name from the zip headers.
    // https://web.archive.org/web/20191129114319/https://users.cs.jmu.edu/buchhofp/forensics/formats/pkzip.html
    fn next(&mut self) -> Option<&[u8]> {
        let t = self;
        if t.index > t.content.len() {
            return None;
        }
        let content = &t.content[t.index..];
        // pkSig is the signature of the zip local file header.
        let pk_sig = b"PK\x03\x04";
        let pk_index = content.index(pk_sig)?;
        // 30 is the offset of the file name in the header.
        let file_name_offset = pk_index + 30;
        // end if signature not found or file name offset outside of file.
        if file_name_offset > content.len() {
            return None;
        }

        let file_name_len = LittleEndian::read_u16(&content[pk_index + 26..pk_index + 28]) as usize;
        if file_name_len == 0 || file_name_offset + file_name_len > content.len() {
            return None;
        }

        t.index += file_name_offset + file_name_len;
        Some(&content[file_name_offset..file_name_offset + file_name_len])
    }
}

// zipContains returns true if the zip file headers from in contain any of the paths.
pub(crate) fn zip_contains(content: &[u8], paths: Vec<&[u8]>) -> bool {
    let mut tokenizer = ZipTokenizer {
        content: content.to_vec(),
        index: 0,
    };
    while let Some(token) = tokenizer.next() {
        for p in paths.clone() {
            if token.starts_with(p) {
                return true;
            }
        }
    }
    false
}

// CRX matches a Chrome extension file: a zip archive prepended by a package header.
pub(crate) struct CrxDetector;

impl MimeDetector for CrxDetector {
    fn detect(&self, content: &[u8], limit: usize) -> bool {
        const MIN_HEADER_LEN: usize = 16;
        if content.len() < MIN_HEADER_LEN || !content.starts_with(b"Cr24") {
            return false;
        }
        let pubkey_len = LittleEndian::read_u32(&content[8..12]) as usize;
        let sig_len = LittleEndian::read_u32(&content[12..16]) as usize;
        let zip_offset = MIN_HEADER_LEN + pubkey_len + sig_len;
        if content.len() < zip_offset {
            return false;
        }
        ZipDetector {}.detect(&content[zip_offset..], limit)
    }
}

// InstallShieldCab matches an InstallShield Cabinet archive file.
pub(crate) struct InstallShieldCabDetector;

impl MimeDetector for InstallShieldCabDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        content.len() > 7
            && &content[0..4] == b"ISc("
            && content[6] == 0
            && (content[7] == 1 || content[7] == 2 || content[7] == 4)
    }
}
