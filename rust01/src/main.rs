fn main() {
    let mut count = 0;

    let measurements: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let mut previous: &i32 = &measurements[0];

    for val in measurements.iter() {
        if val > previous {
            count += 1;
        }
        previous = val;
    }
    println!("Number of increases: {}", count);
}
