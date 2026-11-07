use std::path::{Path, PathBuf};

use minicbor::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::ErrorCode;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Encode, Decode)]
#[serde(transparent)]
#[cbor(transparent)]
pub struct ServerPath(
    #[n(0)]
    #[cbor(with = "wire_path")]
    pub Vec<u8>,
);

/// Stable Unix clients require absolute paths to start with `/`. Keep native
/// paths in memory and on disk, and use the picker's drive/share namespace
/// only at the CBOR boundary. Replies from those clients reverse the mapping.
mod wire_path {
    use minicbor::encode::{Error, Write};
    use minicbor::{Decoder, Encoder};

    pub(super) fn encode<C, W: Write>(
        value: &[u8],
        encoder: &mut Encoder<W>,
        _: &mut C,
    ) -> Result<(), Error<W::Error>> {
        #[cfg(windows)]
        if let Ok(path) = std::str::from_utf8(value) {
            if let Some(share) = path.strip_prefix(r"\\") {
                encoder.bytes(format!("/UNC/{}", share.replace('\\', "/")).as_bytes())?;
                return Ok(());
            }
            if matches!(value, [drive, b':', b'\\' | b'/', ..] if drive.is_ascii_alphabetic()) {
                encoder.bytes(format!("/{}", path.replace('\\', "/")).as_bytes())?;
                return Ok(());
            }
        }
        encoder.bytes(value)?;
        Ok(())
    }

    pub(super) fn decode<C>(
        decoder: &mut Decoder<'_>,
        _: &mut C,
    ) -> Result<Vec<u8>, minicbor::decode::Error> {
        let value = decoder.bytes()?;
        #[cfg(windows)]
        if let Ok(path) = std::str::from_utf8(value) {
            if let Some(share) = path.strip_prefix("/UNC/") {
                return Ok(format!(r"\\{}", share.replace('/', r"\")).into_bytes());
            }
            if matches!(value, [b'/', drive, b':'] if drive.is_ascii_alphabetic()) {
                return Ok(format!(r"{}\", &path[1..]).into_bytes());
            }
            if matches!(value, [b'/', drive, b':', b'/', ..] if drive.is_ascii_alphabetic()) {
                return Ok(path[1..].replace('/', r"\").into_bytes());
            }
        }
        Ok(value.to_vec())
    }
}

impl ServerPath {
    /// The path as it travels: its raw bytes on Unix, and UTF-8 on Windows,
    /// where a path that isn't valid Unicode is refused rather than
    /// converted lossily.
    pub fn from_path(path: &Path) -> Result<Self, ErrorCode> {
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            Ok(Self(path.as_os_str().as_bytes().to_vec()))
        }
        #[cfg(windows)]
        {
            path.to_str()
                .map(|text| Self(text.as_bytes().to_vec()))
                .ok_or(ErrorCode::BadPath)
        }
    }

    /// The path on this computer. On Windows, bytes that aren't UTF-8 are
    /// refused.
    pub fn to_path_buf(&self) -> Result<PathBuf, ErrorCode> {
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            Ok(Path::new(std::ffi::OsStr::from_bytes(&self.0)).to_path_buf())
        }
        #[cfg(windows)]
        {
            std::str::from_utf8(&self.0)
                .map(PathBuf::from)
                .map_err(|_| ErrorCode::BadPath)
        }
    }

    /// Whether the path is absolute on a Unix or a Windows server, whichever
    /// this build is: `/…`, `X:\…`, `X:/…`, or `\\server\share…`. A server
    /// also checks that it is absolute on its own system.
    pub fn is_absolute(&self) -> bool {
        match self.0.as_slice() {
            [b'/', ..] => true,
            [drive, b':', b'\\' | b'/', ..] => drive.is_ascii_alphabetic(),
            [b'\\', b'\\', unc @ ..] => {
                let mut parts = unc.split(|byte| *byte == b'\\');
                matches!(
                    (parts.next(), parts.next()),
                    (Some(server), Some(share)) if !server.is_empty() && !share.is_empty()
                )
            }
            _ => false,
        }
    }
}
