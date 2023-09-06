use crate::kzg_crypto::verify_blob_kzg_proof;
use crate::url::make_url;
use beacon_api_client::{mainnet::Client, BlockId};
use ethereum_consensus::{clock::from_system_time, deneb::mainnet::BlobSidecar};
use tokio_stream::StreamExt;
mod kzg_crypto;
mod url;

// Why are blobs only printing occasionally?
fn process_sidecars(sidecars: Vec<BlobSidecar>) {
    for car in sidecars {
        // TODO: Verify blob against commitment
        verify_blob_kzg_proof(car.blob, car.kzg_commitment, car.kzg_proof);

        // TODO (future): Write sidecars to database
    }
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
