use std::fs::File;
use std::path::Path;
use std::io::BufReader;

type WaveData = Vec<(u64, f32)>;

fn open_csv_file(file: &Path) -> Option<File> {
    match File::open(file) {
        Ok(f) => Some(f),
        Err(e) => {
            println!("Error: {:?}", e);
            None
        }
    }
}

pub fn open_and_parse_csv(file: &Path) -> Option<WaveData> {
    // TODO: (emilio) PR för:
    //   open?
    //   read_content?
    //   parse_content?
    match open_csv_file(file) {
        Some(f) => read_and_parse_file(f),
        None => None,
    }
}

fn read_and_parse_file(file: File) -> Option<WaveData> {
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    match buf_reader.read_to_string(&mut contents) {
        Ok(num_bytes) => {
            println!("Read {} number of bytes", num_bytes);
            parse_contents(contents)
        }
        Err(e) => {
            println!("Could not read data from file: {:?}", e);
            None
        }
    }
}

fn parse_contents(contents: String) -> Option<WaveData> {
    // TODO: do the parsing from String -> WaveData
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4); 
    }
}
