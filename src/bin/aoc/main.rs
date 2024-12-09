use dotenv::dotenv;
use inquire::{required, Text};
use reqwest::{header::COOKIE, Client};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let session = std::env::var("SESSION").expect("SESSION must be set");
    let year = std::env::var("YEAR").expect("YEAR must be set");
    println!("Year: {year}");

    let mut day = match Text::new(&format!("Which day should we get the input for: "))
        .with_validator(required!())
        .prompt()
    {
        Ok(d) => d,
        Err(error) => panic!("{error}"),
    };

    let module_name = match Text::new(&format!("Module name: "))
        .with_validator(required!())
        .prompt()
    {
        Ok(name) => name,
        Err(error) => panic!("{error}"),
    };

    let struct_name = match Text::new(&format!("Struct name: "))
        .with_validator(required!())
        .prompt()
    {
        Ok(name) => name,
        Err(error) => panic!("{error}"),
    };

    // create files
    let module_path = format!("src/solutions/{module_name}.rs");
    let mut solution_file = File::create(module_path).expect("Solutions file path must be valid");
    let imports = "use crate::day::Solution;";
    let define_struct = format!("pub struct {struct_name} {{}}");
    let impl_struct =
        format!("impl {struct_name} {{ pub fn new(input: &'static str) -> Self {{ Self {{}} }} }}");
    let impl_solution = format!("impl Solution<i32, i32> for {struct_name} {{ fn part_one(&mut self) -> i32 {{ 0 }} fn part_two(&mut self) -> i32 {{ 0 }} }}");
    solution_file
        .write_all(
            format!("{imports}\n\n{define_struct}\n\n{impl_struct}\n\n{impl_solution}").as_bytes(),
        )
        .expect("Failed to write solutions file");

    // append module to solutions mod file
    let mod_path = "src/solutions/mod.rs";
    let mut mod_file = File::open(mod_path).expect("Mod file path must be valid");
    let mut mod_contents = String::new();
    mod_file
        .read_to_string(&mut mod_contents)
        .expect("Failed to read mod file");
    let mod_contents = format!("pub mod {module_name};\n{}", mod_contents);

    std::fs::write(mod_path, mod_contents).expect("Failed to write mod file");

    let input_day = if day.len() == 1 {
        format!("0{day}")
    } else {
        day.clone()
    };

    let mut main_file = File::open("src/main.rs").expect("Main file path must be valid");
    let mut main_contents = String::new();
    main_file
        .read_to_string(&mut main_contents)
        .expect("Failed to read main.rs file");
    let main_contents = format!("use solutions::{module_name}::{struct_name};\n{}", main_contents);
    let main_contents = main_contents.replace(
        &format!(" {day} => todo!()"),
        format!(" {day} => print_day({day}, {struct_name}::new(include_str!(\"input/{input_day}.txt\")))")
            .as_str(),
    );

    std::fs::write("src/main.rs", main_contents).expect("Failed to write mod file");

    let client = Client::new();

    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    let response = client
        .get(url)
        .header(COOKIE, format!("session={session}"))
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let input_path = format!("src/input/{input_day}.txt");
        let mut input_file = File::create(input_path).expect("Input file path must be valid");
        input_file
            .write_all(format!("{}\n", body.trim()).as_bytes())
            .expect("Failed to write input file");
    } else {
        eprintln!(
            "Failed with status: {}\nMessage: {}",
            response.status(),
            response.text().await?
        );
    }

    Ok(())
}
