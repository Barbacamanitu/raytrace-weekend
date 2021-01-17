use clap::ArgMatches;

pub struct Config {
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u32,
    pub bounce_depth: u32
}



impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Width: {}, Height: {}, Samples-Per-Pixel: {}, Bounce-Depth: {})", self.width, self.height, self.samples_per_pixel,self.bounce_depth)
    }
}

impl<'a> From<&ArgMatches<'a>> for Config {
    fn from(matches: &ArgMatches) -> Self {    
        let width = match matches.value_of("width") {
            Some(w_str) => { w_str.parse::<u32>().unwrap() }
            None => {200}
        };

        let height = ((9.0/16.0) * (width as f64)) as u32;

        let samples = matches.value_of("samples").unwrap().parse::<u32>().unwrap();

        let bounce_depth = matches.value_of("bounces").unwrap().parse::<u32>().unwrap();
        Config {
            width: width,
            height: height,
            samples_per_pixel: samples,
            bounce_depth: bounce_depth
        }
    }
}
