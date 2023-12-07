mod input;

fn main() {
    let locations = input::get_input();
    let mut visited_houses: Vec<(i32, i32)> = vec![(0, 0)];
    let mut current_location = (0, 0);

    for location in locations.chars() {
        match location {
            '^' => current_location.1 += -1,
            'v' => current_location.1 += 1,
            '>' => current_location.0 += 1,
            '<' => current_location.0 += -1,
            _ => {}
        }

        if !visited_houses.iter().any(|&i| i == current_location) {
            visited_houses.push(current_location);
        }
    }

    println!(
        "Santa ended up delivering presents to {:?} houses! 🏠🎁",
        visited_houses.len()
    );
}
