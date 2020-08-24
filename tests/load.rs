use anyhow::{bail, ensure, Result};
use crypto::{digest::Digest, sha2::Sha256};
use log::info;
use std::{
    fs::File,
    io::{prelude::*, BufReader, SeekFrom},
    path::PathBuf,
};
use tar::Archive;

#[test]
fn load_voc_2012() -> Result<()> {
    pretty_env_logger::init();

    // Prepare test data directory
    let cargo_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_data_dir = cargo_dir.join("test_data");
    std::fs::create_dir_all(&test_data_dir)?;

    // Prepare VOC 2012 dataset
    let file_path = test_data_dir.join("VOCtrainval_11-May-2012.tar");

    if file_path.exists() {
        if !file_path.is_file() {
            bail!("{:?} is not a file", file_path);
        }
    } else {
        // Download file
        info!("Downloading VOC 2012 dataset...");

        let mut file = File::create(&file_path)?;
        let mut resp = reqwest::blocking::get(
            "http://host.robots.ox.ac.uk/pascal/VOC/voc2012/VOCtrainval_11-May-2012.tar",
        )?;
        std::io::copy(&mut resp, &mut file)?;
    }

    // Verify digest
    info!("Verify file checksum");

    let real_digest = {
        let mut reader = BufReader::new(File::open(&file_path)?);
        let mut hasher = Sha256::new();
        let blk_size = hasher.block_size();
        reader.seek(SeekFrom::Start(0))?;

        let mut buf = vec![0; blk_size];
        while reader.read(&mut buf)? > 0 {
            hasher.input(&buf);
        }
        hasher.result_str()
    };

    let expect_digest = "e14f763270cf193d0b5f74b169f44157a4b0c6efa708f4dd0ff78ee691763bcb";
    ensure!(
        &expect_digest == &real_digest,
        "File checksum mismatch. Expect {}, but get {}. Please remove {:?} and try again.",
        &expect_digest,
        &real_digest,
        &file_path,
    );
    info!("Checksum matched!");

    // Untar file
    {
        info!("Unpack dataset");

        let file = File::open(&file_path)?;
        let voc_dir = test_data_dir.join("VOCdevkit");
        if voc_dir.exists() {
            std::fs::remove_dir_all(&voc_dir)?;
        }

        let mut tarball = Archive::new(&file);
        tarball.unpack(&test_data_dir)?;
    };

    let voc_dir = test_data_dir.join("VOCdevkit").join("VOC2012");
    let samples = voc_dataset::load(&voc_dir)?;

    let n_samples = samples.len();
    ensure!(
        17125 == n_samples,
        "Number of samples mismatches. Expect 17125 samples, but get {}",
        n_samples
    );

    Ok(())
}
