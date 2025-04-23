use std::time::Instant;
use clap::{Arg, ArgAction, Command};
use clipboard_win::{formats, set_clipboard};
use rand::Rng;

fn main() {

let start = Instant::now();

let mut password = String::new();

let alphabet_uppercase = "abcdefghijklmnopqrstuvwxyz";
let alphabet_lowercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
let figures = "0123456789";

let mut others  = "";

let mut tmp = String::new();

for i in 33..=47
{tmp.push(i as u8 as char);}

for i in 58..=64
{tmp.push(i as u8 as char);}

for i in 91..=96
{tmp.push(i as u8 as char);}

for i in 123..=126
{tmp.push(i as u8 as char);}

let binding = &(others.to_owned() + &tmp);
others = &binding;

let mut categories : Vec<String> = Vec::new();

    let matches = Command::new("pwdgen")
        .version("0.1.0")
        .author("Rubino Marc <rubino.marc@gmx.com>")
        .about("Rust version of `password_generator`")
        .arg(
            Arg::new("chrs")
                .value_name("number")
                .help("Input the number of required caracters")
                .required(true),
        )
         .arg(
            Arg::new("enable_majs")
                .short('a')
                .action(ArgAction::SetTrue)
                .help("Use A-Z"),
        )
       .arg(
            Arg::new("enable_mins")
                .short('A')
                .action(ArgAction::SetTrue)
                .help("Use a-z"),
        )
         .arg(
            Arg::new("enable_numbers")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Use 0-9"),
        )
         .arg(
            Arg::new("enable_others")
                .short('o')
                .action(ArgAction::SetTrue)
                .help("Use special caracters"),
        )

        .get_matches();

let majs = matches.get_flag("enable_majs");
//println!("MAJ: {}", majs);
if majs { categories.push(String::from("upp")); }

let mins = matches.get_flag("enable_mins");
//println!("MIN: {}", mins);
if mins { categories.push(String::from("low")); }

let nums = matches.get_flag("enable_numbers");
//println!("NUM: {}", nums);
if nums { categories.push(String::from("fig")); }

let oths = matches.get_flag("enable_others");
//println!("OTH: {}", oths);
if oths { categories.push(String::from("oth")); }


if categories.len() < 1 {eprintln!("\n_/!\\_ SORRY: you asked ... NOTHING _/!\\_");
std::process::abort();}

    let number_str = matches.get_one::<String>("chrs").expect("Argument missing");
    let number: i32 = number_str.parse().expect("Please provide a valid integer");


 for _i in 0..number {
    let mut rng = rand::rng();
    let random_index1 = rng.random_range(0..categories.len());
 
    let choosen_cat : &str = &categories[random_index1];

let cat_size  = match choosen_cat 
    {
        "upp" => 26,
        "low" => 26,
        "fig" => 10,
        "oth" => others.len(),     /////////////////////////////////////////////////
        &_ => 33,
    };

    let random_index2 = rng.random_range(0..cat_size);

   let random_char = match choosen_cat
   {

       "upp" => alphabet_uppercase.chars().nth(random_index2).unwrap(),
       "low" => alphabet_lowercase.chars().nth(random_index2).unwrap(),
       "fig" => figures.chars().nth(random_index2).unwrap(),
       "oth" => others.chars().nth(random_index2).unwrap(),
       &_  => ' ' ,
   };

     password = password.to_owned() + &String::from(random_char);
 }


    let elapsed = start.elapsed();

println!("\n{}", password);
set_clipboard(formats::Unicode, password).expect("To set clipboard");
    println!("\n( Elapsed time: {:.2?} )", elapsed);

}
