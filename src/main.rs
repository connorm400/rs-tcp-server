use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{BufRead, BufReader, Write, ErrorKind},
};

mod multithreading;

use crate::multithreading::ThreadPool;

const THREADCOUNT: usize = 4; 
const IP: &str = "127.0.0.1";
const PORT: u16 = 7878;
const DOCUMENTS_PATH: &str = "www";

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(format!("{IP}:{PORT}").as_str())?;

    let pool = ThreadPool::new(THREADCOUNT);

    println!("server has started :)");
    println!("running at {IP}:{PORT}");
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_client(stream);
        });
        
    }   

    println!("Shutting down");

    Ok(())
}


fn handle_client(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let request_line = reader.lines()
        .next() // move over the first line 
        .unwrap().unwrap(); //unwraps the Option and the Result
    

    // we need the HTTP method and the path from the tcp stream. I split it with the space. 
    // I dont think its possible for the unwraps to panic since the two elements should always be there
    let mut split_request_iter = request_line.split(" ");
    let method = split_request_iter.next().unwrap();
    let path = split_request_iter.next().unwrap();

    println!("method: {method}, path: {path}");

    if method != "GET" {
        println!("early return because the http method isnt a GET");
        return;
    }

    let mut file_to_read = String::new();
    
    if path == "/" {file_to_read.push_str(format!("{DOCUMENTS_PATH}/index.html").as_str())}
    
    // put all custom file formats here
    else if path.contains(".css") {file_to_read.push_str(format!("{DOCUMENTS_PATH}/{path}").as_str())} 
        //you could alternatively have only one universal css file with a line like this:
        // else if path.contains(".css") {file_to_read.push_str(format!({DOCUMENTS_PATH}/style.css).as_str())}
    
    //else if path.contains(".png") {file_to_read.push_str(format!("{DOCUMENTS_PATH}/{path}").as_str())} 
    //else if path.contains(".ico") {file_to_read.push_str(format!("{DOCUMENTS_PATH}/{path}").as_str())}
    
    // this is the line for all html files that arent the index
    else {file_to_read.push_str(format!("{DOCUMENTS_PATH}/{path}.html").as_str())} 

    let (status, contents) = match fs::read_to_string(file_to_read) {
        Ok(c) => ("HTTP/1.1 200 OK", c),
        Err(_e) => { // we'll just assume that the error is just that the file doesnt exist
            if let ErrorKind::NotFound = _e.kind() {
                ("HTTP/1.1 404 NOT FOUND", fs::read_to_string(format!("{DOCUMENTS_PATH}/404.html").as_str())
                   .expect(format!("must have 404 in path ./{DOCUMENTS_PATH}/404.html").as_str()))
            } else {
                panic!("not sure how to handle this error...");
            }
        }
    };

    let len = contents.len();
    let responce = format!("{status}\r\nContent-Length: {len}\r\n\r\n{contents}");
    stream.write_all(responce.as_bytes()).unwrap();
    
}
