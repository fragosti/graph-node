extern crate ethabi;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate graph;
extern crate hex;
extern crate nan_preserving_float;
extern crate tiny_keccak;
extern crate uuid;
extern crate wasmi;

mod asc_abi;
mod host;
mod module;
mod to_from;

use self::graph::prelude::*;
use self::graph::web3::types::{Address, Transaction};

pub use self::host::{RuntimeHost, RuntimeHostBuilder, RuntimeHostConfig};

#[derive(Clone, Debug)]
pub(crate) struct UnresolvedContractCall {
    pub contract_name: String,
    pub contract_address: Address,
    pub function_name: String,
    pub function_args: Vec<ethabi::Token>,
}

#[derive(Debug)]
pub(crate) struct EventHandlerContext {
    block: Arc<EthereumBlock>,
    transaction: Arc<Transaction>,
    entity_operations: Vec<EntityOperation>,
}
