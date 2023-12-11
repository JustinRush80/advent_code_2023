'_game:for line in reader.lines(){
    let game_line:&str = &line.unwrap();
    let vec:Vec<&str> = game_line.split(";").collect();
    let game:u32 = index.captures(game_line).unwrap()[1].parse().unwrap();

    for set in &vec {

    let mut blue:u32 = 0;
    let mut green:u32 = 0;
    let mut red:u32 =0;
   
    let blue_caps = blue_re.captures_iter(set);
    let green_caps = green_re.captures_iter(set);
    let red_caps = red_re.captures_iter(set);

    for cap in blue_caps {
       let blue_num:u32 =  cap[1].parse().unwrap();
       blue+=blue_num
    }

    for cap in green_caps {
        let green_num:u32 =  cap[1].parse().unwrap();
        green+=green_num
     }
    
    for cap in red_caps {
        let red_num:u32 =  cap[1].parse().unwrap();
        red+=red_num
     }

    let possible:bool = (blue <= total_blue) & (green <= total_green) &(red <= total_red);

    if possible == false {
        continue '_game;
    }
    // println!("{}",blue)


    }

println!("{}",game);
res+=game;

}

println!("{}",res);