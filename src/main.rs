use crate::url::make_url;
use beacon_api_client::{mainnet::Client, BlockId};
use ethereum_consensus::{clock::from_system_time, deneb::BlobSidecar};
use tokio_stream::StreamExt;

mod url;

const BYTES_PER_BLOB: usize = 131072; // 32 x 4096

// Why are blobs only printing occasionally?
fn process_sidecars(sidecars: Vec<BlobSidecar<BYTES_PER_BLOB>>) {
    for car in sidecars {
        println!("{:?}", car.kzg_commitment);
    }

    // TODO: Verify blob against commitment
    // TODO (future): Write sidecars to database
}

#[tokio::main]
async fn main() {
    let url = make_url();
    let client = Client::new(url);

    let genesis_details = client.get_genesis_details().await.unwrap();
    let genesis_time = genesis_details.genesis_time;
    let seconds_per_slot = 12;
    let slots_per_epoch = 32;

    // Use Ethereum clock to query for blobs
    let clock = from_system_time(genesis_time, seconds_per_slot, slots_per_epoch);
    let slot_stream = clock.stream_slots();
    tokio::pin!(slot_stream);

    while let Some(slot) = slot_stream.next().await {
        println!("Slot: {:?}", slot);
        let id = BlockId::Head;
        let indices = [];
        let sidecars = client.get_blob_sidecars(id, &indices).await.unwrap();
        process_sidecars(sidecars);
        println!("\n");
    }
}
