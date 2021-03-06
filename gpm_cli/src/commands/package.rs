use gpm_core::package_writer::{create_package, CreatePackageError};
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub struct PackageParameter {
    pub input_dir: PathBuf,
    pub output_file: PathBuf,
}

#[derive(thiserror::Error, Debug)]
pub enum PackageError {
    #[error("error while creating the compressed package")]
    CreatePackageError(#[from] CreatePackageError),
    #[error("error while create the destination file {0}")]
    CreateDestinationError(PathBuf, #[source] io::Error),
    #[error("error flushing the destination file {0}")]
    FlushDestinationError(PathBuf, #[source] io::Error),
}

pub fn package(parameter: PackageParameter) -> Result<(), PackageError> {
    let mut destination_file =
        BufWriter::new(File::create(&parameter.output_file).map_err(|err| {
            PackageError::CreateDestinationError(parameter.output_file.to_path_buf(), err)
        })?);
    create_package(&parameter.input_dir, &mut destination_file)?;
    destination_file.flush().map_err(|err| {
        PackageError::FlushDestinationError(parameter.output_file.to_path_buf(), err)
    })?;
    Ok(())
}
