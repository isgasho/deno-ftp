use super::dispatch_json::{Deserialize, JsonOp, Value};
use crate::op_error::OpError;
use crate::state::State;
use deno_core::CoreIsolate;
use deno_core::ZeroCopyBuf;
use std::collections::HashMap;
use std::env;
use std::io::{Error, ErrorKind};
use url::Url;

// All ops are registered here
pub fn init(i: &mut CoreIsolate, s: &State) {
    i.register_op("connect", s.stateful_json_op(op_connect));
}

#[derive(Deserialize, Debug)]
struct ConnectArgs {
    addr: SocketAddr,
}

fn op_connect(
    state: &State,
    args: Value,
    _zero_copy: &mut [ZeroCopyBuf],
) -> Result<JsonOp, OpError> {
    let args: ConnectArgs = serde_json::from_value(args)?;
    dbg!(args);
    let v = FtpStream::connect(args.addr);
    Ok(JsonOp::Sync(r))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
