mod input;

fn main() {
    let mut total_paper = 0;

    let sizes_string = input::get_sizes();
    let sizes_vec: Vec<&str> = sizes_string.split('\n').collect();

    for sizes in sizes_vec {
        let sizes_trimmed = sizes.trim();
        let sizes_i32: Vec<i32> = sizes_trimmed
            .split('x')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let areas = vec![
            2 * sizes_i32[0] * sizes_i32[1],
            2 * sizes_i32[1] * sizes_i32[2],
            2 * sizes_i32[2] * sizes_i32[0],
        ];

        let smallest_face = areas.iter().min().unwrap() / 2;
        total_paper += areas.iter().sum::<i32>() + smallest_face;
    }

    println!(
        "They need to order {} square feet of wrapping paper 🎁.",
        total_paper
    );
}
