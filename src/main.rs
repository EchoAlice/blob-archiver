use crate::secret::url::make_url;
use beacon_api_client::{mainnet::Client, BlockId};
use ethereum_consensus::deneb::BlobSidecar;
<<<<<<< HEAD
mod secret;
// How many bytes per blob? 32 x 4096
const BYTES_PER_BLOB: usize = 131072;
=======
use url::Url;

// How many bytes per blob?
const BYTES_PER_BLOB: usize = 512;
>>>>>>> parent of 9e7212b (Get blobs from Devnet 8 and print to terminal)

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
<<<<<<< HEAD
    let url = make_url();
    let client = Client::new(url);

    let id = BlockId::Head;
    let indices = [];
=======
    // Should I be using mainnet client or minimal client?
    let url = Url::parse("http://localhost::502").unwrap();
    let client = Client::new(url);

    // How do i get the block_id and indices?
    let id = BlockId::Slot(100);
    let indices = [0; 32];

>>>>>>> parent of 9e7212b (Get blobs from Devnet 8 and print to terminal)
    let sidecars = client.get_blob_sidecars(id, &indices).await.unwrap();

    process_sidecars(sidecars);
}
