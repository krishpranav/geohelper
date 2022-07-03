## GeoHelper:

- Get center of certain coordinates:
```rust
use geohelper::Location;

fn main() {

    let location1 = Location::new(52.518611, 13.408056);
    let location2 = Location::new(55.751667, 37.617778);
    let center = Location::center(&vec![&location1, &location2]);

    println!("Center {}, {}", center.latitude(), center.longitude());
}
```

- Check point falls in a certain radius:
```rust
use geohelper::Location;

fn main() {
    let berlin = Location::new(52.518611, 13.408056);
    let moscow = Location::new(55.751667, 37.617778);
    let is_in_radius = berlin.is_in_circle(&moscow, Distance::from_meters(2000.0)).unwrap();

    println!("Is Berlin in 2000m of Moscow? {}", is_in_radius);
}
```