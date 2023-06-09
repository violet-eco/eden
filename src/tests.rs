use super::Eden;
use std::str::FromStr;

#[test]
fn it_works() -> anyhow::Result<()> {
    let mut file = std::fs::File::open("manti.test")?;
    let hash = Eden::b3sum(&mut file)?;
    assert_eq!(
        hash,
        blake3::Hash::from_str("af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262")?
    );

    Ok(())
}
