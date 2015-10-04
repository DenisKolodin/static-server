extern crate static_server;
extern crate hyper;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use static_server::provider;
use static_server::server;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::Connection;

fn get_content(source: &str) -> (StatusCode, String) {
    let client = Client::new();

    let mut res = client.get(source)
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    (res.status, body)
}

fn check_equals(port: u16, resource: &str, source: &str) {
	let resource = format!("http://localhost:{}{}", port, resource);
	let source = format!("examples/static{}", source);
	let resp = get_content(&resource);

	println!("{:?}", source);

	let mut f = File::open(source).unwrap();
	let mut s = String::new();
	f.read_to_string(&mut s).unwrap();
	assert_eq!(resp.0, StatusCode::Ok);
	assert_eq!(resp.1, s);
}

fn check_resources(port: u16) {
	check_equals(port, "/", "/index.html");
	check_equals(port, "/style.css", "/style.css");
	check_equals(port, "/js/app.js", "/js/app.js");
}

#[test]
fn test_folder_provider() {
	let p = provider::provider_from_folder(Path::new("examples/static"));
	let s = server::StaticServer::new(p);
	let _ = s.share(("localhost", 8081));
	check_resources(8081);
}

#[test]
fn test_tar_provider() {
	let p = provider::provider_from_tar(Path::new("examples/static.tar"));
	let s = server::StaticServer::new(p);
	let _ = s.share(("localhost", 8082));
	check_resources(8082);
}

#[test]
fn test_multiple_ports_with_one_provider() {
	let p = provider::provider_from_folder(Path::new("examples/static"));
	let s = server::StaticServer::new(p);
	for delta in 0..20 {
		let _ = s.share(("localhost", 12345 + delta));
	}
	for delta in 0..20 {
		check_resources(12345 + delta);
	}
}

/*
#[test]
fn test_own_provider() {
}
*/