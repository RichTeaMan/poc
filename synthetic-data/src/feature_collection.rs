use std::{io::BufReader, fs::File, path::Path, error::Error};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureProperty {
    #[serde(rename = "@id")]
    pub id :String,
    pub highway: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinate {
    pub longitude: f64,
    pub latitude: f64
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub json_type: String,
    pub coordinates: Vec<Coordinate>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    pub id :String,
    #[serde(rename = "type")]
    pub json_type: String,
    pub properties: FeatureProperty,
    pub geometry: Geometry
}


#[derive(Serialize, Deserialize, Debug)]
pub struct FeatureCollection {
    #[serde(rename = "type")]
    pub json_type: String,
    pub generator: String,
    pub copyright: String,
    pub timestamp: String,
    pub features: Vec<Feature>
}

pub fn deserialize_json(json: String) -> Result<FeatureCollection, serde_json::Error> {
    serde_json::from_str::<FeatureCollection>(&json)
}

pub fn deserialize_json_from_file_path<P: AsRef<Path>>(file_path: P) -> Result<FeatureCollection, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let fc = serde_json::from_reader(reader)?;
    Ok(fc)
}

#[test]
pub fn small_test()
{
    let json = r#"
    {
        "type": "FeatureCollection",
        "generator": "overpass-ide",
        "copyright": "The data included in this document is from www.openstreetmap.org. The data is made available under ODbL.",
        "timestamp": "2022-06-30T15:06:01Z",
        "features": [
        {
            "type": "Feature",
            "properties": {
            "@id": "way/1166",
            "bicycle": "no",
            "highway": "motorway",
            "int_ref": "E 05",
            "lanes": "3",
            "lit": "no",
            "maxheight": "default",
            "maxspeed": "70 mph",
            "maxspeed:type": "GB:motorway",
            "oneway": "yes",
            "operator": "Midland Expressway Ltd",
            "ref": "M6 Toll",
            "toll": "yes"
            },
            "geometry": {
            "type": "LineString",
            "coordinates": [
                [
                -1.7494674,
                52.5582538
                ],
                [
                -1.7476485,
                52.5570226
                ],
                [
                -1.7458686,
                52.5559384
                ],
                [
                -1.7439692,
                52.5549363
                ],
                [
                -1.7422686,
                52.554129
                ],
                [
                -1.7390098,
                52.5526292
                ],
                [
                -1.7389154,
                52.5525873
                ]
            ]
            },
            "id": "way/1166"
        }
        ]
    }
    "#;

    let result = deserialize_json((&json).to_string()).unwrap();
    println!("{:?}", result);

    assert_eq!(7, result.features[0].geometry.coordinates.len());
}