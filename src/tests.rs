use rocket;
use rocket::http::{ContentType, Status};
use rocket::local::Client;

#[test]
fn bad_get_put() {
    let client = Client::new(rocket()).unwrap();

    // Try to get a message with an ID that doesn't exist.
    let mut res = client
        .get("/not-a-route")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::NotFound);

    let body = res.body_string().unwrap();
    assert!(body.contains("error"));
    assert!(body.contains("Resource was not found."));

}
