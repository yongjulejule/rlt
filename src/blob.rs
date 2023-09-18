use std::fs::{self, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::{fs::OpenOptions, io};

use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};

pub struct Blob {
    content: Vec<u8>,
    hash: String,
}

impl Blob {
    fn hash_object(content: &[u8]) -> String {
        let mut hasher = Sha1::new();
        hasher.update(content);
        return hasher
            .finalize()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();
    }

    pub fn new(content: Vec<u8>) -> Self {
        let hash = Blob::hash_object(&content);
        return Self { content, hash };
    }

    pub fn run() -> Result<(), i32> {
        let mut content = String::new();
        println!("Enter blob content: ");
        io::stdin()
            .read_line(&mut content)
            .expect("failed to read from stdin");

        let header = format!("blob {}\0{}", content.len(), content);

        let blob = Blob::new(header.into_bytes());
        let path: PathBuf = blob.path();

        println!("path: {:?}", path);

        create_dir_all(path.parent().expect("failed to get parent path"))
            .expect("failed to create directory");

        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .expect("failed to create file");

        let mut compressed = ZlibEncoder::new(Vec::new(), Compression::default());

        compressed
            .write_all(blob.content())
            .expect("failed to write to compressed file");

        let compressed_file = compressed
            .finish()
            .expect("failed to finish compressed file");

        file.write_all(&compressed_file)
            .expect("failed to write to file");

        return Ok(());
    }

    fn hash(&self) -> String {
        return self.hash.clone();
    }

    fn content(&self) -> &[u8] {
        return &self.content;
    }

    fn path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(".git");
        path.push("objects");
        path.push(&self.hash[0..2]);
        path.push(&self.hash[2..]);
        return path;
    }
}
