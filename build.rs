fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pb_config = prost_build::Config::new();
    pb_config.btree_map(&["."]);
    pb_config.compile_protos(&["proto/message.proto"], &["proto/"])?;
    Ok(())
}
