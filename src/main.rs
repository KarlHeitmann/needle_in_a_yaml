use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::collections::BTreeMap;

fn main() {
    println!("Hello, world!");
    // Create a path to the desired file
    // let path = Path::new("hello.txt");
    let path = Path::new("en.yml");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let yaml = "
\"direct\": Conversations
\"phone\": Phone
\"email\": Email
\"walkin\": Walk-in
    ";
    /*
    let yaml = "
report_method:
  \"direct\": Conversations
  \"phone\": Phone
  \"email\": Email
  \"walkin\": Walk-in
  ";
    */
    /*
    let yaml = "
        - !Newtype 1
        - !Tuple [0, 0, 0]
        - !Struct {x: 1.0, y: 2.0}
    ";
    let yaml = indoc! {"
        - plain nonàscii
        - 'single quoted'
        - \"double quoted\"
    "};
    */


    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
            print!("{} contains:\n{}", display, s);
            // let deserialized_map: BTreeMap<String, f64> = serde_yaml::from_str(&yaml)?;
            // let deserialized_map: BTreeMap<String, f64> = serde_yaml::from_str(&s).unwrap();
            // let deserialized_map: BTreeMap<String, f64> = serde_yaml::from_str(&s).expect("Uh oh, problem");
            let deserialized_map: BTreeMap<String, String> = serde_yaml::from_str(&yaml).expect("Uh oh, problem");

            // let deserialized_map: BTreeMap<String, f64> = serde_yaml::from_str(&yaml).expect("Uh oh, problem");

            // let deserialized_map: _ = serde_yaml::from_str(&s).expect("Uh oh, problem");
            println!("bla bla")
        },
    }

}
