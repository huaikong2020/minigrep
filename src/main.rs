use std::env;
use std::process;

fn main() {
    // let args:Vec<String> = env::args().collect();
    // println!("{:?}",args);
    let config = minigrep::Config::new(env::args()).unwrap_or_else(|err|{
        println!("{err}");
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config){
        println!("application error:{e}");
        process::exit(1);
    }
}



