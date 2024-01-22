use std::collections::HashMap;

fn main() {
    let contents = include_str!("../input-test.txt");
    let lines: Vec<&str> = contents.split("=").collect();
    let mut route_hashmap = HashMap::new();

    // fill route_hashmap
    for index in (0..lines.len()).step_by(2) {
        let route = *lines.get(index).unwrap();
        let distance = lines.get(index - 1).unwrap().parse::<i32>().unwrap();
        route_hashmap.insert(route, distance);
    }

    let destiny = String::new();
    // search destiny
    let search_destiny = |city: &str| {
        for r in routes {}
    };
}
