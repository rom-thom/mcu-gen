use std::fs;

use svd_parser::{parse, svd::Device};

pub fn parse_svd(path: &str)->Device{
    let svd_xml = fs::read_to_string(path).expect("couldn't read svd file when trying to parse it");
    parse(&svd_xml).expect("Couldn't parse the svd file")
}





#[test]
fn test_parsing(){
    let device = parse_svd("../../metadata/stm32f103c8t6/source.svd");
    // Print device name
    println!("Parsed device: {}", device.name);

    let index = 35;
    dbg!(&device.peripherals[index].name);
    dbg!(&device.peripherals[index].description);
}