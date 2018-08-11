#![feature(generator_trait, generators)]

extern crate num;
extern crate num_traits;
#[macro_use]
extern crate num_derive;

pub mod disassembler;
pub mod ops;

#[cfg(test)]
mod tests {
    use std::char;
    use std::fs;
    use std::path::Path;
    #[test]
    fn format_opcodes() {
        let buf = fs::read(Path::new("opcodes.txt")).unwrap();
        let buf = String::from_utf8(buf).unwrap();
        for line in buf.lines() {
            let cols = line.split("\t").take(2).collect::<Vec<&str>>();
            let b = cols[0];
            let op = cols[1];

            let op = op
                .replace(char::is_numeric, "")
                .replace(|f| f == ' ' || f == ',', "_");
            println!("{} = {},", op, b)
        }
    }
}
