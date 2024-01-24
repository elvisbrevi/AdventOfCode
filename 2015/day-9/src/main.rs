use std::{collections::HashMap, u16::MAX};

fn main() {
    let contents = include_str!("../input.txt");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut route_hashmap = HashMap::new();
    let mut destiny = String::new();

    for line in lines {
        // concatenate route with it's distance
        let split_line = line.split(" = ").collect::<Vec<&str>>();
        let route = *split_line.get(0).unwrap();
        let distance = split_line
            .get(1)
            .unwrap()
            .replace("\r", "")
            .parse::<i32>()
            .unwrap();
        route_hashmap.insert(route, distance);

        // search destiny
        let right_city = route.split_whitespace().last().unwrap();
        let city_to_match = right_city.to_string() + " to ";
        let only_one = contents.matches(&city_to_match).count() == 0;
        if only_one {
            destiny = right_city.to_string();
        }
    }

    println!("{}", destiny);
    let total_distance = get_total_distance(&destiny, &route_hashmap);
    println!("{}", total_distance)
}

fn get_total_distance(destiny: &str, route_hashmap: &HashMap<&str, i32>) -> i32 {
    let city_to_match = "to ".to_string() + destiny;
    let mut min_distance = i32::MAX;
    for (key, val) in route_hashmap {
        if key.contains(&city_to_match) && val < &min_distance {
            let key_cities: Vec<&str> = key.split("to").collect();
            let left_city = key_cities.get(0).unwrap().replace(" ", "");
            min_distance = *val;
            min_distance = min_distance + get_total_distance(&left_city, route_hashmap);
        }
    }
    if min_distance == i32::MAX {
        return 0;
    } else {
        min_distance
    }
}
