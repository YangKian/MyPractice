use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // incoming 方法返回一个 TcpStream 的迭代器，每个 stream 表示一个打开的连接
    for stream in listener.incoming() {
        // 之所以要对 stream 调用 unwrap 来捕获错误是因为迭代器迭代的实际上是连接的尝试，
        // 因此有可能出现连接失败的情况
        // 当 stream 离开作用域后，因为其实现了 drop trait，连接会自动完成关闭
        let stream = stream.unwrap();

        pool.execute(||{
            handle_connection(stream);
        });
    }
}

// stream 必须是 mut 的，因为 read 方法可能会读取超过我们设定 buffer 大小的数据，这个时候
// 需要修改 stream 来存储数据，再下次调用 read 时返回。即，read 方法的参数是 &mut 类型
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // String::from_utf8_lossy：将 &[u8] 转换为 String，包括不合法的字符也会被转换
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // 因为我们需要将原始字节读入 buffer，所以通过添加 b"" byte string syntax
    // 来将 &str 转为 &[u8]
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
