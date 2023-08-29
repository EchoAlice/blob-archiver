use beacon_api_client::Client;

fn process_sidecars(sidecars: Sidecar) {
    // Write sidecars to database
    // or...

    for car in sidecars {
        println!("{:?}", car.commitment);
    }
}

fn main() {
    let cllient = Client::new("http://localhost::502");

    // how do i get the id?
    let id = [0; 32];
    // how do i get the indices?
    let indices = [0; 32];
    let sidecars = client.get_blob_sidecars(id, indices).await;

    process_sidecars(sidecars);
}
