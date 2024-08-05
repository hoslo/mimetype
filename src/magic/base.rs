use crate::mime::MimeDetector;

pub(crate) trait BytesExt {
    fn index(self, b: &[u8]) -> Option<usize>;
}

impl BytesExt for &[u8] {
    fn index(self, b: &[u8]) -> Option<usize> {
        self.windows(b.len()).position(|window| window == b)
    }
}

pub struct EmptyDetector;

impl MimeDetector for EmptyDetector {
    fn detect(&self, _: &[u8], _: usize) -> bool {
        true
    }
}

pub(crate) struct PrefixDetector {
    pub sigs: Vec<&'static [u8]>,
}

impl MimeDetector for PrefixDetector {
    fn detect(&self, content: &[u8], _limit: usize) -> bool {
        for sig in &self.sigs {
            if content.starts_with(sig) {
                return true;
            }
        }
        false
    }
}

pub(crate) struct OffsetDetector {
    pub offset: usize,
    pub sig: &'static [u8],
}

impl MimeDetector for OffsetDetector {
    fn detect(&self, content: &[u8], _limit: usize) -> bool {
        content.len() > self.offset && content[self.offset..].starts_with(self.sig)
    }
}

fn ci_check(sig: &&[u8], content: &&[u8]) -> bool {
    if content.len() < sig.len() + 1 {
        return false;
    }
    for (i, b) in sig.iter().enumerate() {
        let mut db = content[i];
        if db.is_ascii_uppercase() {
            db &= 0xDF;
        }
        if b != &db {
            return false;
        }
    }
    true
}

pub(crate) struct CiPrefixDetector {
    pub sigs: Vec<&'static [u8]>,
}

impl MimeDetector for CiPrefixDetector {
    fn detect(&self, content: &[u8], _limit: usize) -> bool {
        for sig in &self.sigs {
            if ci_check(sig, &content) {
                return true;
            }
        }
        false
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub(crate) struct XmlSig {
    local_name: &'static [u8],
    xmlns: &'static [u8],
}

#[allow(unused)]
impl XmlSig {
    pub(crate) fn new(local_name: &'static str, xmlns: &'static str) -> Self {
        let mut ret = XmlSig {
            local_name: &[],
            xmlns: xmlns.as_bytes(),
        };

        if !local_name.is_empty() {
            ret.local_name = format!("<{}", local_name).leak().as_bytes();
        }
        ret
    }
}

#[allow(unused)]
fn xml_check(sig: &XmlSig, content: &[u8]) -> bool {
    let content = &content[..content.len().min(512)];
    if sig.local_name.is_empty() {
        if let Some(i) = content.index(sig.xmlns) {
            return i > 0;
        } else {
            return false;
        }
    }
    if sig.xmlns.is_empty() {
        if let Some(i) = content.index(sig.local_name) {
            return i > 0;
        } else {
            return false;
        }
    }
    if let Some(i) = content.index(sig.local_name) {
        let Some(j) = content.index(sig.xmlns) else {
            return false;
        };
        return i < j;
    }
    false
}

#[allow(unused)]
pub(crate) struct XmlDetector {
    pub sigs: Vec<XmlSig>,
}

impl MimeDetector for XmlDetector {
    fn detect(&self, content: &[u8], _limit: usize) -> bool {
        let content = trim_left_ws(content);
        if content.is_empty() {
            return false;
        }
        for sig in &self.sigs {
            if xml_check(sig, content) {
                return true;
            }
        }
        false
    }
}

pub(crate) fn trim_left_ws(content: &[u8]) -> &[u8] {
    let mut start = 0;
    for c in content.iter() {
        if !c.is_ascii_whitespace() {
            break;
        }
        start += 1;
    }
    &content[start..]
}

pub(crate) fn trim_right_ws(content: &[u8]) -> &[u8] {
    let mut end = content.len();
    for c in content.iter().rev() {
        if !c.is_ascii_whitespace() {
            break;
        }
        end -= 1;
    }
    &content[..end]
}

fn first_line(input: &[u8]) -> &[u8] {
    if let Some(pos) = input.iter().position(|&b| b == b'\n') {
        &input[..pos] // 返回从开头到换行符之前的字节
    } else {
        input // 如果没有换行符，则返回整个字节数组
    }
}

fn mark_up_check(sig: &[u8], content: &[u8]) -> bool {
    if content.len() < sig.len() + 1 {
        return false;
    }
    for (i, b) in sig.iter().enumerate() {
        let mut db = content[i];
        if b'A' <= *b && *b <= b'Z' {
            db &= 0xDF;
        }
        if *b != db {
            return false;
        }
    }

    let db = content[sig.len()];
    if db != b' ' && db != b'>' {
        return false;
    }
    true
}

pub(crate) struct MarkUpDetector {
    pub sigs: Vec<&'static [u8]>,
}

impl MimeDetector for MarkUpDetector {
    fn detect(&self, content: &[u8], _limit: usize) -> bool {
        let mut content = content;
        if content.starts_with(&[0xEF, 0xBB, 0xBF]) {
            content = trim_left_ws(&content[3..]);
        } else {
            content = trim_left_ws(content);
        }
        if content.is_empty() {
            return false;
        }
        for sig in &self.sigs {
            if mark_up_check(sig, content) {
                return true;
            }
        }
        false
    }
}

pub(crate) struct FtypDetector {
    pub sigs: Vec<&'static [u8]>,
}

impl MimeDetector for FtypDetector {
    fn detect(&self, content: &[u8], _limit: usize) -> bool {
        if content.len() < 12 {
            return false;
        }
        if &content[4..8] != b"ftyp" {
            return false;
        }
        for &sig in &self.sigs {
            if &content[8..12] == sig {
                return true;
            }
        }
        false
    }
}

fn shebang_check(sig: &[u8], content: &[u8]) -> bool {
    if content.len() < sig.len() + 2 {
        return false;
    }
    if content[0] != b'#' || content[1] != b'!' {
        return false;
    }
    trim_left_ws(trim_right_ws(&content[2..])) == sig
}

pub(crate) struct SheBangDetector {
    pub sigs: Vec<&'static [u8]>,
}

impl MimeDetector for SheBangDetector {
    fn detect(&self, content: &[u8], _limit: usize) -> bool {
        for sig in &self.sigs {
            if shebang_check(sig, first_line(content)) {
                return true;
            }
        }
        false
    }
}
