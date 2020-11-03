# voc-dataset

[docs](https://docs.rs/voc-dataset/) | [crates.io](https://crates.io/crates/voc-dataset)

The crate provides types and loader for the **PASCAL Visual Object Classes (VOC)** dataset. It features serde-compatible types.

Add this line to use the crate in your project.

```toml
voc-dataset = "0.2"
```

## Usage

The `load()` function loads all available samples from VOC dataset directory.

```rust
let voc_dir = test_data_dir.join("VOCdevkit").join("VOC2012");
let samples = voc_dataset::load(&voc_dir)?;

for sample in samples.iter() {
    // --snip--
}
```

The annotation types are serde-compatible. You can parse the annotation files with [serde\_xml\_rs](https://crates.io/crates/serde_xml_rs).

```rust
use voc_dataset::Annotation;

let xml = std::fs::read_to_string("VOCdevkit/VOC2012/Annotations/2012_001231.xml")?;
let annotation: Annotation = serde_xml_rs::from_str(&xml)?;
```

### License

MIT. See [license file](LICENSE.txt).
