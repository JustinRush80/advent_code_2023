
use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main()-> io::Result<()>  {

   

   let f = File::open("input.txt")?;
   let reader = BufReader::new(f);



   let mut res:u32 = 0;
   
   
   for line in reader.lines(){
   let mut vec = Vec::new();
   
        for letter in line?.chars(){
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


