use std::{
    fs::{remove_file, File, OpenOptions},
    io::{Read, Write},
    net::TcpListener,
    path::Path,
};

fn main() {
    let listener = TcpListener::bind("192.168.59.116:4000").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Incomming stream");
        let mut len = [0; 4];
        let readed = stream.read(&mut len).unwrap();
        let len = u32::from_be_bytes(len) as usize;
        println!("Readed {len}, {readed}");
        if readed > 0 {
            let mut buffer = Vec::new();
            let mut reads = 0;
            while reads < len {
                let mut chunk = [0; 128];
                let readed = stream.read(&mut chunk).unwrap();
                buffer.extend_from_slice(&chunk[0..readed]);
                reads += readed;
                println!("{reads}");
            }

            let mut file = OpenOptions::new()
                .write(true) // Enable writing
                .create(true) // Create the file if it doesn't exist
                .truncate(true) // Truncate the file if it exists
                .open("img.jpg")
                .unwrap();

            file.write_all(&buffer).unwrap();
            println!("Image get successfully");
        }
    }
}
