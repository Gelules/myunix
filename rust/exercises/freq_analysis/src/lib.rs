pub fn freq_analysis(text: &String, table: &String) {
    let text = text.as_bytes();
    let table = table.as_bytes();

    for i in 0..text.len() {
        print!("{}", text[i] as char);
    }

    println!("");

    for i in 0..table.len() {
        print!("{}", table[i] as char);
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        println!("'AAB', 'XY':\nIt should be:\nA X\nB Y\n---");
        freq_analysis(&String::from("AAB"), &String::from("XY"));
        println!("===\n");
    }

    #[test]
    fn second() {
        println!("'ABBCCCDDDD', 'ABCD':\nIt should be:\nA D\nB C\nC B\nD A\n---");
        freq_analysis(&String::from("ABBCCCDDD"), &String::from("ABCD"));
        println!("===\n");
    }

    #[test]
    fn third() {
        println!("'FXOWFFOWOFF', 'ABCD':\nIt should be:\nF A\nO B\nW C\nX D\n---");
        freq_analysis(&String::from("FXOWFFOWOFF"), &String::from("ABCD"));
        println!("===\n");
    }

}
