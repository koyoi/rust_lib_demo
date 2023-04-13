mod generator;

pub fn print_randomnumber(){
    let n = generator::gen_ran();
    println!("rand(u8):{}",n);
}