fn main() {
    let input = "iwrupvqb";
    let mut answer = 0;

    loop {
        let md5 = format!("{:?}", md5::compute(format!("{}{}", input, answer)));

        if &md5[..6] == "000000" {
            break;
        }

        answer += 1;
    }

    println!("{}", answer);
}
