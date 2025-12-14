use std::{env::args, fs::read_to_string, io};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn it_should_do_the_example() {
        assert_eq!(true, true);
    }
}
