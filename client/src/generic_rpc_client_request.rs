use crate::{client_error::Result, rpc_request::RpcRequest};

pub trait GenericRpcClientRequest {
    fn send(
        &self,
        request: RpcRequest,
        params: serde_json::Value,
        retries: usize,
    ) -> Result<serde_json::Value>;
}
