#[macro_use]
extern crate log;
extern crate simple_logger;

extern crate uopen;

use std::env;



fn main() {
    simple_logger::init().unwrap();

    info!("start app...");
    let args: Vec<String> = env::args().collect();
    debug!("get options {:?}", args);

    if args.len() < 2 {
        error!("args length some wrong!");
        std::process::exit(1);
    }

    uopen::open(&args[1]);

    // let fi = uopen::get_config("default-file-manager");
    // println!("{:?}", fi);

}
