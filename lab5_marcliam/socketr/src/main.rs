use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    ffi::CString,
};

static POSITION: std::sync::RwLock<(u32, u32)> = std::sync::RwLock::new((0, 0));

fn main() {
    Config_Pins();
    SPI_TFT_Reset();

    let fd = open_port(c"/dev/ttyACM0".as_ptr());

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
            handle_connection(stream, fd);
        });
    }
}

fn read_position(fd: i32) {
    loop {
        let mut lock = POSITION.write().unwrap();
        write_stop_and_go(fd);

        let position: (f32, f32) = read_port(fd);
        
        let position: (u32, u32) = (position.0 as u32, position.1 as u32);

        *lock = position;

        println!("Posició actualizada: {position:?}");

        // thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn write_servo_angle(fd: i32, angle: u8) {
    let num = format!("s{angle}*");
    let a = CString::new(num);
    dbg!(&a);
    let lock = POSITION.write().unwrap();
    let b = write_port(fd, a.unwrap().as_ptr());
    dbg!(b);
    drop(lock);
}

fn handle_servo_request(subpath: &str, fd: i32) {
    let Some(Ok(angle)) = subpath.split("/").last().map(|s| s.parse::<u8>()) else {
        return;
    };

    dbg!(angle);
    write_servo_angle(fd, angle);
}

fn handle_connection(mut stream: TcpStream, fd : i32) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");

    let start = http_request[0].bytes().position(|x| x == b'/').unwrap();
    let end = http_request[0][start..].bytes().position(|x| x == b' ').unwrap() + start; 

    let subpath = &http_request[0][start+1..end];

    dbg!(subpath);
    
    if subpath.starts_with("servo") {
        handle_servo_request(subpath, fd);
    }


    let response = format!("HTTP/1.1 200 OK\r\n\r\n<b>{:?}</b>", POSITION.read().unwrap());

    stream.write_all(response.as_bytes()).unwrap();
}

unsafe extern "C" {
    safe fn Config_Pins();
    safe fn SPI_TFT_Reset();
    safe fn open_port(port: *const std::os::raw::c_char) -> i32;
    safe fn fill(color: i32);
    safe fn write_stop_and_go(fd: i32);
    safe fn write_port(fd: i32, msg : *const std::os::raw::c_char) -> i32;
    safe fn paint_square(x: i32, y: i32, radius: i32);
    safe fn read_port(fd: i32) -> (f32, f32);
}
