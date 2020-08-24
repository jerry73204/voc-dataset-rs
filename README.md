# Simple data loader for The PASCAL Visual Object Classes (VOC)

This crate defines annotation XML types for VOC dataset.

You can import this crate from crates.io.

```toml
voc-dataset = "0.2"
```

## Usage

This crate provides a `load()` function to load entire VOC data directory.

```rust
extern crate voc_dataset;

let voc_dir = test_data_dir.join("VOCdevkit").join("VOC2012");
let samples: Vec<_> = voc_dataset::load(&voc_dir)?;

for sample in samples.iter() {
    // --snip--
}
```

If you would like to parse a single annotation XML:

```rust
let xml = "...";  // annotation XML data
let annotation = parse_anntation_xml(xml)?;
```

Please see [docs](https://docs.rs/voc-dataset/) for more details.

### License

MIT
