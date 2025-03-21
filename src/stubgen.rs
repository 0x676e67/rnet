fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "docs")]
    {
        let stub = rnet::stub_info()?;
        stub.generate()?;
    }
    Ok(())
}
