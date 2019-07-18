extern crate pretty_env_logger;

use std::path::Path;
use failure::Fallible;

#[test]
fn load_test() -> Fallible<()> {
    pretty_env_logger::init();
    let path = Path::new("/home/aeon/wd_aeon/voc/2012/VOCdevkit/VOC2012");
    voc_dataset::load(path)?;
    Ok(())
}
