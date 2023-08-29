use beacon_api_client::{mainnet::Client, BlockId};
use ethereum_consensus::deneb::BlobSidecar;
use url::Url;

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
    let username = "eth";
    let password = "prAP7jyLJoHzbCa";
    let devnet_name = "dencun-devnet-8";
    let rpc_or_bn = "bn"; // Use "rpc" for the EL

    let url_str = format!(
        "https://{}:{}@{}.{}-{}-1.srv.{}.ethpandaops.io",
        username, password, rpc_or_bn, "teku", "geth", devnet_name
    );

    let url = Url::parse(&url_str).unwrap();
    let client = Client::new(url);

    // How do i get the block_id and indices?
    let id = BlockId::Head;
    let indices = [];

    let sidecars = client.get_blob_sidecars(id, &indices).await.unwrap();
    process_sidecars(sidecars);
}
