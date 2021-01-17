
use raytrace_weekend::raytracer::config::Config;
extern crate clap;
use clap::{App, Arg};
fn main() {
    let matches = App::new("raytrace_weekend")
        .arg(Arg::with_name("width")
        .short("w")
        .long("width")
        .required(false)
        .takes_value(true))

        .arg(Arg::with_name("samples")
        .short("s")
        .long("samples")
        .required(false)
        .takes_value(true)
        .default_value("10"))

        .arg(Arg::with_name("bounces")
        .short("b")
        .long("bounces")
        .required(false)
        .takes_value(true)
        .default_value("10"))
        .get_matches();

    
    let config = Config::from(&matches);
    
    
    println!("Raytracer config: {}", config);
    println!("Starting raytracer...");
    
    raytrace_weekend::run(config);

    
}

