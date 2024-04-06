use eyre::Result;
use rika_args::{
    commands::block::{AgeArgs, BlockArgs, BlockNumberArgs},
    fmt::pretty_block_without_txs,
};
use starknet::{
    core::types::{BlockId, MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs},
    providers::{Provider, ProviderError},
};

use crate::utils;

pub fn age(args: AgeArgs) -> Result<()> {
    let AgeArgs {
        block_id,
        starknet,
        human_readable,
    } = args;

    let provider = starknet.provider();
    let block = utils::block_on(get_block(provider, block_id))?;

    let timestamp = match block {
        MaybePendingBlockWithTxs::Block(b) => b.timestamp,
        MaybePendingBlockWithTxs::PendingBlock(b) => b.timestamp,
    };

    if human_readable {
        use chrono::{Local, TimeZone};
        let formatted = Local.timestamp_opt(timestamp as i64, 0).unwrap();
        println!("{formatted}");
    } else {
        println!("{timestamp}")
    }

    Ok(())
}

pub fn get(args: BlockArgs) -> Result<()> {
    let BlockArgs {
        id,
        starknet,
        full,
        compact,
        display,
    } = args;

    let provider = starknet.provider();

    if compact {
        let block = utils::block_on(get_block_compact(provider, id))?;
        display.display(block)?;
        return Ok(());
    } else {
        let block = utils::block_on(get_block(provider, id))?;
        if full || display.field.is_some() {
            display.display(block)?;
        } else {
            println!("{}", pretty_block_without_txs(&block));
        }
    }

    Ok(())
}

pub fn number(args: BlockNumberArgs) -> Result<()> {
    let provider = args.starknet.provider();
    let number = utils::block_on(provider.block_number())?;
    println!("{number:#x}");
    Ok(())
}

async fn get_block<P>(provider: P, id: BlockId) -> Result<MaybePendingBlockWithTxs, ProviderError>
where
    P: Provider,
{
    provider.get_block_with_txs(id).await
}

async fn get_block_compact<P>(
    provider: P,
    id: BlockId,
) -> Result<MaybePendingBlockWithTxHashes, ProviderError>
where
    P: Provider,
{
    provider.get_block_with_tx_hashes(id).await
}
