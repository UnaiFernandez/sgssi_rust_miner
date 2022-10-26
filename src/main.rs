#![allow(non_snake_case)]

//Import utils
use sha256::digest;
//use sha256::{digest, try_digest};     // sha256 crypto util
use std::path::Path;        // for files
use clap::Parser;           // command line arguments
use rand::Rng;
use std::fmt::Write;
use std::fs::File;
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
struct Cli {
    // Selected file
    file: String,
    // Number of zeros
    zeros: u32, 
}

/*
* This Function counts the number of zeros at the begingin of the digest
*/
fn count_zeros (hash: String) -> u32{
    let mut count: u32 = 0;
    let hash_chars = hash.chars();
    if hash.chars().nth(0).unwrap() == '0' {
        for i in hash_chars {
            if i == '0'{
                count += 1;
            }else{
                break;
            }
        }   
    }
    return count;
}

fn random_hex() -> String {
    //Generate a random number
    let mut rng = rand::thread_rng();
    let num :u32 = rng.gen();
    let mut hex_num = String::new();
    
    //Create the hex_numature to add at the end of the file
    write!(&mut hex_num, "{:X}", num).unwrap();
    hex_num.push_str(" UF21\n");
    return hex_num;
}

/*
 * Main function
 */
fn main() {

    let args = Cli::parse();
    let input_file = Path::new(&args.file);
    
    let sty = ProgressStyle::with_template(
        "[*] Number of zeros [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
    ).unwrap().progress_chars("##-");
    let pb = ProgressBar::new(args.zeros.into());
    pb.set_style(sty.clone());
        
    let start = Instant::now();
    println!("[*] Mining the block: {}", args.file);
    let mut file_content = std::fs::read_to_string(&input_file).expect("could not read the file");
    let original_content = file_content.clone(); 
    let mut iter: u32 = 0;
    let mut find = 0;

    let mut new_file = String::from("new_");
    let file = args.file;
    new_file.push_str(&file);

    let mut dig = String::new();
    let mut count = 0;
    let mut max = 0;
    while find == 0{
        // Add iteration
        iter += 1;

        let hex = random_hex();

        //Add hex_numature to the end of the file.
        file_content.push_str(&hex);

        //Digest for the file content
        dig = digest(file_content.clone());

        //Count the number of zeros
        count = count_zeros(dig.clone());

        // Update the progress bar
        if count > max {
                let inc = count - max;
                max = count;
                pb.inc(inc.into());
        }

        if count >= args.zeros{
            find = 1;
        }else{
            file_content = original_content.clone();    
        }
    }

    //pb.finish_with_message("done");
    pb.finish_with_message("Done!");

    
    let mut output = File::create(&new_file).unwrap();
    use std::io::Write;
    write!(output, "{}", file_content).unwrap();


    let end = start.elapsed();
    println!("[OK] Block mined successfuly!");
    println!("  -Digest:               {}", dig);
    println!("  -Number of zeros:      {}", count);
    println!("  -Number of iterations: {}", iter);
    println!("  -Time:                 {:?}", end);
    println!("  -New file name:        {}", new_file);


}
