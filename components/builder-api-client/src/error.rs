// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{error,
          fmt,
          io,
          num,
          path::PathBuf,
          result};

use hyper;
use serde_json;
use url;

use crate::{bio_core,
            bio_http};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    APIError(hyper::status::StatusCode, String),
    BadResponseBody(io::Error),
    DownloadWrite(PathBuf, io::Error),
    BiomeCore(bio_core::Error),
    BiomeHttpClient(bio_http::Error),
    HyperError(hyper::error::Error),
    IO(io::Error),
    Json(serde_json::Error),
    KeyReadError(PathBuf, io::Error),
    NoFilePart,
    PackageReadError(PathBuf, io::Error),
    ParseIntError(num::ParseIntError),
    IdentNotFullyQualified,
    UploadFailed(String),
    UrlParseError(url::ParseError),
    WriteSyncFailed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            Error::APIError(ref c, ref m) if !m.is_empty() => format!("[{}] {}", c, m),
            Error::APIError(ref c, _) => format!("[{}]", c),
            Error::BadResponseBody(ref e) => format!("Failed to read response body, {}", e),
            Error::DownloadWrite(ref p, ref e) => {
                format!("Failed to write contents of builder response, {}, {}",
                        p.display(),
                        e)
            }
            Error::BiomeCore(ref e) => format!("{}", e),
            Error::BiomeHttpClient(ref e) => format!("{}", e),
            Error::HyperError(ref err) => format!("{}", err),
            Error::IO(ref e) => format!("{}", e),
            Error::Json(ref e) => format!("{}", e),
            Error::KeyReadError(ref p, ref e) => {
                format!("Failed to read origin key, {}, {}", p.display(), e)
            }
            Error::NoFilePart => "An invalid path was passed - we needed a filename, and this \
                                  path does not have one"
                                                         .to_string(),
            Error::PackageReadError(ref p, ref e) => {
                format!("Failed to read package artifact, {}, {}", p.display(), e)
            }
            Error::ParseIntError(ref err) => format!("{}", err),
            Error::IdentNotFullyQualified => {
                "Cannot perform the specified operation. Specify a fully qualifed package \
                 identifier (ex: core/busybox-static/1.42.2/20170513215502)"
                                                                            .to_string()
            }
            Error::UploadFailed(ref s) => format!("Upload failed: {}", s),
            Error::UrlParseError(ref e) => format!("{}", e),
            Error::WriteSyncFailed => {
                "Could not write to destination; perhaps the disk is full?".to_string()
            }
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::APIError(..) => "Received a non-2XX response code from API",
            Error::BadResponseBody(_) => "Failed to read response body",
            Error::DownloadWrite(..) => "Failed to write response contents to file",
            Error::BiomeCore(ref err) => err.description(),
            Error::BiomeHttpClient(ref err) => err.description(),
            Error::HyperError(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::KeyReadError(..) => "Failed to read origin key from disk",
            Error::NoFilePart => {
                "An invalid path was passed - we needed a filename, and this path does not have one"
            }
            Error::PackageReadError(..) => "Failed to read package artifact from disk",
            Error::ParseIntError(ref err) => err.description(),
            Error::IdentNotFullyQualified => {
                "Cannot perform the specified operation. Specify a fully qualifed package \
                 identifier (ex: core/busybox-static/1.42.2/20170513215502)"
            }
            Error::UploadFailed(_) => "Upload failed",
            Error::UrlParseError(ref err) => err.description(),
            Error::WriteSyncFailed => {
                "Could not write to destination; bytes written was 0 on a non-0 buffer"
            }
        }
    }
}

impl From<bio_core::Error> for Error {
    fn from(err: bio_core::Error) -> Error { Error::BiomeCore(err) }
}

impl From<bio_http::Error> for Error {
    fn from(err: bio_http::Error) -> Error { Error::BiomeHttpClient(err) }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Error { Error::HyperError(err) }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { Error::IO(err) }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error { Error::Json(err) }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error { Error::UrlParseError(err) }
}
