use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // arrange
    let address = spawn_app();

    // user reqwest to create clients
    let client = reqwest::Client::new();

    // act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    // create listener
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // extract randomly chosen port
    let port = listener.local_addr().unwrap().port();

    // get server
    let server = zero2prod::run(listener).expect("Failed to bind address");

    // start server
    let _ = tokio::spawn(server);

    // return server address
    format!("http://127.0.0.1:{}", port)
}
