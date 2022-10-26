pub use crate::proto::{LspCodec, LspDecoder, LspEncoder};

pub mod proto;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Serde(serde_json::Error),
    Header(crate::proto::HeaderError),
}

impl From<std::io::Error> for Error {
    fn from(io: std::io::Error) -> Error {
        Error::Io(io)
    }
}

impl From<serde_json::Error> for Error {
    fn from(serde: serde_json::Error) -> Error {
        Error::Serde(serde)
    }
}

impl From<crate::proto::HeaderError> for Error {
    fn from(header: crate::proto::HeaderError) -> Error {
        Error::Header(header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio_util::codec::{FramedRead, FramedWrite};
    use tokio::runtime::Runtime;
    use tokio::stream::StreamExt;

    #[test]
    fn decode() {
        let mut runtime = Runtime::new().unwrap();

        let buf: &[u8] = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/case1.jsonrpc"
        ));

        let reader = FramedRead::new(buf, LspDecoder::default());

        let received = runtime.block_on(reader.collect::<Result<Vec<_>, _>>()).unwrap();
        assert_eq!(
            received,
            vec![
                json!({"jsonrpc": "2.0", "id": 1, "method": "textDocument/didOpen", "params": {}}),
                json!({"jsonrpc": "2.0", "id": 2, "method": "textDocument/didOpen", "params": {}}),
            ]
        );
    }

    #[test]
    fn encode() {
        use futures::sink::SinkExt;

        let mut writer = FramedWrite::new(vec![], LspEncoder);

        let obj = json!({
            "key": "value"
        });

        let mut a = Runtime::new().unwrap();
        let fut = SinkExt::send(&mut writer, obj);

        let _ = a.block_on(fut);

        let buf = writer.into_inner();
        let s = String::from_utf8(buf).unwrap();

        assert_eq!(s, "Content-Length: 15\r\n\r\n{\"key\":\"value\"}");
    }
}
