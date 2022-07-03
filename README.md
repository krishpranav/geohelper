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

let l1 = Location::new(27.740068, 85.337576);
let l2 = Location::new(27.740286, 85.337059);
let distance = l1.distance_to(&l2);

println!("Distance: {}", distance.meters());

```

## License:
- geohelper is licensed under [GPL-2.0 License](https://github.com/krishpranav/geohelper/blob/master/LICENSE)
