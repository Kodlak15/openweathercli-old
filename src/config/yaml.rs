use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Yaml {
    pub lat: Option<f32>,
    pub lon: Option<f32>,
    pub units: Option<String>,
}

pub fn get_yaml(fname: String) -> Yaml {
    let f = std::fs::File::open(&fname)
        .expect(&format!("Could not open file: {}", fname));
    let yaml: Yaml = serde_yaml::from_reader(f)
        .expect(&format!("Could not read values from file: {}", fname));
    yaml
}
