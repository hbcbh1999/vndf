use acceptance::TreeBuilder;
use hyper::status;
use hyper::header::common::location::Location;

use infra::{
	DotResponse,
	Rocks,
};


#[test]
fn it_should_return_a_custom_response() {
	let tree = TreeBuilder::new()
		.with_file(
			"source/localhost/test.response",
			DotResponse::new(301, "/other-directory").build().as_slice()
		)
		.build();

	let rocks = Rocks::start(tree);

	let response = rocks.request("/test").send();

	assert_eq!(status::MovedPermanently, response.status);
	assert_eq!(
		&Location(format!("http://localhost:{}/other-directory", rocks.port)),
		response.headers.get::<Location>().unwrap()
	);
}

#[test]
fn it_should_return_a_custom_response_for_a_directory() {
	let tree = TreeBuilder::new()
		.with_file(
			"source/localhost/test/.response",
			DotResponse::new(301, "/other-directory").build().as_slice()
		)
		.build();

	let rocks = Rocks::start(tree);

	let response = rocks.request("/test").send();

	assert_eq!(status::MovedPermanently, response.status);
	assert_eq!(
		&Location(format!("http://localhost:{}/other-directory", rocks.port)),
		response.headers.get::<Location>().unwrap()
	);
}
