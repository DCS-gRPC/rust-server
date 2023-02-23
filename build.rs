use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::Hasher;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

fn main() {
    println!("cargo:rerun-if-changed=lua/DCS-gRPC");
    println!("cargo:rerun-if-changed=lua/Hooks");

    let path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("lua_files.rs");
    let mut out = File::create(path).unwrap();

    for (ident, base_path) in [("DCS_GRPC", "./lua/DCS-gRPC"), ("HOOKS", "./lua/Hooks")] {
        writeln!(out, "const {ident}: &[(&str, u64)] = &[").unwrap();

        for entry in WalkDir::new(base_path) {
            let entry = entry.unwrap();
            if !entry.metadata().unwrap().is_file() {
                continue;
            }

            let path = entry
                .path()
                .strip_prefix(base_path)
                .unwrap()
                .to_str()
                .expect("non-utf8 path");
            let hash = file_hash(entry.path());
            writeln!(out, r##"    (r#"{path}"#, {hash}),"##).unwrap();
            eprintln!("{}", entry.path().display());
        }

        writeln!(out, "];").unwrap();
    }
}

fn file_hash(path: &Path) -> u64 {
    // Not a cryptographic hasher, but good enough for our use-case.
    let mut hasher = DefaultHasher::new();
    let mut buffer = [0; 1024];
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    loop {
        let count = reader.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        hasher.write(&buffer[..count]);
    }

    hasher.finish()
}
