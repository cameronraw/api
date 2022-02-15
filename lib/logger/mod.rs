use std::{fs::File, io, io::{ErrorKind, Write}};

pub fn get_log_file() -> Result<File, io::Error> {

    let f = File::open("logs/log.txt");

    let f = match f {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("logs/log.txt"){
                Ok(fc) => fc,
                Err(err) => panic!("Problem creating log file"),
            },
            _ => panic!("Problem creating log file")
        }
    };

    return Ok(f);

}

pub fn write(message: &str){
    
    let mut file = get_log_file().unwrap();

    file.write_all(message.as_bytes()).unwrap();
}

