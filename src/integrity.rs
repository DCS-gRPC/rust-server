use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::Hasher;
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use std::{error, fmt};

use crate::config::Config;

include!(concat!(env!("OUT_DIR"), "/lua_files.rs"));

/// Check the integrity (compare the file hashes) of all DCS-gRPC related Lua files.
pub fn check(config: &Config) -> Result<(), IntegrityError> {
    let dcs_grpc_base_path = AsRef::<Path>::as_ref(&config.lua_path);
    for (path, expected_hash) in DCS_GRPC {
        let path = dcs_grpc_base_path.join(path);
        log::debug!("checking integrity of `{}`", path.display());
        let file = File::open(&path).map_err(|err| IntegrityError::Read(path.clone(), err))?;
        let hash = file_hash(&file).map_err(|err| IntegrityError::Hash(path.clone(), err))?;
        if hash != *expected_hash {
            return Err(IntegrityError::HashMismatch(path));
        }
    }

    let hooks_base_path = AsRef::<Path>::as_ref(&config.write_dir).join("Scripts/Hooks");
    for (path, expected_hash) in HOOKS {
        let path = hooks_base_path.join(path);
        log::debug!("checking integrity of `{}`", path.display());
        let file = File::open(&path).map_err(|err| IntegrityError::Read(path.clone(), err))?;
        let hash = file_hash(&file).map_err(|err| IntegrityError::Hash(path.clone(), err))?;
        if hash != *expected_hash {
            return Err(IntegrityError::HashMismatch(path));
        }
    }

    Ok(())
}

fn file_hash(file: &File) -> io::Result<u64> {
    // Not a cryptographic hasher, but good enough for our use-case.
    let mut hasher = DefaultHasher::new();
    let mut buffer = [0; 1024];
    let mut reader = BufReader::new(file);

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.write(&buffer[..count]);
    }

    Ok(hasher.finish())
}

#[derive(Debug)]
pub enum IntegrityError {
    Read(PathBuf, io::Error),
    Hash(PathBuf, io::Error),
    HashMismatch(PathBuf),
}

impl error::Error for IntegrityError {}

impl fmt::Display for IntegrityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "integrity check failed ")?;
        match self {
            IntegrityError::Read(path, err) => {
                write!(f, "(could not read `{}`: {err})", path.display())
            }
            IntegrityError::Hash(path, err) => {
                write!(f, "(could not create hash for `{}`: {err})", path.display())
            }
            IntegrityError::HashMismatch(path) => {
                write!(f, "(hash mismatch of `{}`)", path.display())
            }
        }?;
        write!(
            f,
            ", DCS-gRPC is not started, please check your installation"
        )?;
        Ok(())
    }
}
