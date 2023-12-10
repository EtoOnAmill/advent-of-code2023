use std::fs;

fn main() {
    let file = fs::read_to_string("/home/etonit/advent-of-code2023/6day/input").ok().unwrap();
    let mut lines = file.split("\n");
    let stringy = |e:&mut dyn Iterator<Item=usize>| e
        .fold(String::new(), |mut acc,n| {acc.push_str(&n.to_string()); acc})
        .parse::<usize>().unwrap();

    let mut time = lines.next().unwrap().split(" ").filter_map(|e|e.parse::<usize>().ok());
    let true_time:usize = stringy(&mut time);

    let mut distance = lines.next().unwrap().split(" ").filter_map(|e|e.parse::<usize>().ok());
    let true_distance:usize = stringy(&mut distance);


    let mut zipped = time.zip(distance);
    let mut true_zip = [true_time,true_distance];

    let mut ways_to_win = 1;
    let [t,d] = true_zip;
        let mut beated = 0;
        println!("{} {}",t,d);
        for ptime in 0..t {
            let ttime = t - ptime;
            if d < ttime*ptime {
                beated += 1;
            }
        }
        ways_to_win *= beated;
    println!("{}",ways_to_win);
}
