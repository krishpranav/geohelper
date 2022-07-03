# geohelper
geolocation utilities for rust projects and applications

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

## Installation:
```rust
geohelper = { git = "https://github.com/krishpranav/geohelper" }
```

## Usage:
```rust
use geohelper::Location;

fn main() {
    /**
     * random cordinates of two location
     */
    let location1 = Location::new(27.740068, 85.337576);
    let location2 = Location::new(27.740286, 85.337059);

    /**
     * calculate the distance between the two location
     */
    let distance = location1.distance_to(&location2).unwrap();

    /**
     * print them out!!
     */
    println!("Distance: {}", distance.meters());
}
```

## License:
- geohelper is licensed under [GPL-2.0 License](https://github.com/krishpranav/geohelper/blob/master/LICENSE)
