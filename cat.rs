use std::fs::File;
use std::io::{self,Read,Write};
use std::str;
use std::iter;
use std::env;

fn main() -> io::Result<()>{
   let args: Vec<String> = env::args().collect();
//   println!("{}",args[1]);
   let mut buffer = Vec::new();

    if args.len() >= 2{
        let file = File::open(&args[1]);
       file?.read_to_end(&mut buffer)?;
    }else{
        let mut file = io::stdin();
        file.read_to_end(&mut buffer)?;
    }
    let mut pos =0;
    let mut stdout = io::stdout();
    let mut j = 0;
    let j =0;
    while pos < buffer.len(){
        for i in 0..buffer.len(){
            let k = 0;
            if buffer[i] == '\n' as u8{
                let j = i;
                let bytes_written=stdout.write(&buffer[k..j+1]);
                pos = pos +j -k+1;
                continue;
            }
        }

       } 
       Ok(())
}
