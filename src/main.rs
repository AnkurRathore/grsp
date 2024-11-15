use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App, Arg};
fn main() {

    let args = App::new("grsp")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .help("File to search")
            .takes_value(true)
            .required(true))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let input  = args.value_of("input").unwrap();
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);
    let ctx_lines = 2;
    let re = Regex::new(pattern).unwrap();
    

//tags holds line numbers where there is a match
let mut tags: Vec<usize> = vec![];
//this will contain a vector per match to hold context lines
let mut ctx: Vec<Vec<(usize, String)>> = vec![];
for (i ,line_result) in reader.lines().enumerate(){
    match line_result{
        Ok(line) => {
            if re.is_match(&line){
                tags.push(i);
    
                let v = Vec::with_capacity(2*ctx_lines + 1);
                ctx.push(v);
                
            }
        }
        Err(e)=>{
            eprintln!("Error reading line {}: {}", i+1, e);
            return;
        }
       
    }

    }        

    //if no match, exit early
    if tags.is_empty(){
        return;
    }

    // Reset the reader to iterate through the lines again
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    for (i, line_result) in reader.lines().enumerate(){
        match line_result{
            Ok(line) => {
                for (j, tag) in tags.iter().enumerate(){
                    let lower_bound = tag.saturating_sub(ctx_lines);
                    let upper_bound = tag + ctx_lines;
        
                    if (i >= lower_bound) && (i<= upper_bound){
                        let local_ctx = (i, line.clone());
                        ctx[j].push(local_ctx);
                    }
                }

            }
            Err(e) => {
                eprintln!("Error reading from line {}: {}", i +1, e);
                return;
            }
        }
       
    }

    for local_ctx in ctx.iter(){
        for &(i, ref line) in local_ctx.iter(){
            let line_num = i + 1;
            println!("{}: {}", line_num, line)
        }
    }
}
