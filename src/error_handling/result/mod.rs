use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::path::PathBuf;
use std::result;

// Result hass 2 varients Ok and Err

pub fn output() {
    let path = "../../z_test_result/lines.txt";
    let res = File::create(path);
    let mut result = match res {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file in First File Create : {:?}", error),
    };
    write!(result, "Just Some\nRandom Words").expect("Failed to write file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let res2 = File::create("../../z_test_result/rand.txt");
    let mut result2 = match res2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("../../z_test_result/rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", error),
            },
            _other_error => panic!("Problem opening file : {:}", error),
        },
    };
    let mut rng = rand::thread_rng();
    let n: u8 = rng.gen();
    write!(result2, "{n}").expect("Something wrong write number in file");
}
