use failure::Fallible;
use minidom::Element;
use std::collections::HashMap;

/// Correspond to <pose> in annotation XML.
#[derive(Debug, Clone, Copy)]
pub enum Pose {
    Frontal,
    Rear,
    Left,
    Right,
    Unspecified,
}

/// Correspond to <bndbox> in annotation XML.
#[derive(Debug, Clone, Copy)]
pub struct BndBox {
    pub xmin: f64,
    pub ymin: f64,
    pub xmax: f64,
    pub ymax: f64,
}

/// Correspond to <object> in annotation XML.
#[derive(Debug, Clone)]
pub struct Object {
    pub name: String,
    pub pose: Pose,
    pub bndbox: BndBox,
    // TODO Implement actions
    // pub actions: Option<>,
    pub part: HashMap<String, BndBox>,
    pub truncated: Option<bool>,
    pub difficult: Option<bool>,
    pub occluded: Option<bool>,
    pub point: Option<(f64, f64)>,
}

/// Correspond to <source> in annotation XML.
#[derive(Debug, Clone)]
pub struct Source {
    pub database: String,
    pub annotation: String,
    pub image: String,
}

/// Parsed annotation XML.
#[derive(Debug, Clone)]
pub struct Annotation {
    pub folder: String,
    pub filename: String,
    pub size: (f64, f64, f64),
    pub objects: Vec<Object>,
    pub source: Source,
    pub segmented: Option<bool>,
}

/// Parse annotation XML to Annotation struct.
pub fn parse_anntation_xml(content: &str) -> Fallible<Annotation> {
    let root: Element = content
        .parse()
        .map_err(|err| format_err!("Failed to parse annotation XML: {:?}", err))?;
    ensure!(
        root.name() == "annotation",
        "Expect <annotation> root element"
    );

    let mut folder = None;
    let mut filename = None;
    let mut size = None;
    let mut objects = vec![];
    let mut source = None;
    let mut segmented = None;

    for annotation_child in root.children() {
        match annotation_child.name() {
            "folder" => match folder {
                Some(_) => bail!("<folder> is duplicated in <annotation>"),
                None => folder = Some(annotation_child.text()),
            },
            "filename" => match filename {
                Some(_) => bail!("<filename> is duplicated in <annotation>"),
                None => filename = Some(annotation_child.text()),
            },
            "size" => match size {
                Some(_) => bail!("<size> is duplicated in <annotation>"),
                None => size = Some(parse_size_elem(annotation_child)?),
            },
            "object" => {
                let object = parse_object_elem(annotation_child)?;
                objects.push(object);
            }
            "segmented" => {
                if let Some(_) = segmented {
                    bail!("<segmented> is duplicated in <annotation>");
                }
                let val = match annotation_child.text().as_str() {
                    "0" => false,
                    "1" => true,
                    _ => bail!(
                        "expect 0 or 1 in <segmented>, but get {}",
                        annotation_child.text()
                    ),
                };
                segmented = Some(val);
            }
            "source" => match source {
                Some(_) => bail!("<source> is duplicated in <annotation>"),
                None => source = Some(parse_source_elem(annotation_child)?),
            },
            _ => bail!("Unexpected <{}> in <annotation>", annotation_child.name()),
        }
    }

    let annotation = Annotation {
        folder: folder.unwrap(),
        filename: filename.unwrap(),
        size: size.unwrap(),
        objects,
        source: source.unwrap(),
        segmented,
    };

    Ok(annotation)
}

fn parse_size_elem(size_elem: &Element) -> Fallible<(f64, f64, f64)> {
    let mut width = None;
    let mut height = None;
    let mut depth = None;

    for size_child in size_elem.children() {
        match size_child.name() {
            "width" => match width {
                Some(_) => bail!("<width> is duplicated in <size>"),
                None => width = Some(size_child.text().parse()?),
            },
            "height" => match height {
                Some(_) => bail!("<height> is duplicated in <size>"),
                None => height = Some(size_child.text().parse()?),
            },
            "depth" => match depth {
                Some(_) => bail!("<depth> is duplicated in <size>"),
                None => depth = Some(size_child.text().parse()?),
            },
            _ => bail!(
                "Unexpected <{:?}> element found in <size>",
                size_child.name()
            ),
        }
    }

    if let None = width {
        bail!("<width> is missing in <size>");
    }

    if let None = height {
        bail!("<height> is missing in <size>");
    }

    if let None = depth {
        bail!("<depth> is missing in <size>");
    }

    Ok((width.unwrap(), height.unwrap(), depth.unwrap()))
}

fn parse_object_elem(object_elem: &Element) -> Fallible<Object> {
    let mut name = None;
    // let mut actions = None;
    let mut bndbox = None;
    let mut truncated = None;
    let mut difficult = None;
    let mut occluded = None;
    let mut pose = None;
    let mut point = None;
    let mut part = HashMap::new();

    for object_child in object_elem.children() {
        match object_child.name() {
            "name" => match name {
                Some(_) => bail!("<name> is duplicated in <object>"),
                None => name = Some(object_child.text()),
            },
            "actions" => {
                // if let Some(_) = actions {
                //     bail!("<action> is duplicated in <object>");
                // }

                // TODO impl
                warn!("<actions> is not implemented");
            }
            "bndbox" => match bndbox {
                Some(_) => bail!("<bndbox> is duplicated in <object>"),
                None => bndbox = Some(parse_bndbox_elem(object_child)?),
            },
            "difficult" => {
                if let Some(_) = difficult {
                    bail!("<difficult> is duplicated in <object>");
                }

                let val = match object_child.text().as_str() {
                    "0" => false,
                    "1" => true,
                    _ => bail!(
                        "expect 0 or 1 in <difficult>, but get {}",
                        object_child.text()
                    ),
                };

                difficult = Some(val);
            }
            "truncated" => {
                if let Some(_) = truncated {
                    bail!("<truncated> is duplicated in <object>");
                }

                let val = match object_child.text().as_str() {
                    "0" => false,
                    "1" => true,
                    _ => bail!(
                        "expect 0 or 1 in <truncated>, but get {}",
                        object_child.text()
                    ),
                };

                truncated = Some(val);
            }
            "occluded" => {
                if let Some(_) = occluded {
                    bail!("<occluded> is duplicated in <object>");
                }

                let val = match object_child.text().as_str() {
                    "0" => false,
                    "1" => true,
                    _ => bail!(
                        "expect 0 or 1 in <occluded>, but get {}",
                        object_child.text()
                    ),
                };

                occluded = Some(val);
            }
            "pose" => {
                if let Some(_) = pose {
                    bail!("<pose> is duplicated in <object>");
                }

                let val = match object_child.text().as_str() {
                    "Frontal" => Pose::Frontal,
                    "Rear" => Pose::Rear,
                    "Left" => Pose::Left,
                    "Right" => Pose::Right,
                    "Unspecified" => Pose::Unspecified,
                    _ => bail!("Expect \"Frontal\", \"Rear\", \"Left\", \"Right\", \"Unspecified\" in <pose>, but get \"{}\"", object_child.text()),
                };
                pose = Some(val);
            }
            "point" => {
                if let Some(_) = point {
                    bail!("<point> is duplicated in <object>");
                }

                let mut x = None;
                let mut y = None;

                for point_child in object_child.children() {
                    match point_child.name() {
                        "x" => {
                            if let Some(_) = x {
                                bail!("<x> is duplicated in <point>");
                            }
                            x = Some(point_child.text().parse()?);
                        }
                        "y" => {
                            if let Some(_) = y {
                                bail!("<y> is duplicated in <point>");
                            }
                            y = Some(point_child.text().parse()?);
                        }
                        _ => bail!("Unexpected <{}> in <point>", point_child.name()),
                    }
                }

                if let None = x {
                    bail!("<x> is missing in <point>");
                }

                if let None = y {
                    bail!("<y> is missing in <point>");
                }

                point = Some((x.unwrap(), y.unwrap()));
            }
            "part" => {
                let mut name = None;
                let mut bndbox = None;

                for part_child in object_child.children() {
                    match part_child.name() {
                        "name" => match name {
                            Some(_) => bail!("<name> is duplicated in <part>"),
                            None => name = Some(part_child.text()),
                        },
                        "bndbox" => match bndbox {
                            Some(_) => bail!("<bndbox> is duplicated in <part>"),
                            None => bndbox = Some(parse_bndbox_elem(part_child)?),
                        },
                        _ => bail!("Unexpected <{}> in <part>", part_child.name()),
                    }
                }

                if let None = name {
                    bail!("<name> is missing in <part>");
                }

                if let None = bndbox {
                    bail!("<bndbox> is missing in <part>");
                }

                part.insert(name.unwrap(), bndbox.unwrap());
            }
            _ => bail!("Unexpected <{}> in <object>", object_child.name()),
        }
    }

    if let None = name {
        bail!("<name> is missing in <object>");
    }

    if let None = pose {
        bail!("<pose> is missing in <object>");
    }

    if let None = bndbox {
        bail!("<bndbox> is missing in <object>");
    }

    let object = Object {
        name: name.unwrap(),
        pose: pose.unwrap(),
        bndbox: bndbox.unwrap(),
        part,
        truncated,
        difficult,
        occluded,
        point,
    };

    Ok(object)
}

fn parse_source_elem(source_elem: &Element) -> Fallible<Source> {
    let mut database = None;
    let mut annotation = None;
    let mut image = None;

    for source_child in source_elem.children() {
        match source_child.name() {
            "database" => match database {
                Some(_) => bail!("duplicated <database> in <source>"),
                None => database = Some(source_child.text()),
            },
            "annotation" => match annotation {
                Some(_) => bail!("duplicated <annotation> in <source>"),
                None => annotation = Some(source_child.text()),
            },
            "image" => match image {
                Some(_) => bail!("duplicated <image> in <source>"),
                None => image = Some(source_child.text()),
            },
            _ => bail!("Unexpected <{}> in <source>", source_child.name()),
        }
    }

    if let None = database {
        bail!("<database> is missing in <source>");
    }

    if let None = annotation {
        bail!("<annotation> is missing in <source>");
    }

    if let None = image {
        bail!("<image> is missing in <source>");
    }

    let source = Source {
        database: database.unwrap(),
        annotation: annotation.unwrap(),
        image: image.unwrap(),
    };
    Ok(source)
}

fn parse_bndbox_elem(object_elem: &Element) -> Fallible<BndBox> {
    let mut xmin = None;
    let mut ymin = None;
    let mut xmax = None;
    let mut ymax = None;

    for bndbox_child in object_elem.children() {
        let val = bndbox_child.text().parse()?;
        match bndbox_child.name() {
            "xmin" => xmin = Some(val),
            "xmax" => xmax = Some(val),
            "ymin" => ymin = Some(val),
            "ymax" => ymax = Some(val),
            _ => bail!("unexpected <{}> in <bndbox>", bndbox_child.name()), // TODO [arse error
        }
    }

    if let None = xmin {
        bail!("<xmin> is missing in <bndbox>");
    }

    if let None = xmax {
        bail!("<xmax> is missing in <bndbox>");
    }

    if let None = ymin {
        bail!("<ymin> is missing in <bndbox>");
    }

    if let None = ymax {
        bail!("<ymax> is missing in <bndbox>");
    }

    let bndbox = BndBox {
        xmin: xmin.unwrap(),
        ymin: ymin.unwrap(),
        xmax: xmax.unwrap(),
        ymax: ymax.unwrap(),
    };

    Ok(bndbox)
}
