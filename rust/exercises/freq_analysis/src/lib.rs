pub fn freq_analysis(text: &String, table: &String) {
    let text = text.as_bytes();
    let table = table.as_bytes();
    let mut alphabet: Vec<i32> = Vec::new();
    let mut count: Vec<i32> = Vec::new();
    

    for i in 'A'..='Z' {
        alphabet.push(i as i32);
        count.push(0);
    }

    for c in text {
        count[(*c as usize) - ('A' as usize)] += 1;
    }

    // sort_freq(alphabet, count);
    for i in 0..25 {
        let mut max = i;
        for j in (i + 1)..26 {
            if count[j] > count[max] {
                max = j;
            }
        }

        if max != i {
            let tmp = count[i];
            count[i] = count[max];
            count[max] = tmp;

            let tmp = alphabet[i];
            alphabet[i] = alphabet[max];
            alphabet[max] = tmp;
        }
    }

    for c in 'A'..='Z' {
        for i in 0..26 {
            if alphabet[i] == c as i32 && count[i] != 0 {
                println!("{} {}", c , table[i] as char);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        println!("\n'AAB', 'XY':\nExpected:\nA X\nB Y\n---");
        freq_analysis(&String::from("AAB"), &String::from("XY"));
        println!("===");
    }

    #[test]
    fn second() {
        println!("\n'ABBCCCDDDDD', 'ABCD':\nExpected:\nA D\nB C\nC B\nD A\n---");
        freq_analysis(&String::from("ABBCCCDDDDD"), &String::from("ABCD"));
        println!("===");
    }

    #[test]
    fn third() {
        println!("\n'FXOWFFOWOFF', 'ABCD':\nExpected:\nF A\nO B\nW C\nX D\n---");
        freq_analysis(&String::from("FXOWFFOWOFF"), &String::from("ABCD"));
        println!("===");
    }

}
