use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};

use crate::{magic::base::OffsetDetector, mime::MimeDetector};

fn class_or_mach_ofat(content: &[u8]) -> bool {
    if content.len() < 8 {
        return false;
    }

    content.starts_with(&[0xCA, 0xFE, 0xBA, 0xBE])
}

// Class matches a java class file.
pub(crate) struct ClassDetector;

impl MimeDetector for ClassDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        class_or_mach_ofat(content) && content[7] > 30
    }
}

// MachO matches Mach-O binaries format.
pub(crate) struct MachODetector;

impl MimeDetector for MachODetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        if class_or_mach_ofat(content) && content[7] < 20 {
            return true;
        }

        if content.len() < 4 {
            return false;
        }

        let Ok(be) = std::io::Cursor::new(&content).read_u32::<BigEndian>() else {
            return false;
        };
        let Ok(le) = std::io::Cursor::new(&content).read_u32::<LittleEndian>() else {
            return false;
        };

        be == 0xfeedface || le == 0xfeedface || be == 0xfeedfacf || le == 0xfeedfacf
    }
}

// ElfObj matches an object file.
pub(crate) struct ElfObjDetector;

impl MimeDetector for ElfObjDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        content.len() > 17
            && ((content[16] == 0x01 && content[17] == 0x00)
                || (content[16] == 0x00 && content[17] == 0x01))
    }
}

// ElfExe matches an executable file.
pub(crate) struct ElfExeDetector;

impl MimeDetector for ElfExeDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        content.len() > 17
            && ((content[16] == 0x02 && content[17] == 0x00)
                || (content[16] == 0x00 && content[17] == 0x02))
    }
}

// ElfLib matches a shared library file.
pub(crate) struct ElfLibDetector;

impl MimeDetector for ElfLibDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        content.len() > 17
            && ((content[16] == 0x03 && content[17] == 0x00)
                || (content[16] == 0x00 && content[17] == 0x03))
    }
}

// ElfDump matches a core dump file.
pub(crate) struct ElfDumpDetector;

impl MimeDetector for ElfDumpDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        content.len() > 17
            && ((content[16] == 0x04 && content[17] == 0x00)
                || (content[16] == 0x00 && content[17] == 0x04))
    }
}

// Ttf matches a TrueType font file.
pub(crate) struct TtfDetector;

impl MimeDetector for TtfDetector {
    fn detect(&self, content: &[u8], limit: usize) -> bool {
        if !content.starts_with(&[0x00, 0x01, 0x00, 0x00]) {
            return false;
        }
        !OffsetDetector {
            sig: b"Standard ACE DB",
            offset: 4,
        }
        .detect(content, limit)
            && !OffsetDetector {
                sig: b"Standard Jet DB",
                offset: 4,
            }
            .detect(content, limit)
    }
}

// Ttc matches a TrueType Collection font file.
pub(crate) struct TtcDetector;

impl MimeDetector for TtcDetector {
    fn detect(&self, content: &[u8], _limit: usize) -> bool {
        content.len() > 7
            && content.starts_with(b"ttcf")
            && (content[4..8] == [0x00, 0x01, 0x00, 0x00]
                || content[4..8] == [0x00, 0x02, 0x00, 0x00])
    }
}

// Eot matches an Embedded OpenType font file.
pub(crate) struct EotDetector;

impl MimeDetector for EotDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        content.len() > 35
            && content[34..36] == [0x4C, 0x50]
            && (content[8..11] == [0x02, 0x00, 0x01]
                || content[8..11] == [0x01, 0x00, 0x00]
                || content[8..11] == [0x02, 0x00, 0x02])
    }
}

// Dbf matches a dBase file.
// https://www.dbase.com/Knowledgebase/INT/db7_file_fmt.htm
pub(crate) struct DbfDetector;

impl MimeDetector for DbfDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        if content.len() < 68 {
            return false;
        }

        // 3rd and 4th bytes contain the last update month and day of month.
        if !(0 < content[2] && content[2] < 13 && 0 < content[3] && content[3] < 32) {
            return false;
        }

        // 12, 13, 30, 31 are reserved bytes and always filled with 0x00.
        if content[12] != 0x00 || content[13] != 0x00 || content[30] != 0x00 || content[31] != 0x00
        {
            return false;
        }
        // Production MDX flag;
        // 0x01 if a production .MDX file exists for this table;
        // 0x00 if no .MDX file exists.
        if content[28] > 0x01 {
            return false;
        }

        // dbf type is dictated by the first byte.
        let dbf_types = &[
            0x02, 0x03, 0x04, 0x05, 0x30, 0x31, 0x32, 0x42, 0x62, 0x7B, 0x82, 0x83, 0x87, 0x8A,
            0x8B, 0x8E, 0xB3, 0xCB, 0xE5, 0xF5, 0xF4, 0xFB,
        ];
        for &b in dbf_types {
            if content[0] == b {
                return true;
            }
        }

        false
    }
}

// Dcm matches a DICOM medical format file.
pub(crate) struct DcmDetector;

impl MimeDetector for DcmDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        content.len() > 132 && content[128..132] == [0x44, 0x49, 0x43, 0x4D]
    }
}

// DjVu matches a DjVu file.
pub(crate) struct DjVuDetector;

impl MimeDetector for DjVuDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        if content.len() < 12 {
            return false;
        }
        if !content.starts_with(&[0x41, 0x54, 0x26, 0x54, 0x46, 0x4F, 0x52, 0x4D]) {
            return false;
        }
        &content[12..] == b"DJVM"
            || &content[12..] == b"DJVU"
            || &content[12..] == b"DJVI"
            || &content[12..] == b"THUM"
    }
}

// Marc matches a MARC21 (MAchine-Readable Cataloging) file.
pub(crate) struct MarcDetector;

impl MimeDetector for MarcDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        // File is at least 24 bytes ("leader" field size).
        if content.len() < 24 {
            return false;
        }

        // Fixed bytes at offset 20.
        if &content[20..24] != b"4500" {
            return false;
        }

        // First 5 bytes are ASCII digits.
        for &b in content.iter().take(5) {
            if b.is_ascii_digit() {
                return false;
            }
        }

        // Field terminator is present in first 2048 bytes.
        content[..2048.min(content.len())].contains(&0x1E)
    }
}

// Zstd matches a Zstandard archive file.
pub(crate) struct ZstdDetector;

impl MimeDetector for ZstdDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        content.len() >= 4
            && (0x22 <= content[0] && content[0] <= 0x28 || content[0] == 0x1E)
            && content[1..].starts_with(&[0xB5, 0x2F, 0xFD])
    }
}

// TzIf matches a Time Zone Information Format (TZif) file.
// See more: https://tools.ietf.org/id/draft-murchison-tzdist-tzif-00.html#rfc.section.3
// Its header structure is shown below:
// 	+---------------+---+
// 	|  magic    (4) | <-+-- version (1)
// 	+---------------+---+---------------------------------------+
// 	|           [unused - reserved for future use] (15)         |
// 	+---------------+---------------+---------------+-----------+
// 	|  isutccnt (4) |  isstdcnt (4) |  leapcnt  (4) |
// 	+---------------+---------------+---------------+
// 	|  timecnt  (4) |  typecnt  (4) |  charcnt  (4) |
pub(crate) struct TzIfDetector;

impl MimeDetector for TzIfDetector {
    fn detect(&self, content: &[u8], _: usize) -> bool {
        // File is at least 44 bytes (header size).
        if content.len() < 44 {
            return false;
        }

        if !content.starts_with(b"TZif") {
            return false;
        }

        // Field "typecnt" MUST not be zero.
        if BigEndian::read_u32(&content[36..40]) == 0 {
            return false;
        }

        // Version has to be NUL (0x00), '2' (0x32) or '3' (0x33).
        content[4] == 0x00 || content[4] == 0x32 || content[4] == 0x33
    }
}
