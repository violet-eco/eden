use std::io::Read;

pub struct Eden;

impl Eden {
    pub fn b3sum(file: &mut std::fs::File) -> Result<blake3::Hash, Box<dyn std::error::Error>> {
        let mut hasher = blake3::Hasher::new();
    
        let mut buf = [0; 4096];
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
    fn it_works() {
        let mut file = std::fs::File::open("Cargo.toml").expect("Err");
        let hash = Eden::b3sum(&mut file).expect("Err");
        assert_eq!(hash, blake3::Hash::from_str("d5300d9f6cb008b5695f9f36f1f3e1f50d0dc95f75df4fd57b50a8df53fb01eb").expect("Err"));
    }
}