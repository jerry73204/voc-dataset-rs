//! Simple loader for the PASCAL Visual Object Classes (VOC)
//!
//! This crate supports dataset formats from VOC2007 to VOC2012.

mod common;
mod parse;

pub use crate::parse::{parse_anntation_xml, Annotation, BndBox, Object, Source};

use crate::common::*;

/// The sample corresponds to an image along with annotations
#[derive(Debug, Clone)]
pub struct Sample {
    pub image_path: PathBuf,
    pub annotation: Annotation,
}

/// Load VOC data directory
pub fn load<P: AsRef<Path>>(dataset_dir: P) -> Result<Vec<Sample>> {
    let dataset_dir_r = dataset_dir.as_ref();
    let image_dir = dataset_dir_r.join("JPEGImages");
    let annotations_dir = dataset_dir_r.join("Annotations");

    let samples = image_dir
        .read_dir()?
        .map(|entry_result| -> Result<Option<_>> {
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
        .filter_map(|arg| match arg {
            Ok(None) => None,
            Ok(Some(pair)) => Some(Ok(pair)),
            Err(err) => Some(Err(err)),
        })
        .map(|arg| {
            let (image_name, image_path) = arg?;
            let mut xml_path = annotations_dir.join(&image_name);
            ensure!(xml_path.set_extension("xml"), "set_extension() failed");
            info!("Loading {}", xml_path.display());

            // File annotation xml
            let content = std::fs::read_to_string(&xml_path)
                .with_context(|| format!("cannot open file {}", xml_path.display()))?;
            let annotation = parse::parse_anntation_xml(&content)
                .with_context(|| format!("failed to parse file {}", xml_path.display()))?;

            // Verify if filename matches
            let mut file_name = image_name.clone();
            file_name.push(".jpg");
            ensure!(
                file_name == annotation.filename.as_str(),
                "Expect \"{}\" in <filename>, but get \"{}\"",
                file_name.to_str().unwrap(),
                annotation.filename,
            );

            let sample = Sample {
                image_path,
                annotation,
            };

            Ok(sample)
        })
        .collect::<Result<Vec<Sample>, _>>()?;

    Ok(samples)
}
