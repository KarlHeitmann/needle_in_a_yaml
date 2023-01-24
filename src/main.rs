use serde_yaml; // 0.8.7
use serde_yaml::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let f = std::fs::File::open("en.yml")?;
    let f = std::fs::File::open("sample.yml")?;
    // let d: String = serde_yaml::from_reader(f)?;
    let d: Value = serde_yaml::from_reader(f)?;
    // println!("Read YAML string: {}", d);

    println!("Read YAML string: {:?}", d);
    // let d = d.unwrap();
    Ok(())
}
