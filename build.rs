fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().build_server(false).compile(
        &[
            "proto/google/datastore/v1/datastore.proto",
            "proto/google/datastore/v1beta3/datastore.proto",
        ],
        &["proto"],
    )?;
    Ok(())
}
