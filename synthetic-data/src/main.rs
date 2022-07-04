mod feature_collection;

fn main() {
    println!("Data Faking!");

    let u = feature_collection::deserialize_json_from_file_path("/home/tom/proejcts/poc/england-motorways.geojson").unwrap();
    println!("{:?}", u);

}
