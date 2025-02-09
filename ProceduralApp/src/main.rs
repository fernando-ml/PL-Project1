use std::fs;
use std::f64::consts::PI;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct Moon {
    #[serde(rename = "Name")]
    name: String,
    #[serde(default)]
    #[serde(rename = "Diameter")]
    diameter: f64,
    #[serde(default)]
    #[serde(rename = "Circumference")]
    circumference: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Planet { 
    #[serde(rename = "Name")]
    name: String,
    #[serde(default)]
    #[serde(rename = "DistanceFromSun")]
    distance_from_sun: f64,
    #[serde(default)]
    #[serde(rename = "OrbitalPeriod")]
    orbital_period: f64,
    #[serde(default)]
    #[serde(rename = "Diameter")]
    diameter: f64,
    #[serde(default)]
    #[serde(rename = "Circumference")]
    circumference: f64,
    #[serde(default)]
    #[serde(rename = "Moons")]
    moons: Vec<Moon>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SolarSystem {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Diameter")]
    diameter: f64,
    #[serde(default)]
    #[serde(rename = "Circumference")]
    circumference: f64,
    #[serde(rename = "Planets")]
    planets: Vec<Planet>,
}

// we had to do some renaming since the JSON file had some conventions that Rust doesn't support. Rust support snake_case, but the JSON file has Name, Diameter, Circumference, etc.

fn calculate_circumference(diameter: f64) -> f64 {
    diameter * PI
}

fn calculate_diameter(circumference: f64) -> f64 {
    circumference / PI
}

fn calculate_orbital_period(distance: f64) -> f64 {
    (distance.powf(3.0)).sqrt()
}

fn calculate_distance(period: f64) -> f64 {
    (period.powf(2.0)).powf(1.0/3.0)
}

fn calculate_volume(diameter: f64) -> f64 {
    let radius = diameter / 2.0;
    (4.0/3.0) * PI * radius.powf(3.0)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <json_file>");
        std::process::exit(1);
    }

    let contents = fs::read_to_string(&args[1])
        .unwrap_or_else(|err| {
            eprintln!("Error reading file {}: {}", args[1], err);
            std::process::exit(1);
        });
    
    let mut solar_system: SolarSystem = serde_json::from_str(&contents)
        .unwrap_or_else(|err| {
            eprintln!("Error parsing JSON: {}", err);
            std::process::exit(1);
        });

    if solar_system.circumference == 0.0 {
        solar_system.circumference = calculate_circumference(solar_system.diameter);
    }

    println!("Sun: {}", solar_system.name);
    println!("Diameter: {:#.0} km", solar_system.diameter);
    println!("Circumference: {:#.0} km\n", solar_system.circumference);

    let mut total_planets_volume = 0.0;
    let sun_volume = calculate_volume(solar_system.diameter);

    for planet in &mut solar_system.planets {
        if planet.distance_from_sun == 0.0 && planet.orbital_period != 0.0 {
            planet.distance_from_sun = calculate_distance(planet.orbital_period);
        } else if planet.orbital_period == 0.0 && planet.distance_from_sun != 0.0 {
            planet.orbital_period = calculate_orbital_period(planet.distance_from_sun);
        }

        if planet.diameter == 0.0 && planet.circumference != 0.0 {
            planet.diameter = calculate_diameter(planet.circumference);
        } else if planet.circumference == 0.0 && planet.diameter != 0.0 {
            planet.circumference = calculate_circumference(planet.diameter);
        }

        total_planets_volume += calculate_volume(planet.diameter);

        println!("Planet: {}", planet.name);
        println!("Distance from sun: {:.2} au", planet.distance_from_sun);
        println!("Orbital period: {:.2} yr", planet.orbital_period);
        println!("Diameter: {:#.0} km", planet.diameter);
        println!("Circumference: {:#.0} km", planet.circumference);

        for moon in &mut planet.moons {
            if moon.diameter == 0.0 && moon.circumference != 0.0 {
                moon.diameter = calculate_diameter(moon.circumference);
            } else if moon.circumference == 0.0 && moon.diameter != 0.0 {
                moon.circumference = calculate_circumference(moon.diameter);
            }

            println!("Moon: {}", moon.name);
            println!("Diameter: {:#.1} km", moon.diameter);
            println!("Circumference: {:#.1} km", moon.circumference);
        }
        println!();
    }

    println!("All the planets' volumes added together could fit in the Sun: {}", total_planets_volume < sun_volume);
}