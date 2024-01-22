use std::collections::HashMap;

fn main() {
    let contents = include_str!("../input-test.txt");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut route_hashmap = HashMap::new();

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
        let exist = contents.find(right_city).unwrap();
        println!("{}, {}", right_city, exist)
    }
    println!("{:?}", route_hashmap)

    // let destiny = String::new();
    // // search destiny
    // let search_destiny = |city: &str| {
    //     for r in routes {}
    // };
}
