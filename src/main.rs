fn main() {
    let input = "hello my name is chatgpt".to_uppercase();
    let dot = '.';
    let dash = '-';

    for character in input.chars() {
        match character {
            'A' => print!("{}{} ", dot, dash),
            'B' => print!("{}{}{}{} ", dash, dot, dot, dot),
            'C' => print!("{}{}{}{} ", dash, dot, dash, dot),
            'D' => print!("{}{}{} ", dash, dot, dot),
            'E' => print!("{} ", dot),
            'F' => print!("{}{}{}{} ", dot, dot, dash, dot),
            'G' => print!("{}{}{} ", dash, dash, dot),
            'H' => print!("{}{}{}{} ", dot, dot, dot, dot),
            'I' => print!("{}{} ", dot, dot),
            'J' => print!("{}{}{}{} ", dot, dash, dash, dash),
            'K' => print!("{}{}{} ", dash, dot, dash),
            'L' => print!("{}{}{}{} ", dot, dash, dot, dot),
            'M' => print!("{}{} ", dash, dash),
            'N' => print!("{}{} ", dash, dot),
            'O' => print!("{}{}{} ", dash, dash, dash),
            'P' => print!("{}{}{}{} ", dot, dash, dash, dot),
            'Q' => print!("{}{}{}{} ", dash, dash, dot, dash),
            'R' => print!("{}{}{} ", dot, dash, dot),
            'S' => print!("{}{}{} ", dot, dot, dot),
            'T' => print!("{} ", dash),
            'U' => print!("{}{}{} ", dot, dot, dash),
            'V' => print!("{}{}{}{} ", dot, dot, dot, dash),
            'W' => print!("{}{}{} ", dot, dash, dash),
            'X' => print!("{}{}{}{} ", dash, dot, dot, dash),
            'Y' => print!("{}{}{}{} ", dash, dot, dash, dash),
            'Z' => print!("{}{}{}{} ", dash, dash, dot, dot),
            _ => (),
        }
    }
}
