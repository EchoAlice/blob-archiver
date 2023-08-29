use beacon_api_client::{mainnet::Client, BlockId};
use ethereum_consensus::deneb::BlobSidecar;
use url::Url;

// How many bytes per blob?
const BYTES_PER_BLOB: usize = 512;

// TODO: Print blobs to terminal
fn process_sidecars(sidecars: Vec<BlobSidecar<BYTES_PER_BLOB>>) {
    for car in sidecars {
        println!("{:?}", car.kzg_commitment);
    }

    // TODO: Verify blob against commitment
    // TODO (future): Write sidecars to database
}

#[tokio::main]
async fn main() {
    // Should I be using mainnet client or minimal client?
    let url = Url::parse("http://localhost::502").unwrap();
    let client = Client::new(url);

    // How do i get the block_id and indices?
    let id = BlockId::Slot(100);
    let indices = [0; 32];

    let sidecars = client.get_blob_sidecars(id, &indices).await.unwrap();

    process_sidecars(sidecars);
}
