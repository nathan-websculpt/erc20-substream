use substreams::{store, Hex};
use substreams::pb::substreams::eth::v2::Block;
use crate::pb::erc20;

// protoc --proto_path=proto --rs_out=src proto/erc20.proto
// rustup target add wasm32-unknown-unknown
// cargo build --target wasm32-unknown-unknown --release

#[substreams::handlers::map]
fn map_transfers(block: Block) -> Result<erc20::Transfers, substreams::errors::Error> {
    let mut transfers = erc20::Transfers::default();

    for tx in block.transactions {
        for log in tx.receipt.unwrap_or_default().logs {
            // ERC20 Transfer event topic
            if log.topics.get(0) == Some(&Hex::decode("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a3f7f8e6da").unwrap()) {
                let from = Hex::encode(&log.topics[1]);
                let to = Hex::encode(&log.topics[2]);
                let amount = u128::from_be_bytes(log.data[0..16].try_into().unwrap());

                transfers.transfers.push(erc20::Transfer {
                    from,
                    to,
                    amount: amount.to_string(),
                    token_address: Hex::encode(&log.address),
                    block_number: block.number,
                });
            }
        }
    }

    Ok(transfers)
}
