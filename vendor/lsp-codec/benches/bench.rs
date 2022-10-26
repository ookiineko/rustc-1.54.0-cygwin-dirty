#![feature(test)]

extern crate test;

const DATA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/case2.jsonrpc"
));

use lsp_codec::LspDecoder;

use tokio::runtime::current_thread::Runtime;
use tokio_codec::FramedRead;

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream::Stream;
    use test::{black_box, Bencher};

    #[bench]
    fn codec(b: &mut Bencher) {
        let mut rt = Runtime::new().unwrap();

        b.iter(|| {
            let read = FramedRead::new(DATA, LspDecoder::default());
            black_box(rt.block_on(read.collect()).unwrap());
        });
    }
}
