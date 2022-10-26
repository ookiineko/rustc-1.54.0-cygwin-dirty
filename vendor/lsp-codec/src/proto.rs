use std::fmt::Write;
use std::io;

use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use crate::Error;

type Body = serde_json::Value;

#[derive(Default)]
pub struct LspCodec {
    encoder: LspEncoder,
    decoder: LspDecoder,
}

impl Encoder<Body> for LspCodec {
    type Error = <LspEncoder as Encoder<Body>>::Error;

    fn encode(&mut self, item: Body, dst: &mut BytesMut) -> Result<(), Self::Error> {
        Encoder::encode(&mut self.encoder, item, dst)
    }
}

impl Decoder for LspCodec {
    type Item = Body;
    type Error = <LspEncoder as Encoder<Body>>::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Decoder::decode(&mut self.decoder, buf)
    }
}

#[derive(Default)]
pub struct LspDecoder {
    state: State,
}

#[derive(Default)]
pub struct LspEncoder;

enum State {
    ReadingHeader {
        header: HeaderBuilder,
        cursor: usize,
    },
    ReadingBody(Header),
    Parsed(Body),
}

impl Default for State {
    fn default() -> State {
        State::ReadingHeader {
            header: HeaderBuilder::default(),
            cursor: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HeaderError {
    DuplicateHeaderField,
    MissingContentLength,
    UnsupportedCharset,
    HeaderFieldParseError(String),
    WrongEntryField(String),
}

#[derive(Debug, Default, PartialEq)]
pub struct Header {
    content_length: ContentLength,
    content_type: Option<ContentType>,
}

#[derive(Default)]
struct HeaderBuilder {
    content_length: Option<ContentLength>,
    content_type: Option<ContentType>,
}

impl HeaderBuilder {
    fn try_field(&mut self, field: HeaderField) -> Result<&mut Self, HeaderError> {
        match field {
            HeaderField::ContentLength(len) => {
                if self.content_length.is_some() {
                    Err(HeaderError::DuplicateHeaderField)
                } else {
                    self.content_length = Some(len);
                    Ok(self)
                }
            }
            HeaderField::ContentType(typ) => {
                if self.content_type.is_some() {
                    Err(HeaderError::DuplicateHeaderField)
                } else {
                    self.content_type = Some(typ);
                    Ok(self)
                }
            }
        }
    }

    fn try_build(self) -> Result<Header, HeaderError> {
        if let Some(len) = self.content_length {
            Ok(Header {
                content_length: len,
                content_type: self.content_type,
            })
        } else {
            Err(HeaderError::MissingContentLength)
        }
    }
}

#[derive(Debug, Default, PartialEq)]
struct ContentLength(usize);
#[derive(Debug, PartialEq)]
struct ContentType(String);

impl Default for ContentType {
    fn default() -> ContentType {
        ContentType(String::from("application/vscode-jsonrpc; charset=utf-8"))
    }
}

enum HeaderField {
    ContentLength(ContentLength),
    ContentType(ContentType),
}

impl std::str::FromStr for HeaderField {
    type Err = HeaderError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ContentLength::from_str(s)
            .map(HeaderField::ContentLength)
            .or_else(|_| ContentType::from_str(s).map(HeaderField::ContentType))
    }
}

impl std::str::FromStr for ContentLength {
    type Err = HeaderError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("Content-Length: ") {
            let len = s["Content-Length: ".len()..]
                .trim_end()
                .parse()
                .map_err(|_| HeaderError::HeaderFieldParseError(s.to_owned()))?;
            Ok(ContentLength(len))
        } else {
            Err(HeaderError::HeaderFieldParseError(s.to_owned()))
        }
    }
}

impl std::str::FromStr for ContentType {
    type Err = HeaderError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("Content-Type: ") {
            let typ = &s["Content-Type: ".len()..];

            match typ.find("charset=").map(|i| &typ[i + "charset=".len()..]) {
                Some(charset)
                    if charset.starts_with("utf8")
                        || charset.starts_with("utf-8")
                        || charset.starts_with("UTF-8") => {}
                // https://github.com/Microsoft/language-server-protocol/issues/600
                _ => Err(HeaderError::UnsupportedCharset)?,
            }

            Ok(ContentType(typ.to_owned()))
        } else {
            Err(HeaderError::HeaderFieldParseError(s.to_owned()))
        }
    }
}

enum UpdateState {
    NotEnough,
    Ready,
    Parsed,
}

impl State {
    fn try_update(&mut self, buf: &mut BytesMut) -> Result<UpdateState, Error> {
        match self {
            State::ReadingHeader { header, cursor } => {
                if let Some(index) = buf[*cursor..].windows(2).position(|w| w == [b'\r', b'\n']) {
                    let index = *cursor + index;

                    let line = buf.split_to(index + 2); // consume \r *and* trailing \n
                    *cursor = 0;

                    let line = &line[..line.len() - 2];
                    let line = std::str::from_utf8(&line).expect("invalid utf8 data");

                    if line.is_empty() {
                        let header = std::mem::replace(header, HeaderBuilder::default())
                            .try_build()
                            .map_err(|_| HeaderError::MissingContentLength)?;
                        *self = State::ReadingBody(header);
                    } else {
                        let field = line
                            .parse()
                            .map_err(|_| HeaderError::WrongEntryField(line.to_owned()))?;
                        header
                            .try_field(field)
                            .map_err(|_| HeaderError::DuplicateHeaderField)?;
                    }

                    Ok(UpdateState::Ready)
                } else {
                    *cursor = buf.len();

                    Ok(UpdateState::NotEnough)
                }
            }
            State::ReadingBody(header) => {
                if buf.len() >= header.content_length.0 {
                    let buf = buf.split_to(header.content_length.0);

                    let s = std::str::from_utf8(&buf).expect("invalid utf8 data");
                    let body = serde_json::from_str(s).map_err(Error::Serde)?;

                    *self = State::Parsed(body);

                    Ok(UpdateState::Parsed)
                } else {
                    Ok(UpdateState::NotEnough)
                }
            }
            State::Parsed(..) => Ok(UpdateState::Parsed),
        }
    }
}

impl Decoder for LspDecoder {
    type Item = Body;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        loop {
            match self.state.try_update(buf)? {
                UpdateState::Ready => continue,
                UpdateState::NotEnough => break Ok(None),
                UpdateState::Parsed => {
                    break match std::mem::replace(&mut self.state, State::default()) {
                        State::Parsed(body) => Ok(Some(body)),
                        _ => unreachable!(),
                    };
                }
            };
        }
    }
}

impl Encoder<Body> for LspEncoder {
    type Error = Error;

    fn encode(&mut self, item: Body, dst: &mut BytesMut) -> Result<(), Error> {
        let body = serde_json::to_string(&item).map_err(Error::Serde)?;
        let body_len: usize = body.chars().map(char::len_utf8).sum();

        let header = format!("Content-Length: {}\r\n\r\n", body_len);
        let header_len: usize = header.chars().map(char::len_utf8).sum();

        dst.reserve(header_len + body_len);
        Ok(write!(dst, "{}{}", header, body)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Formatting into buffer failed"))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_type() {
        // Backwards compatibility, see https://github.com/Microsoft/language-server-protocol/pull/199.
        let ContentType(typ) = "Content-Type: application/vscode-jsonrpc; charset=utf8"
            .parse()
            .unwrap();
        assert_eq!(typ, "application/vscode-jsonrpc; charset=utf8");

        let ContentType(typ) = "Content-Type: application/vscode-jsonrpc; charset=utf-8"
            .parse()
            .unwrap();
        assert_eq!(typ, "application/vscode-jsonrpc; charset=utf-8");

        let ContentType(typ) = "Content-Type: application/vscode-jsonrpc; charset=UTF-8"
            .parse()
            .unwrap();
        assert_eq!(typ, "application/vscode-jsonrpc; charset=UTF-8");

        let res = "Content-Type: application/vscode-jsonrpc; charset=utf-16".parse::<ContentType>();
        assert!(res.is_err());

        let res = "Content-Type: application/vscode-jsonrpc; charset=latin1".parse::<ContentType>();
        assert!(res.is_err());
    }
}
