use std::future::Future;

use eyre::{eyre, Result};
use starknet::providers::ProviderError;

use crate::{rpc::error::StarknetRpcError, utils};

pub(super) fn do_call_with_mapped_rpc_err<F, T>(fut: F) -> Result<T>
where
    F: Future<Output = Result<T, ProviderError>>,
{
    match utils::block_on(fut) {
        Ok(res) => Ok(res),
        Err(ProviderError::StarknetError(e)) => Err(eyre!(StarknetRpcError::from(e))),
        Err(e) => Err(eyre!(e)),
    }
}
