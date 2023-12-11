use regex::Regex;
use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() -> io::Result<()>  {


    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);



    
    let index = Regex::new(r"Game ([0-9]*)").unwrap();
    let red_re = Regex::new(r"([0-9]*) red").unwrap();
    let blue_re = Regex::new(r"([0-9]*) blue").unwrap();
    let green_re = Regex::new(r"([0-9]*) green").unwrap();

    let total_green:u32 = 13;
    let total_red:u32 = 12;
    let total_blue:u32 = 14;

    let mut res:u32 =0;

    '_game:for line in reader.lines(){
        let game_line:&str = &line.unwrap();
        let vec:Vec<&str> = game_line.split(";").collect();
        let game:u32 = index.captures(game_line).unwrap()[1].parse().unwrap();

        let mut blue:Vec<u32> = Vec::new();
        let mut green:Vec<u32> = Vec::new();
        let mut red:Vec<u32> = Vec::new();
        for set in &vec {
       
        let blue_caps = blue_re.captures_iter(set);
        let green_caps = green_re.captures_iter(set);
        let red_caps = red_re.captures_iter(set);

        for cap in blue_caps {
           let blue_num:u32 =  cap[1].parse().unwrap();
           blue.push(blue_num);
        }

        for cap in green_caps {
            let green_num:u32 =  cap[1].parse().unwrap();
            green.push(green_num);
         }
        
        for cap in red_caps {
            let red_num:u32 =  cap[1].parse().unwrap();
            red.push(red_num);
         }



        }
    
    let power = blue.iter().max().unwrap() * green.iter().max().unwrap() * red.iter().max().unwrap();
    
    res += power

}

    println!("{}",res);
    return Ok(())

}

// to recive the game id or just use a count 
// is to split the using the ;
// then count from string 
// use regex for grouping the colors together 
