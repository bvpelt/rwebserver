#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::io::{BufRead, BufReader, Error, Write, stdin};
use std::net::{TcpListener, TcpStream};
use std::{env, str, thread};

#[derive(Serialize, Deserialize, Debug)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

fn handle_client(stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut data = Vec::new();
    let mut stream = BufReader::new(stream);

    loop {
        println!("Start reading data");
        data.clear();
        let bytes_read = stream.read_until(b'\n', &mut data)?;
        println!("bytes_read: {}", bytes_read);
        if bytes_read == 0 {
            return Ok(());
        }
        let input: Point3D = serde_json::from_slice(&data)?;
        println!("received input: {:?}", input);
        let value = input.x.pow(2) + input.y.pow(2) + input.z.pow(2);
        write!(stream.get_mut(), "{}", f64::from(value).sqrt())?;
        write!(stream.get_mut(), "{}", "\n")?;
    }
    // println!("End of reading data");
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Please provice --client or --server as argument");
        std::process::exit(1);
    } else if args[1] == "--client" {
        let mut stream = TcpStream::connect("127.0.0.1:8888").expect("Could not connect to server");
        println!("Please provide a 3D point as three comma separated integers");
        loop {
            let mut input = String::new();
            let mut buffer: Vec<u8> = Vec::new();

            stdin()
                .read_line(&mut input)
                .expect("Failed to read from stdin");

            let parts: Vec<&str> = input.trim_matches('\n').split(',').collect();
            let point = Point3D {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
                z: parts[2].parse().unwrap(),
            };

            let json = serde_json::to_string(&point).unwrap() + "\n";
            stream
                .write_all(json.as_bytes())
                .expect("Failed to write to server");

            let mut reader = BufReader::new(&stream);

            reader
                .read_until(b'\n', &mut buffer)
                .expect("Could not read info buffer");

            let input = str::from_utf8(&buffer).expect("Could not write buffer as string");
            println!("Server response: {}", input);
            if input == "" {
                eprintln!("Empty response from server")
            }
        }
    } else if args[1] == "--server" {
        let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");
        for stream in listener.incoming() {
            match stream {
                Err(e) => {
                    eprintln!("failed: {}", e)
                }
                Ok(stream) => {
                    thread::spawn(move || {
                        handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                    });
                }
            }
        }
    }
}
