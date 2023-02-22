use clap::{App, Arg};

fn main() {
    let matches = App::new("Temperature Converter")
        .version("1.0")
        .author("Zuhang")
        .about("temperature converter")
        .arg(Arg::with_name("temperature")
            .long("temperature")
            .value_name("TEMPERATURE")
            .required(true))
        .arg(Arg::with_name("convert")
            .long("convert")
            .value_name("UNIT")
            .required(true))
        .arg(Arg::with_name("to")
            .long("to")
            .value_name("UNIT")
            .required(true))
        .get_matches();

    let input_temp = matches.value_of("temperature").unwrap().parse::<f32>().unwrap();
    let convert_from_unit = matches.value_of("convert").unwrap().to_lowercase();
    let to_unit = matches.value_of("to").unwrap().to_lowercase();

    let output_temp = match (convert_from_unit.as_str(), to_unit.as_str()) {
        ("c", "f") => (input_temp * 9.0 / 5.0) + 32.0,
        ("f", "c") => (input_temp - 32.0) * 5.0 / 9.0,
        (_, _) => {
            return;
        }
    };

    println!("{}{} is the same as {}{}", input_temp, convert_from_unit, output_temp, to_unit);
}
