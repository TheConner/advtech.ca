use std::path::Path;
use std::{io, fs};

use rand::distributions::{Alphanumeric, DistString};

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
  fs::create_dir_all(&dst)?;
  for entry in fs::read_dir(src)? {
      let entry = entry?;
      let ty = entry.file_type()?;
      if ty.is_dir() {
          copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
      } else {
          fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
      }
  }
  Ok(())
}

pub fn gen_cache_buster() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 8)
}