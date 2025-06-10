use crate::abi::{self};
use crate::pb::erc20::types::v1::{TransferEvents, TransferEvent};
use abi::erc20::events::Transfer;
use substreams::errors::Error;
use substreams::Hex;
use substreams_ethereum::block_view::LogView;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

// protoc --proto_path=proto --rs_out=src proto/v1/erc20.proto
// rustup target add wasm32-unknown-unknown
// cargo build --target wasm32-unknown-unknown --release

// substreams build 
// Generates the necessary Protobufs specified in the substreams.yaml file.
// Compiles the Rust code.
// Creates a Substreams package file (.spkg).

// cargo clean	
// Removes build artifacts (target directory), then you will probably have to run: cargo build --target wasm32-unknown-unknown --release



// substreams protogen ./substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google"

#[substreams::handlers::map]
pub fn map_transfers(block: Block) -> Result<TransferEvents, Error> {
    let  transfers = map_events(&block);

    Ok(TransferEvents {
        transfers
    })
}

pub fn map_events(block: &Block) -> Vec<TransferEvent> {
    let mut transfers = vec![];

    for log in block.logs() {
        
        if let Some(transfer) = Transfer::match_and_decode(log.log) {
            transfers.push(decode_transfer(transfer, log));
            continue;
        }

        // no data
    }

    return transfers;
}

fn decode_transfer(event: Transfer, log: LogView) -> TransferEvent {
    TransferEvent {
        // contract address
        contract: Hex::encode(log.address()),

        // event payload
        from: Hex::encode(event.from),
        to: Hex::encode(event.to),
        value: event.value.to_string(),

        // trace information
        tx_id: Hex::encode(&log.receipt.transaction.hash),
        block_index: log.log.block_index.into(),
        index: log.index()
    }
}
