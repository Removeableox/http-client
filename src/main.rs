mod http;
use http::get_http;

fn main() {
    let url = String::from("127.0.0.1:80");
    let res = get_http(url);
    println!("{}", res);
}

