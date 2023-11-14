use mongodb::{Client, options::ClientOptions};

pub async fn get_connection(uri: &str) {
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.app_name = Some("My App".to_string());
    let client = Client::with_options(client_options)?;
    return client;
}

trait DBHandler<T> {
    fn new(&self, uri: &str) -> Self {
        self.uri = uri;
    }

    async fn get_one(&self, uri: &str) {
        get_connection()
    }

}