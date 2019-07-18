extern crate glob;
extern crate minidom;
#[macro_use] extern crate log;
#[macro_use] extern crate failure;

mod parse;
pub mod error;

use std::path::{Path, PathBuf};
use failure::{Error, Fallible};

pub use crate::error::ParseAnnotationError;
pub use crate::parse::{
    parse_anntation_xml,
    BndBox,
    Object,
    Source,
    Annotation,
};

pub struct Sample {
    pub image_path: PathBuf,
    pub annotation: Annotation,
}

pub fn load<P: AsRef<Path>>(dataset_dir: P) -> Fallible<Vec<Sample>> {
    let dataset_dir_r = dataset_dir.as_ref();
    let image_dir = dataset_dir_r.join("JPEGImages");
    let annotations_dir = dataset_dir_r.join("Annotations");

    let samples = image_dir
        .read_dir()?
        .map(|entry_result| -> Fallible<Option<_>> {
            let entry = entry_result?;
            if entry.file_type()?.is_file() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "jpg" {
                        let name = path.file_stem().unwrap();
                        return Ok(Some((name.to_owned(), path.to_owned())));
                    }
                }
            }
            Ok(None)
        })
        .filter_map(|arg| {
            match arg {
                Ok(None) => None,
                Ok(Some(pair)) => Some(Ok(pair)),
                Err(err) => Some(Err(err)),
            }
        })
        .map(|arg| {
            let (image_name, image_path) = arg?;
            let mut xml_path = annotations_dir.join(&image_name);
            ensure!(xml_path.set_extension("xml"), "set_extension() failed");
            info!("Loading {}", xml_path.display());

            // File annotation xml
            let content = std::fs::read_to_string(&xml_path)?;
            let annotation = match parse::parse_anntation_xml(&content) {
                Ok(result) => result,
                Err(err) => return Err(Error::from(ParseAnnotationError::new(&xml_path, err))),
            };

            // Verify if filename matches
            let mut file_name = image_name.clone();
            file_name.push(".jpg");
            ensure!(file_name == annotation.filename.as_str(), "Expect \"{}\" in <filename>, but get \"{}\"", file_name.to_str().unwrap(), annotation.filename);

            let sample = Sample {
                image_path,
                annotation,
            };

            Ok(sample)
        })
        .collect::<Result<Vec<Sample>, _>>()?;

    Ok(samples)
}
