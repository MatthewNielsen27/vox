
pub mod fwd;

mod parser_binary;
mod parser_ascii;

pub mod stl {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    use crate::fwd::Facet;
    use crate::parser_ascii;
    use crate::parser_binary;

    enum Encoding {
        Ascii,
        Binary
    }

    pub fn parse_from_file(path: &Path) -> Result<Vec<Facet>, String> {
        let mut file = File::open(&path).unwrap();

        match get_stl_encoding(&mut file)? {
            Encoding::Ascii =>  { parser_ascii::facets_from_ascii_stl(&mut file) }
            Encoding::Binary => { parser_binary::facets_from_binary_stl(&mut file) }
        }
    }

    /// Returns StlEncoding::Ascii if the file begins with 'solid', else StlEncoding::Binary
    ///
    fn get_stl_encoding(file: &mut File) -> Result<Encoding, String> {
        let parsed_header = {
            let mut bytes = [0; 5];
            file.read_exact(&mut bytes).expect("could not read 5 bytes from file header");
            String::from_utf8(Vec::from(bytes))
        };

        match parsed_header {
            Err(msg) => {  Err(msg.to_string()) }

            Ok(header) => {
                let encoding = {
                    if header == "solid" {
                        Encoding::Ascii
                    } else {
                        Encoding::Binary
                    }
                };

                Ok(encoding)
            }
        }
    }
}
