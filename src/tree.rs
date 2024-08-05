use std::{sync::LazyLock, vec};

use crate::{
    magic::{archive::*, audio::*, base::*, binary::*, ftyp::QuickTimeDetector, image::*, text::*},
    mime::Mime,
};
// use crate::base::text::HTML;

pub(crate) static ROOT: LazyLock<Mime> = LazyLock::new(|| {
    // Xpm matches X PixMap image data.
    let xpm = Mime::new(
        "image/x-xpixmap".to_string(),
        ".xpm".to_string(),
        PrefixDetector {
            sigs: vec![&[0x2F, 0x2A, 0x20, 0x58, 0x50, 0x4D, 0x20, 0x2A, 0x2F]],
        },
    );

    // 7z
    let seven_z = Mime::new(
        "application/x-7z-compressed".to_string(),
        ".7z".to_string(),
        PrefixDetector {
            sigs: vec![&[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C]],
        },
    );

    // xlsx
    let xlsx = Mime::new(
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
        ".xlsx".to_string(),
        XlsxDetector {},
    );

    // docx
    let docx = Mime::new(
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
        ".docx".to_string(),
        DocxDetector {},
    );

    // pptx
    let pptx = Mime::new(
        "application/vnd.openxmlformats-officedocument.presentationml.presentation".to_string(),
        ".pptx".to_string(),
        PptxDetector {},
    );

    // Epub matches an EPUB file.
    let epub = Mime::new(
        "application/epub+zip".to_string(),
        ".epub".to_string(),
        OffsetDetector {
            sig: b"mimetypeapplication/epub+zip",
            offset: 30,
        },
    );

    // Jar matches a Java archive file.
    let jar = Mime::new(
        "application/jar".to_string(),
        ".jar".to_string(),
        JarDetector {},
    );

    // Ott matches an OpenDocument Text Template file.
    let ott = Mime::new(
        "application/vnd.oasis.opendocument.text-template".to_string(),
        ".ott".to_string(),
        OffsetDetector {
            sig: b"mimetypeapplication/vnd.oasis.opendocument.text-template",
            offset: 30,
        },
    )
    .aliases(vec!["application/x-vnd.oasis.opendocument.text-template"]);

    // Odt matches an OpenDocument Text file.
    let odt = Mime::new(
        "application/vnd.oasis.opendocument.text".to_string(),
        ".odt".to_string(),
        OffsetDetector {
            sig: b"mimetypeapplication/vnd.oasis.opendocument.text",
            offset: 30,
        },
    )
    .aliases(vec!["application/x-vnd.oasis.opendocument.text"])
    .children(vec![ott]);

    // Ots matches an OpenDocument Spreadsheet Template file.
    let ots = Mime::new(
        "application/vnd.oasis.opendocument.spreadsheet-template".to_string(),
        ".ots".to_string(),
        OffsetDetector {
            sig: b"mimetypeapplication/vnd.oasis.opendocument.spreadsheet-template",
            offset: 30,
        },
    )
    .aliases(vec![
        "application/x-vnd.oasis.opendocument.spreadsheet-template",
    ]);

    // Ods matches an OpenDocument Spreadsheet file.
    let ods = Mime::new(
        "application/vnd.oasis.opendocument.spreadsheet".to_string(),
        ".ods".to_string(),
        OffsetDetector {
            sig: b"mimetypeapplication/vnd.oasis.opendocument.spreadsheet",
            offset: 30,
        },
    )
    .aliases(vec!["application/x-vnd.oasis.opendocument.spreadsheet"])
    .children(vec![ots]);

    // Otp matches an OpenDocument Presentation Template file.
    let otp = Mime::new(
        "application/vnd.oasis.opendocument.presentation-template".to_string(),
        ".otp".to_string(),
        OffsetDetector {
            sig: b"mimetypeapplication/vnd.oasis.opendocument.presentation-template",
            offset: 30,
        },
    )
    .aliases(vec![
        "application/x-vnd.oasis.opendocument.presentation-template",
    ]);

    // Odp matches an OpenDocument Presentation file.
    let odp = Mime::new(
        "application/vnd.oasis.opendocument.presentation".to_string(),
        ".odp".to_string(),
        OffsetDetector {
            sig: b"mimetypeapplication/vnd.oasis.opendocument.presentation",
            offset: 30,
        },
    )
    .aliases(vec!["application/x-vnd.oasis.opendocument.presentation"])
    .children(vec![otp]);

    // Otg matches an OpenDocument Drawing Template file.
    let otg = Mime::new(
        "application/vnd.oasis.opendocument.graphics-template".to_string(),
        ".otg".to_string(),
        OffsetDetector {
            sig: b"mimetypeapplication/vnd.oasis.opendocument.graphics-template",
            offset: 30,
        },
    )
    .aliases(vec![
        "application/x-vnd.oasis.opendocument.graphics-template",
    ]);

    // Odg matches an OpenDocument Drawing file.
    let odg = Mime::new(
        "application/vnd.oasis.opendocument.graphics".to_string(),
        ".odg".to_string(),
        OffsetDetector {
            sig: b"mimetypeapplication/vnd.oasis.opendocument.graphics",
            offset: 30,
        },
    )
    .aliases(vec!["application/x-vnd.oasis.opendocument.graphics"])
    .children(vec![otg]);

    // Odf matches an OpenDocument Formula file.
    let odf = Mime::new(
        "application/vnd.oasis.opendocument.formula".to_string(),
        ".odf".to_string(),
        OffsetDetector {
            sig: b"mimetypeapplication/vnd.oasis.opendocument.formula",
            offset: 30,
        },
    )
    .aliases(vec!["application/x-vnd.oasis.opendocument.formula"]);

    // Odc matches an OpenDocument Chart file.
    let odc = Mime::new(
        "application/vnd.oasis.opendocument.chart".to_string(),
        ".odc".to_string(),
        OffsetDetector {
            sig: b"mimetypeapplication/vnd.oasis.opendocument.chart",
            offset: 30,
        },
    )
    .aliases(vec!["application/x-vnd.oasis.opendocument.chart"]);

    // Sxc matches an OpenOffice Spreadsheet file.
    let sxc = Mime::new(
        "application/vnd.sun.xml.calc".to_string(),
        ".sxc".to_string(),
        OffsetDetector {
            sig: b"mimetypeapplication/vnd.sun.xml.calc",
            offset: 30,
        },
    )
    .aliases(vec!["application/x-vnd.sun.xml.calc"]);

    // zip matches a zip archive.
    let zip = Mime::new(
        "application/zip".to_string(),
        ".zip".to_string(),
        ZipDetector {},
    )
    .children(vec![
        xlsx, docx, pptx, epub, jar, odt, ods, odp, odg, odf, odc, sxc,
    ]);

    // Pdf matches a Portable Document Format file.
    // https://github.com/file/file/blob/11010cc805546a3e35597e67e1129a481aed40e8/magic/Magdir/pdf
    let pdf = Mime::new(
        "application/pdf".to_string(),
        ".pdf".to_string(),
        PrefixDetector {
            sigs: vec![
                // usual pdf signature
                b"%PDF-",
                // new-line prefixed signature
                b"\x0a%PDF-",
                // UTF-8 BOM prefixed signature
                b"\xef\xbb\xbf%PDF-",
            ],
        },
    )
    .aliases(vec!["application/x-pdf"]);

    // Fdf matches a Forms Data Format file.
    let fdf = Mime::new(
        "application/vnd.fdf".to_string(),
        ".fdf".to_string(),
        PrefixDetector {
            sigs: vec![b"%FDF"],
        },
    );

    // Msi matches a Microsoft Windows Installer file.
    // http://fileformats.archiveteam.org/wiki/Microsoft_Compound_File
    let msi = Mime::new(
        "application/x-ms-installer".to_string(),
        ".msi".to_string(),
        MsiDetector {},
    )
    .aliases(vec!["application/x-windows-installer", "application/x-msi"]);

    // Aaf matches an Advanced Authoring Format file.
    let aaf = Mime::new(
        "application/octet-stream".to_string(),
        ".aaf".to_string(),
        AafDetector {},
    );

    // Msg matches a Microsoft Outlook email file.
    let msg = Mime::new(
        "application/vnd.ms-outlook".to_string(),
        ".msg".to_string(),
        MsgDetector {},
    );

    // Xls matches a Microsoft Excel 97-2003 file.
    let xls = Mime::new(
        "application/vnd.ms-excel".to_string(),
        ".xls".to_string(),
        XlsDetector {},
    )
    .aliases(vec!["application/msexcel"]);

    // Pub matches a Microsoft Publisher file.
    let publisher = Mime::new(
        "application/vnd.ms-publisher".to_string(),
        ".pub".to_string(),
        PubDetector {},
    );

    // Ppt matches a Microsoft PowerPoint 97-2003 file or a PowerPoint 95 presentation.
    let ppt = Mime::new(
        "application/vnd.ms-powerpoint".to_string(),
        ".ppt".to_string(),
        PptDetector {},
    )
    .aliases(vec!["application/mspowerpoint"]);

    // Doc matches a Microsoft Word 97-2003 file.
    let doc = Mime::new(
        "application/msword".to_string(),
        ".doc".to_string(),
        DocDetector {},
    )
    .aliases(vec!["application/vnd.ms-word"]);

    // Ole matches an Open Linking and Embedding file.
    //
    // https://en.wikipedia.org/wiki/Object_Linking_and_Embedding
    let ole = Mime::new(
        "application/x-ole-storage".to_string(),
        "".to_string(),
        PrefixDetector {
            sigs: vec![&[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]],
        },
    )
    .children(vec![msi, aaf, msg, xls, publisher, ppt, doc]);

    // Ps matches a PostScript file.
    let ps = Mime::new(
        "application/postscript".to_string(),
        ".ps".to_string(),
        PrefixDetector {
            sigs: vec![b"%!PS-Adobe-"],
        },
    );

    // Psd matches a Photoshop Document file.
    let psd = Mime::new(
        "image/vnd.adobe.photoshop".to_string(),
        ".psd".to_string(),
        PrefixDetector {
            sigs: vec![b"8BPS"],
        },
    )
    .aliases(vec!["image/x-psd", "application/photoshop"]);

    // P7s matches an .p7s signature File (PEM, Base64).
    let p7s = Mime::new(
        "application/pkcs7-signature".to_string(),
        ".p7s".to_string(),
        P7sDetector {},
    );

    // OggAudio matches an audio ogg file.
    let ogg_audio = Mime::new(
        "audio/ogg".to_string(),
        ".ogg".to_string(),
        OggAudioDetector {},
    );

    // OggVideo matches a video ogg file.
    let ogg_video = Mime::new(
        "video/ogg".to_string(),
        ".ogv".to_string(),
        OggVideoDetector {},
    );

    // Ogg matches an Ogg file.
    let ogg = Mime::new(
        "application/ogg".to_string(),
        ".ogg".to_string(),
        PrefixDetector {
            sigs: vec![b"\x4F\x67\x67\x53\x00"],
        },
    )
    .aliases(vec!["application/x-ogg"])
    .children(vec![ogg_audio, ogg_video]);

    // Apng
    let apng = Mime::new(
        "image/vnd.mozilla.apng".to_string(),
        ".apng".to_string(),
        OffsetDetector {
            offset: 37,
            sig: b"acTL",
        },
    );

    // png
    let png = Mime::new(
        "image/png".to_string(),
        ".png".to_string(),
        PrefixDetector {
            sigs: vec![&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]],
        },
    )
    .children(vec![apng]);

    // Jpg
    let jpg = Mime::new(
        "image/jpeg".to_string(),
        ".jpg".to_string(),
        PrefixDetector {
            sigs: vec![&[0xFF, 0xD8, 0xFF]],
        },
    );

    // Jxl matches JPEG XL image file.
    let jxl = Mime::new("image/jxl".to_string(), ".jxl".to_string(), JxlDetector {});

    // Jp2 matches a JPEG 2000 Image file (ISO 15444-1).
    let jp2 = Mime::new(
        "image/jp2".to_string(),
        ".jp2".to_string(),
        Jpeg2kDetector {
            sig: &[0x6a, 0x70, 0x32, 0x20],
        },
    );

    // Jpx matches a JPEG 2000 Image file (ISO 15444-2).
    let jpx = Mime::new(
        "image/jpx".to_string(),
        ".jpf".to_string(),
        Jpeg2kDetector {
            sig: &[0x6a, 0x70, 0x78, 0x20],
        },
    );

    // Jpm matches a JPEG 2000 Image file (ISO 15444-6).
    let jpm = Mime::new(
        "image/jpm".to_string(),
        ".jpm".to_string(),
        Jpeg2kDetector {
            sig: &[0x6a, 0x70, 0x6D, 0x20],
        },
    );

    // Jxs matches a JPEG XS coded image file (ISO/IEC 21122-3).
    let jxs = Mime::new(
        "image/jxs".to_string(),
        ".jxs".to_string(),
        PrefixDetector {
            sigs: vec![&[
                0x00, 0x00, 0x00, 0x0C, 0x4A, 0x58, 0x53, 0x20, 0x0D, 0x0A, 0x87, 0x0A,
            ]],
        },
    );

    // Gif matches a Graphics Interchange Format file.
    let gif = Mime::new(
        "image/gif".to_string(),
        ".gif".to_string(),
        PrefixDetector {
            sigs: vec![b"GIF87a", b"GIF89a"],
        },
    );

    // Webp matches a WebP file.
    let webp = Mime::new(
        "image/webp".to_string(),
        ".webp".to_string(),
        WebpDetector {},
    );

    // Exe matches a Windows/DOS executable file.
    let exe = Mime::new(
        "application/vnd.microsoft.portable-executable".to_string(),
        ".exe".to_string(),
        PrefixDetector {
            sigs: vec![&[0x4D, 0x5A]],
        },
    );

    // ElfObj matches an object file.
    let elf_obj = Mime::new(
        "application/x-object".to_string(),
        "".to_string(),
        ElfObjDetector {},
    );

    // ElfExe matches an executable file.
    let elf_exe = Mime::new(
        "application/x-executable".to_string(),
        "".to_string(),
        ElfExeDetector {},
    );

    // ElfLib matches a shared library file.
    let elf_lib = Mime::new(
        "application/x-sharedlib".to_string(),
        ".so".to_string(),
        ElfLibDetector {},
    );

    // ElfDump matches a core dump file.
    let elf_dump = Mime::new(
        "application/x-coredump".to_string(),
        "".to_string(),
        ElfDumpDetector {},
    );

    // Elf matches an Executable and Linkable Format file.
    let elf = Mime::new(
        "application/x-elf".to_string(),
        ".elf".to_string(),
        PrefixDetector {
            sigs: vec![&[0x7F, 0x45, 0x4C, 0x46]],
        },
    )
    .children(vec![elf_obj, elf_exe, elf_lib, elf_dump]);

    // Deb matches a Debian package file.
    let deb = Mime::new(
        "application/vnd.debian.binary-package".to_string(),
        ".deb".to_string(),
        OffsetDetector {
            sig: &[
                0x64, 0x65, 0x62, 0x69, 0x61, 0x6E, 0x2D, 0x62, 0x69, 0x6E, 0x61, 0x72, 0x79,
            ],
            offset: 8,
        },
    );

    // Ar matches an ar (Unix) archive file.
    let ar = Mime::new(
        "application/x-archive".to_string(),
        ".a".to_string(),
        PrefixDetector {
            sigs: vec![&[0x21, 0x3C, 0x61, 0x72, 0x63, 0x68, 0x3E]],
        },
    )
    .children(vec![deb]);

    // Tar matches a (t)ape (ar)chive file.
    let tar = Mime::new(
        "application/x-tar".to_string(),
        ".tar".to_string(),
        TarDetector {},
    );

    // Xar matches an eXtensible ARchive format file.
    let xar = Mime::new(
        "application/x-xar".to_string(),
        ".xar".to_string(),
        PrefixDetector {
            sigs: vec![&[0x78, 0x61, 0x72, 0x21]],
        },
    );

    // Bz2 matches a bzip2 file.
    let bz2 = Mime::new(
        "application/x-bzip2".to_string(),
        ".bz2".to_string(),
        PrefixDetector {
            sigs: vec![&[0x42, 0x5A, 0x68]],
        },
    );

    // Fits matches an Flexible Image Transport System file.
    let fits = Mime::new(
        "application/fits".to_string(),
        ".fits".to_string(),
        PrefixDetector {
            sigs: vec![&[
                0x53, 0x49, 0x4D, 0x50, 0x4C, 0x45, 0x20, 0x20, 0x3D, 0x20, 0x20, 0x20, 0x20, 0x20,
                0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
                0x20, 0x54,
            ]],
        },
    );

    // Tiff matches a Tagged Image File Format file.
    let tiff = Mime::new(
        "image/tiff".to_string(),
        ".tiff".to_string(),
        PrefixDetector {
            sigs: vec![&[0x49, 0x49, 0x2A, 0x00], &[0x4D, 0x4D, 0x00, 0x2A]],
        },
    );

    // Bmp matches a bitmap image file.
    let bmp = Mime::new(
        "image/bmp".to_string(),
        ".bmp".to_string(),
        PrefixDetector {
            sigs: vec![&[0x42, 0x4D]],
        },
    )
    .aliases(vec!["image/x-bmp", "image/x-ms-bmp"]);

    // Ico matches an ICO file.
    let ico = Mime::new(
        "image/x-icon".to_string(),
        ".ico".to_string(),
        PrefixDetector {
            sigs: vec![&[0x00, 0x00, 0x01, 0x00], &[0x00, 0x00, 0x02, 0x00]],
        },
    );

    // Mp3 matches an mp3 file.
    let mp3 = Mime::new("audio/mpeg".to_string(), ".mp3".to_string(), Mp3Detector {})
        .aliases(vec!["audio/x-mpeg", "audio/mp3"]);

    // Flac matches a Free Lossless Audio Codec file.
    let flac = Mime::new(
        "audio/flac".to_string(),
        ".flac".to_string(),
        PrefixDetector {
            sigs: vec![b"\x66\x4C\x61\x43\x00\x00\x00\x22"],
        },
    );

    // Midi matches a Musical Instrument Digital Interface file.
    let midi = Mime::new(
        "audio/midi".to_string(),
        ".midi".to_string(),
        PrefixDetector {
            sigs: vec![b"\x4D\x54\x68\x64"],
        },
    );

    // Ape matches a Monkey's Audio file.
    let ape = Mime::new(
        "audio/ape".to_string(),
        ".ape".to_string(),
        PrefixDetector {
            sigs: vec![b"\x4D\x41\x43\x20\x96\x0F\x00\x00\x34\x00\x00\x00\x18\x00\x00\x00\x90\xE3"],
        },
    );

    // MusePack matches a Musepack file.
    let musepack = Mime::new(
        "audio/musepack".to_string(),
        ".mpc".to_string(),
        PrefixDetector {
            sigs: vec![b"MPCK"],
        },
    );

    // Amr matches an Adaptive Multi-Rate file.
    let amr = Mime::new(
        "audio/amr".to_string(),
        ".amr".to_string(),
        PrefixDetector {
            sigs: vec![b"\x23\x21\x41\x4D\x52"],
        },
    )
    .aliases(vec!["audio/amr-nb"]);

    // Wav matches a Waveform Audio File Format file.
    let wav = Mime::new("audio/wav".to_string(), ".wav".to_string(), WavDetector {}).aliases(vec![
        "audio/x-wav",
        "audio/vnd.wave",
        "audio/wave",
    ]);

    // Aiff matches Audio Interchange File Format file.
    let aiff = Mime::new(
        "audio/aiff".to_string(),
        ".aiff".to_string(),
        AiffDetector {},
    )
    .aliases(vec!["audio/x-aiff"]);

    // Au matches a Sun Microsystems au file.
    let au = Mime::new(
        "audio/basic".to_string(),
        ".au".to_string(),
        PrefixDetector {
            sigs: vec![b"\x2E\x73\x6E\x64"],
        },
    );

    // Mpeg matches a Moving Picture Experts Group file.
    let mpeg = Mime::new(
        "video/mpeg".to_string(),
        ".mpeg".to_string(),
        MpegDetector {},
    );

    // QuickTime matches a QuickTime File Format file.
    let quicktime = Mime::new(
        "video/quicktime".to_string(),
        ".mov".to_string(),
        QuickTimeDetector {},
    );

    // Mqv matches a Sony / Mobile QuickTime  file.
    let mqv = Mime::new(
        "video/quicktime".to_string(),
        ".mqv".to_string(),
        FtypDetector {
            sigs: vec![b"mqt "],
        },
    );

    // Mp4 matches an MP4 file.
    let mp4 = Mime::new(
        "video/mp4".to_string(),
        ".mp4".to_string(),
        FtypDetector {
            sigs: vec![
                b"avc1", b"dash", b"iso2", b"iso3", b"iso4", b"iso5", b"iso6", b"isom", b"mmp4",
                b"mp41", b"mp42", b"mp4v", b"mp71", b"MSNV", b"NDAS", b"NDSC", b"NSDC", b"NSDH",
                b"NDSM", b"NDSP", b"NDSS", b"NDXC", b"NDXH", b"NDXM", b"NDXP", b"NDXS", b"F4V ",
                b"F4P ",
            ],
        },
    );

    // WebM matches a WebM file.
    let webm = Mime::new(
        "video/webm".to_string(),
        ".webm".to_string(),
        WebMDetector {},
    )
    .aliases(vec!["audio/webm"]);

    // ThreeGP matches a 3GPP file.
    let three_gp = Mime::new(
        "video/3gpp".to_string(),
        ".3gp".to_string(),
        FtypDetector {
            sigs: vec![
                b"3gp1", b"3gp2", b"3gp3", b"3gp4", b"3gp5", b"3gp6", b"3gp7", b"3gs7", b"3ge6",
                b"3ge7", b"3gg6",
            ],
        },
    )
    .aliases(vec!["video/3gp", "audio/3gpp"]);

    // ThreeG2 matches a 3GPP2 file.
    let three_g2 = Mime::new(
        "video/3gpp2".to_string(),
        ".3g2".to_string(),
        FtypDetector {
            sigs: vec![
                b"3g24", b"3g25", b"3g26", b"3g2a", b"3g2b", b"3g2c", b"KDDI",
            ],
        },
    )
    .aliases(vec!["video/3g2", "audio/3gpp2"]);

    // Avi matches an Audio Video Interleaved file.
    let avi = Mime::new(
        "video/x-msvideo".to_string(),
        ".avi".to_string(),
        AviDetector {},
    )
    .aliases(vec!["video/avi", "video/msvideo"]);

    // Flv matches a Flash video file.
    let flv = Mime::new(
        "video/x-flv".to_string(),
        ".flv".to_string(),
        PrefixDetector {
            sigs: vec![b"\x46\x4C\x56\x01"],
        },
    );

    // Mkv matches a mkv file.
    let mkv = Mime::new(
        "video/x-matroska".to_string(),
        ".mkv".to_string(),
        MkvDetector {},
    );

    // Asf matches an Advanced Systems Format file.
    let asf = Mime::new(
        "video/x-ms-asf".to_string(),
        ".asf".to_string(),
        PrefixDetector {
            sigs: vec![&[
                0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA, 0x00, 0x62,
                0xCE, 0x6C,
            ]],
        },
    )
    .aliases(vec!["video/asf", "video/x-ms-wmv"]);

    // AAC matches an Advanced Audio Coding file.
    let aac = Mime::new(
        "audio/aac".to_string(),
        ".aac".to_string(),
        PrefixDetector {
            sigs: vec![&[0xFF, 0xF1], &[0xFF, 0xF9]],
        },
    );

    // Voc matches a Creative Voice file.
    let voc = Mime::new(
        "audio/x-unknown".to_string(),
        ".voc".to_string(),
        PrefixDetector {
            sigs: vec![b"Creative Voice File"],
        },
    );

    // AMp4 matches an audio MP4 file.
    let amp4 = Mime::new(
        "audio/mp4".to_string(),
        ".mp4".to_string(),
        FtypDetector {
            sigs: vec![
                // audio for Adobe Flash Player 9+
                b"F4A ", b"F4B ", // Apple iTunes AAC-LC (.M4A) Audio
                b"M4B ", b"M4P ", // MPEG-4 (.MP4) for SonyPSP
                b"MSNV", // Nero Digital AAC Audio
                b"NDAS",
            ],
        },
    )
    .aliases(vec!["audio/x-m4a", "audio/x-mp4a"]);

    // M4a matches an audio M4A file.
    let m4a = Mime::new(
        "audio/x-m4a".to_string(),
        ".m4a".to_string(),
        FtypDetector {
            sigs: vec![b"M4A "],
        },
    );

    // M3u matches a Playlist file.
    let m3u = Mime::new(
        "application/vnd.apple.mpegurl".to_string(),
        ".m3u".to_string(),
        PrefixDetector {
            sigs: vec![b"#EXTM3U"],
        },
    )
    .aliases(vec!["audio/mpegurl"]);

    // M4v matches an Appl4 M4V video file.
    let m4v = Mime::new(
        "video/x-m4v".to_string(),
        ".m4v".to_string(),
        FtypDetector {
            sigs: vec![b"M4V ", b"M4VH", b"M4VP"],
        },
    );

    // Rmvb matches a RealMedia Variable Bitrate file.
    let rmvb = Mime::new(
        "application/vnd.rn-realmedia-vbr".to_string(),
        ".rmvb".to_string(),
        PrefixDetector {
            sigs: vec![&[0x2E, 0x52, 0x4D, 0x46]],
        },
    );

    // Gzip matches gzip files based on http://www.zlib.org/rfc-gzip.html#header-trailer.
    let gzip = Mime::new(
        "application/gzip".to_string(),
        ".gz".to_string(),
        PrefixDetector {
            sigs: vec![&[0x1f, 0x8b]],
        },
    )
    .aliases(vec![
        "application/x-gzip",
        "application/x-gunzip",
        "application/gzipped",
        "application/gzip-compressed",
        "application/x-gzip-compressed",
        "gzip/document",
    ]);

    // Class matches a java class file.
    let class = Mime::new(
        "application/x-java-applet".to_string(),
        ".class".to_string(),
        ClassDetector {},
    );

    // SWF matches an Adobe Flash swf file.
    let swf = Mime::new(
        "application/x-shockwave-flash".to_string(),
        ".swf".to_string(),
        PrefixDetector {
            sigs: vec![b"CWS", b"FWS", b"ZWS"],
        },
    );

    // CRX matches a Chrome extension file: a zip archive prepended by a package header.
    let crx = Mime::new(
        "application/x-chrome-extension".to_string(),
        ".crx".to_string(),
        CrxDetector {},
    );

    // Ttf matches a TrueType font file.
    let ttf = Mime::new("font/ttf".to_string(), ".ttf".to_string(), TtfDetector {}).aliases(vec![
        "font/sfnt",
        "application/x-font-ttf",
        "application/font-sfnt",
    ]);

    // Woff matches a Web Open Font Format file.
    let woff = Mime::new(
        "font/woff".to_string(),
        ".woff".to_string(),
        PrefixDetector {
            sigs: vec![b"wOFF"],
        },
    );

    // Woff2 matches a Web Open Font Format version 2 file.
    let woff2 = Mime::new(
        "font/woff2".to_string(),
        ".woff2".to_string(),
        PrefixDetector {
            sigs: vec![b"wOF2"],
        },
    );

    // Otf matches an OpenType font file.
    let otf = Mime::new(
        "font/otf".to_string(),
        ".otf".to_string(),
        PrefixDetector {
            sigs: vec![&[0x4F, 0x54, 0x54, 0x4F, 0x00]],
        },
    );

    // Ttc matches a TrueType Collection font file.
    let ttc = Mime::new(
        "font/collection".to_string(),
        ".ttc".to_string(),
        TtcDetector {},
    );

    // Eot matches an Embedded OpenType font file.
    let eot = Mime::new(
        "application/vnd.ms-fontobject".to_string(),
        ".eot".to_string(),
        EotDetector {},
    );

    // Wasm matches a web assembly File Format file.
    let wasm = Mime::new(
        "application/wasm".to_string(),
        ".wasm".to_string(),
        PrefixDetector {
            sigs: vec![&[0x00, 0x61, 0x73, 0x6D]],
        },
    );

    // Shp matches a shape format file.
    let shp = Mime::new(
        "application/vnd.shp".to_string(),
        ".shp".to_string(),
        ShpDetector {},
    );

    // Shx matches a shape index format file.
    // https://www.esri.com/library/whitepapers/pdfs/shapefile.pdf
    let shx = Mime::new(
        "application/vnd.shx".to_string(),
        ".shx".to_string(),
        PrefixDetector {
            sigs: vec![&[0x00, 0x00, 0x27, 0x0A]],
        },
    )
    .children(vec![shp]);

    // Dbf matches a dBase file.
    let dbf = Mime::new(
        "application/x-dbf".to_string(),
        ".dbf".to_string(),
        DbfDetector {},
    );

    // Dcm matches a DICOM medical format file.
    let dcm = Mime::new(
        "application/dicom".to_string(),
        ".dcm".to_string(),
        DcmDetector {},
    );

    // RAR matches a RAR archive file.
    let rar = Mime::new(
        "application/x-rar-compressed".to_string(),
        ".rar".to_string(),
        PrefixDetector {
            sigs: vec![b"Rar!\x1A\x07\x00", b"Rar!\x1A\x07\x01\x00"],
        },
    )
    .aliases(vec!["application/x-rar"]);

    // DjVu matches a DjVu file.
    let djvu = Mime::new(
        "image/vnd.djvu".to_string(),
        ".djvu".to_string(),
        DjVuDetector {},
    );

    // Mobi matches a Mobi file.
    let mobi = Mime::new(
        "application/x-mobipocket-ebook".to_string(),
        ".mobi".to_string(),
        OffsetDetector {
            offset: 60,
            sig: b"BOOKMOBI",
        },
    );

    // Lit matches a Microsoft Lit file.
    let lit = Mime::new(
        "application/x-ms-reader".to_string(),
        ".lit".to_string(),
        PrefixDetector {
            sigs: vec![b"ITOLITLS"],
        },
    );

    // Bpg matches a Better Portable Graphics file.
    let bpg = Mime::new(
        "image/bpg".to_string(),
        ".bpg".to_string(),
        PrefixDetector {
            sigs: vec![&[0x42, 0x50, 0x47, 0xFB]],
        },
    );

    // Sqlite matches an SQLite database file.
    let sqlite3 = Mime::new(
        "application/vnd.sqlite3".to_string(),
        ".sqlite".to_string(),
        PrefixDetector {
            sigs: vec![&[
                0x53, 0x51, 0x4c, 0x69, 0x74, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20,
                0x33, 0x00,
            ]],
        },
    )
    .aliases(vec!["application/x-sqlite3"]);

    // Dwg matches a CAD drawing file.
    let dwg = Mime::new(
        "image/vnd.dwg".to_string(),
        ".dwg".to_string(),
        DwgDetector {},
    )
    .aliases(vec![
        "image/x-dwg",
        "application/acad",
        "application/x-acad",
        "application/autocad_dwg",
        "application/dwg",
        "application/x-dwg",
        "application/x-autocad",
        "drawing/dwg",
    ]);

    // Nes matches a Nintendo Entertainment system ROM file.
    let nes = Mime::new(
        "application/vnd.nintendo.snes.rom".to_string(),
        ".nes".to_string(),
        PrefixDetector {
            sigs: vec![&[0x4E, 0x45, 0x53, 0x1A]],
        },
    );

    // Lnk matches Microsoft lnk binary format.
    let lnk = Mime::new(
        "application/x-ms-shortcut".to_string(),
        ".lnk".to_string(),
        PrefixDetector {
            sigs: vec![&[0x4C, 0x00, 0x00, 0x00, 0x01, 0x14, 0x02, 0x00]],
        },
    );

    // MachO matches Mach-O binaries format.
    let macho = Mime::new(
        "application/x-mach-binary".to_string(),
        ".macho".to_string(),
        MachODetector {},
    );

    // Qcp matches a Qualcomm Pure Voice file.
    let qcp = Mime::new(
        "audio/qcelp".to_string(),
        ".qcp".to_string(),
        QcpDetector {},
    );

    // Icns matches an ICNS (Apple Icon Image format) file.
    let icns = Mime::new(
        "image/x-icns".to_string(),
        ".icns".to_string(),
        PrefixDetector {
            sigs: vec![b"icns"],
        },
    );

    // Heic matches a High Efficiency Image Coding (HEIC) file.
    let heic = Mime::new(
        "image/heic".to_string(),
        ".heic".to_string(),
        FtypDetector {
            sigs: vec![b"heic", b"heix"],
        },
    );

    // HeicSequence matches a High Efficiency Image Coding (HEIC) file sequence.
    let heic_sequence = Mime::new(
        "image/heic-sequence".to_string(),
        ".heic".to_string(),
        FtypDetector {
            sigs: vec![b"hevc", b"hevx"],
        },
    );

    // Heif matches a High Efficiency Image File Format (HEIF) file.
    let heif = Mime::new(
        "image/heif".to_string(),
        ".heif".to_string(),
        FtypDetector {
            sigs: vec![b"mif1", b"heim", b"heis", b"avic"],
        },
    );

    // HeifSequence matches a High Efficiency Image File Format (HEIF) file sequence.
    let heif_sequence = Mime::new(
        "image/heif-sequence".to_string(),
        ".heif".to_string(),
        FtypDetector {
            sigs: vec![b"msf1", b"hevm", b"hevs", b"avcs"],
        },
    );

    // Hdr matches Radiance HDR image.
    // https://web.archive.org/web/20060913152809/http://local.wasp.uwa.edu.au/~pbourke/dataformats/pic/
    let hdr = Mime::new(
        "image/vnd.radiance".to_string(),
        ".hdr".to_string(),
        PrefixDetector {
            sigs: vec![b"#?RADIANCE\n"],
        },
    );

    // Marc matches a MARC21 (MAchine-Readable Cataloging) file.
    let marc = Mime::new(
        "application/marc".to_string(),
        ".mrc".to_string(),
        MarcDetector {},
    );

    // MsAccessMdb matches legacy Microsoft Access database file (JET, 2003 and earlier).
    let ms_access_mdb = Mime::new(
        "application/x-msaccess".to_string(),
        ".mdb".to_string(),
        OffsetDetector {
            sig: b"Standard Jet DB",
            offset: 4,
        },
    );

    // MsAccessAce matches Microsoft Access dababase file.
    let ms_access_ace = Mime::new(
        "application/x-msaccess".to_string(),
        ".accdb".to_string(),
        OffsetDetector {
            sig: b"Standard ACE DB",
            offset: 4,
        },
    );

    // Zstd matches a Zstandard archive file.
    let zstd = Mime::new(
        "application/zstd".to_string(),
        ".zst".to_string(),
        ZstdDetector {},
    );

    // Cab matches a Microsoft Cabinet archive file.
    let cab = Mime::new(
        "application/vnd.ms-cab-compressed".to_string(),
        ".cab".to_string(),
        PrefixDetector {
            sigs: vec![b"MSCF\x00\x00\x00\x00"],
        },
    );

    // RPM matches an RPM or Delta RPM package file.
    let rpm = Mime::new(
        "application/x-rpm".to_string(),
        ".rpm".to_string(),
        PrefixDetector {
            sigs: vec![&[0xed, 0xab, 0xee, 0xdb], b"drpm"],
        },
    );

    // Xz matches an xz compressed stream based on https://tukaani.org/xz/xz-file-format.txt.
    let xz = Mime::new(
        "application/x-xz".to_string(),
        ".xz".to_string(),
        PrefixDetector {
            sigs: vec![&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00]],
        },
    );

    // Lzip matches an Lzip compressed file.
    let lzip = Mime::new(
        "application/lzip".to_string(),
        ".lz".to_string(),
        PrefixDetector {
            sigs: vec![&[0x4c, 0x5a, 0x49, 0x50]],
        },
    )
    .aliases(vec!["application/x-lzip"]);

    // Torrent has bencoded text in the beginning.
    let torrent = Mime::new(
        "application/x-bittorrent".to_string(),
        ".torrent".to_string(),
        PrefixDetector {
            sigs: vec![b"d8:announce"],
        },
    );

    // Cpio matches a cpio archive file.
    let cpio = Mime::new(
        "application/x-cpio".to_string(),
        ".cpio".to_string(),
        PrefixDetector {
            sigs: vec![b"070707", b"070701", b"070702"],
        },
    );

    // TzIf matches a Time Zone Information Format (TZif) file.
    let tzif = Mime::new(
        "application/tzif".to_string(),
        "".to_string(),
        TzIfDetector {},
    );

    // Xcf matches GIMP image data.
    let xcf = Mime::new(
        "image/x-xcf".to_string(),
        ".xcf".to_string(),
        PrefixDetector {
            sigs: vec![b"gimp xcf"],
        },
    );

    // Pat matches GIMP pattern data.
    let pat = Mime::new(
        "image/x-gimp-pat".to_string(),
        ".pat".to_string(),
        OffsetDetector {
            offset: 20,
            sig: b"GPAT",
        },
    );

    // Gbr matches GIMP brush data.
    let gbr = Mime::new(
        "image/x-gimp-gbr".to_string(),
        ".gbr".to_string(),
        OffsetDetector {
            sig: b"GIMP",
            offset: 20,
        },
    );

    // Glb matches a glTF model format file.
    // GLB is the binary file format representation of 3D models saved in
    // the GL transmission Format (glTF).
    // GLB uses little endian and its header structure is as follows:
    //
    // 	<-- 12-byte header                             -->
    // 	| magic            | version          | length   |
    // 	| (uint32)         | (uint32)         | (uint32) |
    // 	| \x67\x6C\x54\x46 | \x01\x00\x00\x00 | ...      |
    // 	| g   l   T   F    | 1                | ...      |
    //
    // Visit [glTF specification] and [IANA glTF entry] for more details.
    //
    // [glTF specification]: https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html
    // [IANA glTF entry]: https://www.iana.org/assignments/media-types/model/gltf-binary
    let glb = Mime::new(
        "model/gltf-binary".to_string(),
        ".glb".to_string(),
        PrefixDetector {
            sigs: vec![
                b"\x67\x6C\x54\x46\x02\x00\x00\x00",
                b"\x67\x6C\x54\x46\x01\x00\x00\x00",
            ],
        },
    );

    // AVIF matches an AV1 Image File Format still or animated.
    // Wikipedia page seems outdated listing image/avif-sequence for animations.
    // https://github.com/AOMediaCodec/av1-avif/issues/59
    let avif = Mime::new(
        "image/avif".to_string(),
        ".avif".to_string(),
        FtypDetector {
            sigs: vec![b"avif", b"avis"],
        },
    );

    // InstallShieldCab matches an InstallShield Cabinet archive file.
    let installshield_cab = Mime::new(
        "application/x-installshield".to_string(),
        ".cab".to_string(),
        InstallShieldCabDetector {},
    );

    // Jxr matches Microsoft HD JXR photo file.
    let jxr = Mime::new(
        "image/jxr".to_string(),
        ".jxr".to_string(),
        PrefixDetector {
            sigs: vec![&[0x49, 0x49, 0xBC, 0x01]],
        },
    )
    .aliases(vec!["image/vnd.ms-photo"]);

    // HTML matches a Hypertext Markup Language file.
    let html = Mime::new(
        "text/html".to_string(),
        ".html".to_string(),
        MarkUpDetector {
            sigs: vec![
                b"<!DOCTYPE HTML",
                b"<HTML",
                b"<HEAD",
                b"<SCRIPT",
                b"<IFRAME",
                b"<H1",
                b"<DIV",
                b"<FONT",
                b"<TABLE",
                b"<A",
                b"<STYLE",
                b"<TITLE",
                b"<B",
                b"<BODY",
                b"<BR",
                b"<P",
            ],
        },
    );

    // Svg matches a SVG file.
    let svg = Mime::new(
        "image/svg+xml".to_string(),
        ".svg".to_string(),
        SvgDetector {},
    );

    // GeoJSON matches a RFC 7946 GeoJSON file.
    let geojson = Mime::new(
        "application/geo+json".to_string(),
        ".geojson".to_string(),
        GeoJsonDetector {},
    );

    // Rtf matches a Rich Text Format file.
    let rtf = Mime::new(
        "text/rtf".to_string(),
        ".rtf".to_string(),
        PrefixDetector {
            sigs: vec![b"{\\rtf"],
        },
    )
    .aliases(vec!["application/rtf"]);

    // XML matches an Extensible Markup Language file.
    let xml = Mime::new(
        "text/xml".to_string(),
        ".xml".to_string(),
        MarkUpDetector {
            sigs: vec![b"<?XML"],
        },
    );

    // Php matches a PHP: Hypertext Preprocessor file.
    let php = Mime::new("text/x-php".to_string(), ".php".to_string(), PhpDetector {});

    // Js matches a Javascript file.
    let js = Mime::new(
        "application/javascript".to_string(),
        ".js".to_string(),
        SheBangDetector {
            sigs: vec![
                b"/bin/node",
                b"/usr/bin/node",
                b"/bin/nodejs",
                b"/usr/bin/nodejs",
                b"/usr/bin/env node",
                b"/usr/bin/env nodejs",
            ],
        },
    )
    .aliases(vec!["application/x-javascript", "text/javascript"]);

    // Lua matches a Lua programming language file.
    let lua = Mime::new(
        "text/x-lua".to_string(),
        ".lua".to_string(),
        SheBangDetector {
            sigs: vec![b"/usr/bin/lua", b"/usr/local/bin/lua", b"/usr/bin/env lua"],
        },
    );

    // Perl matches a Perl programming language file.
    let perl = Mime::new(
        "text/x-perl".to_string(),
        ".pl".to_string(),
        SheBangDetector {
            sigs: vec![b"/usr/bin/perl", b"/usr/bin/env perl"],
        },
    );

    // Python matches a Python programming language file.
    let python = Mime::new(
        "text/x-python".to_string(),
        ".py".to_string(),
        SheBangDetector {
            sigs: vec![
                b"/usr/bin/python",
                b"/usr/local/bin/python",
                b"/usr/bin/env python",
            ],
        },
    );

    // HAR matches a HAR Spec file.
    let har = Mime::new(
        "application/json".to_string(),
        ".har".to_string(),
        HarDetector {},
    );

    // JSON matches a JavaScript Object Notation file.
    let _json = Mime::new(
        "application/json".to_string(),
        ".json".to_string(),
        JsonDetector {},
    )
    .children(vec![geojson, har]);

    // Keep text last because it is the slowest check
    let text = Mime::new(
        "text/plain".to_string(),
        ".txt".to_string(),
        TextDetector {},
    )
    .children(vec![html, svg, xml, php, js, lua, perl, python, rtf]);

    Mime::new(
        "application/octet-stream".to_string(),
        "".to_string(),
        EmptyDetector {},
    )
    .children(vec![
        xpm,
        seven_z,
        zip,
        pdf,
        fdf,
        ole,
        ps,
        psd,
        p7s,
        ogg,
        png,
        jpg,
        jxl,
        jp2,
        jpx,
        jpm,
        jxs,
        gif,
        webp,
        exe,
        elf,
        ar,
        tar,
        xar,
        bz2,
        fits,
        tiff,
        bmp,
        ico,
        mp3,
        flac,
        midi,
        ape,
        musepack,
        amr,
        wav,
        aiff,
        au,
        mpeg,
        quicktime,
        mqv,
        mp4,
        webm,
        three_gp,
        three_g2,
        avi,
        flv,
        mkv,
        asf,
        aac,
        voc,
        amp4,
        m4a,
        m3u,
        m4v,
        rmvb,
        gzip,
        class,
        swf,
        crx,
        ttf,
        woff,
        woff2,
        otf,
        ttc,
        eot,
        wasm,
        shx,
        dbf,
        dcm,
        rar,
        djvu,
        mobi,
        lit,
        bpg,
        sqlite3,
        dwg,
        nes,
        lnk,
        macho,
        qcp,
        icns,
        heic,
        heic_sequence,
        heif,
        heif_sequence,
        hdr,
        marc,
        ms_access_mdb,
        ms_access_ace,
        zstd,
        cab,
        rpm,
        xz,
        lzip,
        torrent,
        cpio,
        tzif,
        xcf,
        pat,
        gbr,
        glb,
        avif,
        installshield_cab,
        jxr,
        text,
    ])
});
