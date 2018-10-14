use std::process::Command;

fn main() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("mpstat -P ALL 1 1 | awk '/Average:/ && $2 ~ /[0-9]/ {print $3}'")
        .output()
        .expect("failed to execute process")
        .stdout;

    let nums = match String::from_utf8(output) {
        Ok(s) => s
            .split('\n')
            .into_iter()
            .map(|s| s.parse::<f32>())
            .filter_map(Result::ok)
            .collect::<Vec<f32>>(),
        Err(e) => panic!("{}", e),
    };

    let sum: f32 = nums.clone().into_iter().sum();

    println!("{}", sum / nums.len() as f32);
}
