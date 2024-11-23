use std::{
    fs::{remove_file, File},
    io::{Read, Write},
    net::TcpListener,
    path::Path,
};

fn main() {
    let listener = TcpListener::bind("192.168.1.123:4000").unwrap();

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

            let mut file = File::create("img.jpg").unwrap();
            file.write_all(&buffer).unwrap();
            println!("Image get successfully");
        }
    }
}
