
use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use regex::Regex;


fn main()-> io::Result<()>  {

    let digit_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five","5"),
        ("six","6"),
        ("seven","7"),
        ("eight","8"),
        ("nine","9")
    ]);

   
  
   let f = File::open("input.txt")?;
   let reader = BufReader::new(f);



   let mut res:u32 = 0;
   let digit = Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))").unwrap();
   
   
   for line in reader.lines(){
        let code:&str = &line.unwrap();
        let mut vec = Vec::new();
        let matches: Vec<_> = digit.find_iter(code).map(|m| m.as_str()).collect();
        println!("{:?}",matches);

    
  
    
   
        for letter in code.chars(){
            if letter.is_digit(10){
                vec.push(letter.to_digit(10).unwrap());
            }
        }
   let digit:u32 = vec[0]*10+vec[vec.len()-1];
   res+=digit;
    
    }

   
   println!("{:?}",res);
   Ok(())
}


// loop through the string 
// we only care about the first and last digit
// how to know what the place of the numbers are 
// grab all the numbers from the string and put them in a list and 
// return res[0]res[-1]


