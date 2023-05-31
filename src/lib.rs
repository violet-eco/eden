use std::io::Read;

pub struct Eden;

impl Eden {
  pub fn b3sum(file: &mut std::fs::File) -> anyhow::Result<blake3::Hash> {
    let mut hasher = blake3::Hasher::new();

    let mut buf: [u8; 4096] = [0; 4096];
    loop {
      let n = file.read(&mut buf)?;

      if n == 0 {
        break;
      }

      hasher.update(&buf[..n]);
    }

    Ok(hasher.finalize())
  }
}

#[cfg(test)]
mod tests {
  use super::Eden;
  use std::str::FromStr;

  #[test]
  fn it_works() -> anyhow::Result<()> {
    let mut file = std::fs::File::open(".gitignore")?;
    let hash = Eden::b3sum(&mut file)?;
    assert_eq!(
      hash,
      blake3::Hash::from_str("91ae008eb0f251d17211c155019e6aeed915ff03ca84cc4aca55f965b9d458ec")?
    );

    Ok(())
  }
}
