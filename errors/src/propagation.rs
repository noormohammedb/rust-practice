use std::{
    fs::{read_to_string, File},
    io,
    io::Read,
};

pub mod error_two {
    use super::*;
    pub fn read_username_from_file() -> Result<String, io::Error> {
        // let f = File::open("hello.txt");
        let mut f = File::open("myfile.txt")?;

        // let mut f = match f {
        //     Ok(file) => file,
        //     Err(e) => return Err(e),
        // };

        let mut s = String::new();

        // match f.read_to_string(&mut s) {
        //     Ok(_) => Ok(s),
        //     Err(e) => Err(e),
        // }

        f.read_to_string(&mut s)?;
        Ok(s)
    }

    pub fn read_username_from_file_two() -> Result<String, io::Error> {
        let mut data_string = String::new();
        File::open("Cargo.lock")?.read_to_string(&mut data_string)?;
        Ok(data_string)
    }

    pub fn read_username_from_file_three() -> Result<String, io::Error> {
        read_to_string("hello.txt")
    }
}
