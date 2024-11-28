use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

static POSITION: std::sync::RwLock<(u32, u32)> = std::sync::RwLock::new((0, 0));

fn main() {
    Config_Pins();
    SPI_TFT_Reset();

    let fd = open_port();

    fill(0xFFFFFF);

    paint_square(100, 100, 3);

    let listener = TcpListener::bind("0.0.0.0:80").unwrap();

    println!("Listening at: http://10.10.73.197/");

    thread::spawn(move || {
        read_position(fd);
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(move || {
            handle_connection(stream);
        });
    }
}

fn read_position(fd: i32) {
    loop {
        write_stop_and_go(fd);

        let position: (f32, f32) = read_port(fd);
        
        let position: (u32, u32) = (position.0 as u32, position.1 as u32);

        *POSITION.write().unwrap() = position;

        println!("Posició actualizada: {position:?}");

        // thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // println!("Request: {http_request:#?}");

    let response = format!("HTTP/1.1 200 OK\r\n\r\n<b>{:?}</b>", POSITION.read().unwrap());

    stream.write_all(response.as_bytes()).unwrap();
}

unsafe extern "C" {
    safe fn Config_Pins();
    safe fn SPI_TFT_Reset();
    safe fn open_port() -> i32;
    safe fn fill(color: i32);
    safe fn write_stop_and_go(fd: i32);
    safe fn paint_square(x: i32, y: i32, radius: i32);
    safe fn read_port(fd: i32) -> (f32, f32);
}
