fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.contains(&String::from("--help")) || args.len() != 2 {
        help();
    }

    let mut content = args.into_iter().map(|path| {
        use std::fs::read_to_string;
        read_to_string(&path).expect(&format!("to read file file {}", path))
    });

    let f1 = content.next().unwrap();
    let f2 = content.next().unwrap();

    for line1 in f1.lines() {
        for line2 in f2.lines() {
            println!("{} {}", line1, line2);
        }
    }
}

fn help() -> ! {
    eprintln!(
        "cross-prod <files...>
        for every line of input of all the files,
        output the cross product
        "
    );
    std::process::exit(0)
}
