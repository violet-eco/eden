use std::io::Read;
pub struct Eden;

impl Eden {
    pub fn b3sum_with_size(
        file: &mut std::fs::File,
        buf_size: usize,
    ) -> anyhow::Result<blake3::Hash> {
        let mut hasher = blake3::Hasher::new();

        let mut buf: Vec<u8> = Vec::new();
        buf.resize(buf_size, 0);
        loop {
            let n = file.read(&mut buf)?;

            if n == 0 {
                break;
            }

            hasher.update(&buf[..n]);
        }

        Ok(hasher.finalize())
    }

    pub fn b3sum(file: &mut std::fs::File) -> anyhow::Result<blake3::Hash> {
        Self::b3sum_with_size(file, 4096)
    }
}

#[cfg(test)]
mod tests;
