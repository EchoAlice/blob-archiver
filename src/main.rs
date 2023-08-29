use crate::secret::url::make_url;
use beacon_api_client::{mainnet::Client, BlockId};
use ethereum_consensus::deneb::BlobSidecar;
mod secret;
// How many bytes per blob? 32 x 4096
const BYTES_PER_BLOB: usize = 131072;

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

    let id = BlockId::Head;
    let indices = [];
    let sidecars = client.get_blob_sidecars(id, &indices).await.unwrap();
    process_sidecars(sidecars);
}
