mod input;

fn main() {
    let mut total_paper = 0;
    let mut total_ribbon = 0;

    let sizes_string = input::get_sizes();
    let sizes_vec: Vec<&str> = sizes_string.split('\n').collect();

    for sizes in sizes_vec {
        let sizes_trimmed = sizes.trim();
        let sizes_i32_result: Result<Vec<i32>, _> =
            sizes_trimmed.split('x').map(|s| s.parse::<i32>()).collect();

        let mut sizes_i32 = vec![];
        if let Some(result) = sizes_i32_result.ok() {
            sizes_i32 = result;
        }

        let areas = vec![
            2 * sizes_i32[0] * sizes_i32[1],
            2 * sizes_i32[1] * sizes_i32[2],
            2 * sizes_i32[2] * sizes_i32[0],
        ];

        if let Some(min) = areas.iter().min() {
            let smallest_face = min / 2;
            total_paper += areas.iter().sum::<i32>() + smallest_face;
        }

        // perimeter
        let sizes_sum = sizes_i32.iter().sum::<i32>();
        let mut perimeter = 0;
        if let Some(max) = sizes_i32.iter().max() {
            perimeter = 2 * (sizes_sum - max);
        }

        // volume
        let mut volume = 1;
        for size in sizes_i32 {
            volume *= size
        }

        total_ribbon += volume + perimeter;
    }

    println!(
        "They need to order {} square feet of wrapping paper 🎁.",
        total_paper
    );
    println!("We need {} feet of ribbon in total 🎀.", total_ribbon);
}
