mod input;

fn main() {
    let locations = input::get_input();
    let mut visited_houses: Vec<(i32, i32)> = vec![(0, 0)];
    let mut current_location = vec![(0, 0), (0, 0)];

    for (index, location) in locations.chars().enumerate() {
        let delivery = index % 2;

        match location {
            '^' => current_location[delivery].1 += -1,
            'v' => current_location[delivery].1 += 1,
            '>' => current_location[delivery].0 += 1,
            '<' => current_location[delivery].0 += -1,
            _ => {}
        }

        if !visited_houses
            .iter()
            .any(|&i| i == current_location[delivery])
        {
            visited_houses.push(current_location[delivery]);
        }
    }

    println!(
        "Santa 🎅 and his robot 🤖 ended up delivering presents 🎁 to {:?} houses 🏠!",
        visited_houses.len()
    );
}
