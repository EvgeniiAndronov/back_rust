fn main() {
    let size_sq: i8 = 127;

    piramide_creator(size_sq, size_sq);
}

fn writter(line: Vec<String>) -> String {
    let mut l: String = String::new();
    for i in line {
        l += &format!("{}", i);
    }
    l += "\n";
    l
}

fn piramide_creator(size: i8, n: i8) {
    let mut i = 1;
    loop {
        let res = create_line(size, i);
        println!("{}", writter(res));
        i += 2;
        if i >= n {
            let res = create_line(size, i);
            println!("{}", writter(res));
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn writter_test() {
        let test_v: Vec<String> = vec![" ".to_string(), "-".to_string(), " ".to_string()];
        let test_a: String = " - \n".to_string();
        assert_eq!(writter(test_v), test_a);
    }
}