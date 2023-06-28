use std::collections::HashMap;

// Struct to hold address information
struct Address {
    street: String,
    city: String,
    state: String,
    zip: u32,
}

// Struct to hold Housing Development data
struct HousingDevelopment {
    name: String,
    address: Address,
    number_of_units: u32,
    estimated_cost: f64,
}

// Vector to store all of the housing developments
let mut housing_developments: Vec<HousingDevelopment> = Vec::new();

// Function to add a housing development
fn add_housing_development(name: String, street: String, city: String,
    state: String, zip: u32, num_units: u32, estimated_cost: f64) {
    let addr = Address {
        street: street,
        city: city,
        state: state,
        zip: zip,
    };

    let development = HousingDevelopment {
        name: name,
        address: addr,
        number_of_units: num_units,
        estimated_cost: estimated_cost,
    };

    housing_developments.push(development);
}

// Hashmap to store housing developments by city
let mut housing_by_city: HashMap<String, Vec<HousingDevelopment>> = HashMap::new();

// Function to create a mapping of cities to housing developments
fn map_housing_by_city() {
    for development in housing_developments.iter() {
        let city_name = &development.address.city;
        if !housing_by_city.contains_key(city_name) {
            let mut developments = Vec::new();
            developments.push(development.clone());
            housing_by_city.insert(city_name.to_string(), developments);
        } else {
            let developments = housing_by_city.get_mut(city_name).unwrap();
            developments.push(development.clone());
        }
    }
}

// Function to get a list of all housing developments within a given city
fn get_housing_in_city(city_name: &str) -> Vec<HousingDevelopment> {
    let mut results = Vec::new();
    if housing_by_city.contains_key(city_name) {
        results = housing_by_city.get(city_name).unwrap().clone();
    }
    results
}

// Function to calculate the total cost of all housing developments
fn get_total_cost() -> f64 {
    let mut total_cost = 0.0;
    for development in housing_developments.iter() {
        total_cost += development.estimated_cost;
    }
    total_cost
}

// Function to get the total number of units for all housing developments
fn get_total_units() -> u32 {
    let mut total_units = 0;
    for development in housing_developments.iter() {
        total_units += development.number_of_units;
    }
    total_units
}

fn main() {
    // Example data
    add_housing_development("Highland Park".to_string(),
        "123 Main St".to_string(),
        "New York".to_string(),
        "NY".to_string(),
        10004,
        400,
        5000000.0);

    add_housing_development("Sunset Ridge".to_string(),
        "456 Second Ave".to_string(),
        "New York".to_string(),
        "NY".to_string(),
        10004,
        500,
        8000000.0);

    add_housing_development("Haley Estates".to_string(),
        "789 Third Ave".to_string(),
        "San Francisco".to_string(),
        "CA".to_string(),
        94110,
        600,
        7000000.0);

    // Create the mapping of cities to housing developments
    map_housing_by_city();

    // Get the total cost
    println!("Total cost: {}", get_total_cost());

    // Get the total number of units
    println!("Total units: {}", get_total_units());

    // Get all housing developments in New York
    let developments_in_ny = get_housing_in_city("New York");
    println!("Number of developments in New York: {}", developments_in_ny.len());
}