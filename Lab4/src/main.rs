use std::collections::BTreeSet;
use std::num::NonZero;
use std::{fs, hint, time};
use std::net::{TcpListener, TcpStream};
use std::time::Instant;
use std::time::Duration;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

fn divisors(n: NonZero<u32>) -> BTreeSet<NonZero<u32>> {
    let mut set : BTreeSet<NonZero<u32>> = BTreeSet::new();
    let nu = n.get();
    for i in 1..=nu.isqrt() {
        if nu % i == 0{
            set.insert(NonZero::new(i).unwrap());
            if i != nu.isqrt() {
                set.insert(NonZero::new(nu / i).unwrap());
            }
        }
    }
    set
}

fn assert_sorted(buf: &[i32]) {
    for pair in buf.windows(2) {
        if pair[0] > pair[1] {
            panic!("{:?} > {:?}", pair[0], pair[1]);
        }
    }
}

fn divisors_benchmark() {
    let start = Instant::now();
    for i in 1..=10000 {
        let s = hint::black_box(divisors(NonZero::new(i).unwrap()));
        
    }
    let elapsed = time::Instant::now().duration_since(start);
    let mean = elapsed.as_millis() as f64 / 100f64;
    println!("Mean time: {:.20}", mean);
}

fn bulk_write(stream: &mut TcpStream, buf: &[u8]) -> io::Result<()> {
    let mut p = buf;
    let mut c: usize;
    let mut io_res;
    loop {
        io_res = stream.write(&p);
        if io_res.is_err() {
            return Err(io_res.err().unwrap());
        }
        else { 
            c = io_res.ok().unwrap();
        }
        if c == 0{
            break;
        }
        p = &p[c..];
    }
    Ok(())
}

fn bulk_read(stream: &mut TcpStream, size: usize) -> io::Result<Vec<u8>> {
    let mut buf = vec![0u8; size];
    let mut c: usize;
    let mut len: usize = 0;
    let mut io_res;
    while len < size {
        io_res = stream.read(&mut buf[len..]);
        if io_res.is_err() {
            return Err(io_res.err().unwrap());
        }
        else {
            c = io_res.ok().unwrap();
        }
        if c == 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, ""));
        }
        len += c;
    }
    Ok(buf)
}

fn handle_client(stream: TcpStream) -> io::Result<()> {
    let mut buf:Vec<u8>;
    buf = bulk_read(&mut stream, 1)?;
    let conv_res =  PathBuf::from_str(String::from_utf8(buf).unwrap().as_str());
    match conv_res {
        Err(_) => {
            bulk_write(&mut &stream, str::as_bytes("Bad path\n"))?;
            Ok(())
        }
        Ok(path) => {
            match fs::read_dir(path) {
                Err(e) => {
                    bulk_write(&mut &stream, str::as_bytes("Bad dir\n"))?;
                    println!("{}", e.to_string());
                    Ok(())
                }
                Ok(paths) => {
                    let mut message : String = String::new();
                    for path in paths {
                        match path {
                            Err(e) => {
                                bulk_write(&mut &stream, str::as_bytes("Bad dir\n"))?;
                                println!("{}", e.to_string());
                            },
                            Ok(entry) => {
                                message.push_str(entry.file_name().into_string().unwrap().as_str());
                            }
                        }
                    }
                    bulk_write(&mut &stream, message.as_bytes())?;
                    Ok(())
                }
            }
        }
    }
    
    
}
fn main() {
    divisors_benchmark();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        handle_client(stream).unwrap();
    }
}
