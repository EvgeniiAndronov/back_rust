fn main() {
    let size_sq: i8 = 40;

    piramide_creator(size_sq, size_sq);
}

fn writter(line: Vec<String>) {
    for i in line {
        print!("{}", i);
    }
    print!("\n");
}

fn piramide_creator(size: i8, n: i8) {
    let mut i = 1;
    loop {
        let res = create_line(size, i);
        writter(res);
        i += 2;
        if i >= n {
            let res = create_line(size, i);
            writter(res);
            break;
        }
    }
}

fn create_line(size: i8, n: i8) -> Vec<String> {
    let mut line: Vec<String> = vec![];
    for _ in 0..(size/2 - n/2) {
        line.push(" ".to_string());
    }
    for _ in 0..n {
        line.push("-".to_string());
    }
    for _ in 0..(size/2 - n/2) {
        line.push(" ".to_string());
    }

    line
}