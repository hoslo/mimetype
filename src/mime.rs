use std::{
    fmt::Debug,
    sync::atomic::{AtomicUsize, Ordering},
    vec,
};

use crate::{magic::base::EmptyDetector, tree::ROOT};

const DEFAULT_LIMIT: usize = 3072;

static RATE_LIMIT: AtomicUsize = AtomicUsize::new(DEFAULT_LIMIT);

/// Set the rate limit for the MIME detection.
/// If the content is larger than the limit, only the first `limit` bytes will be used.
/// The default limit is 3072 bytes.
/// If the limit is set to 0, the whole content will be used.
pub fn set_rate_limit(limit: usize) {
    RATE_LIMIT.store(limit, Ordering::Relaxed);
}

pub(crate) trait MimeDetector: Send + Sync {
    fn detect(&self, content: &[u8], limit: usize) -> bool;
}

/// MIME struct holds information about a file format: the string representation
/// of the MIME type, the extension and the parent file format.
pub struct Mime {
    /// The string representation of the MIME type.
    pub mime: String,
    /// The aliases of the MIME type.
    pub aliases: Vec<String>,
    /// The extension of the file format.
    pub extension: String,
    detector: Box<dyn MimeDetector>,
    chilren: Vec<Mime>,
}

impl Mime {
    pub(crate) fn new<T>(mime: String, extension: String, detector: T) -> Self
    where
        for<'r> T: MimeDetector + 'r,
    {
        Mime {
            mime,
            aliases: Vec::new(),
            extension,
            detector: Box::new(detector),
            chilren: Vec::new(),
        }
    }

    pub(crate) fn aliases(mut self, aliases: Vec<&'static str>) -> Self {
        self.aliases = aliases.iter().map(|s| s.to_string()).collect();
        self
    }

    pub(crate) fn children(mut self, children: Vec<Mime>) -> Self {
        self.chilren = children;
        self
    }

    fn match_mime(&self, content: &[u8], limit: usize) -> Mime {
        for c in &self.chilren {
            if c.detector.detect(content, limit) {
                return c.match_mime(content, limit);
            }
        }

        self.clone()
    }
}

impl Debug for Mime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mime")
            .field("mime", &self.mime)
            .field("aliases", &self.aliases)
            .field("extension", &self.extension)
            .finish()
    }
}

impl Clone for Mime {
    fn clone(&self) -> Self {
        Mime {
            mime: self.mime.clone(),
            aliases: self.aliases.clone(),
            extension: self.extension.clone(),
            detector: Box::new(EmptyDetector {}),
            chilren: self.chilren.clone(),
        }
    }
}

/// Detect the MIME type of the content.
/// If the content is larger than the rate limit, only the first `limit` bytes will be used.
/// The default limit is 3072 bytes.
pub fn detect(content: &[u8]) -> Mime {
    let limit = RATE_LIMIT.load(Ordering::Relaxed);
    let mut content = content;
    if limit > 0 && content.len() > limit {
        content = &content[..limit];
    }

    ROOT.match_mime(content, limit)
}

#[cfg(feature = "async")]
use tokio::io::{AsyncRead, AsyncReadExt};

#[cfg(feature = "async")]
/// Detect the MIME type of the content.
/// If the content is larger than the rate limit, only the first `limit` bytes will be used.
/// The default limit is 3072 bytes.
pub async fn detech_from_reader<T: AsyncRead + Unpin>(reader: T) -> Mime {
    let limit = RATE_LIMIT.load(Ordering::Relaxed);
    let mut reader = reader;
    let mut content = vec![];
    if limit > 0 {
        let mut limit_content = vec![0; limit];
        let n = reader.read(&mut limit_content).await.unwrap();
        if n == 0 {
            return ROOT.clone();
        }
        content = limit_content[..n].to_vec();
    } else {
        let n = reader.read_to_end(&mut content).await.unwrap_or(0);
        if n == 0 {
            return ROOT.clone();
        }
    }

    ROOT.match_mime(&content, limit)
}

#[cfg(feature = "sync")]
use std::io::Read;

#[cfg(feature = "sync")]
/// Detect the MIME type of the content.
/// If the content is larger than the rate limit, only the first `limit` bytes will be used.
/// The default limit is 3072 bytes.
pub fn detech_from_reader<T: Read>(reader: T) -> Mime {
    let limit = RATE_LIMIT.load(Ordering::Relaxed);
    let mut reader = reader;
    let mut content = vec![];
    if limit > 0 {
        let mut limit_content = vec![0; limit];
        let n = reader.read(&mut limit_content).unwrap();
        if n == 0 {
            return ROOT.clone();
        }
        content = limit_content[..n].to_vec();
    } else {
        let n = reader.read_to_end(&mut content).unwrap_or(0);
        if n == 0 {
            return ROOT.clone();
        }
    }

    ROOT.match_mime(&content, limit)
}
