use jsonrpc_core::Result;
use jsonrpc_derive::rpc;



#[rpc]
pub trait ExampleRpc {
    #[rpc(name = "example_value")]
    fn get_value(&self) -> Result<u32>;
}

pub struct Example;

impl ExampleRpc for Example {

    fn get_value(&self) -> Result<u32> {
        Ok(5)
    }
}

