use std::fmt::{self, Display, Formatter};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lat >= 0.0 { 'E' } else { 'W' };

        write!(
            f,
            "{}: {:.3}Deg{} {:3}Deg{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.blue, self.red, self.green,)
    }
}

fn main() {
    for city in [
        City {
            name: "Dublin",
            lat: 5.347778,
            lon: -6.259722,
        },
        City {
            name: "Olso",
            lat: 5.95,
            lon: 10.75,
        },
        City {
            name: "Vancover",
            lat: 4.25,
            lon: -123.1,
        },
    ] {
        println!("{}", city);
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        println!("RGG {}", color);
    }
}
