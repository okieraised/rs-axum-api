
use aws_config::Region;
use aws_sdk_s3::{Client, Config};
use aws_credential_types::Credentials;
use aws_sdk_s3::operation::list_buckets::{ListBucketsError, ListBucketsOutput};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct S33Client {
    client: Client,
}


pub async fn new_client() {
    let creds = Credentials::from(Credentials::new("minioadmin", "minioadmin", None, None, ""));
    let config = Config::builder()
        .endpoint_url("http://127.0.0.1:9000")
        .credentials_provider(creds)
        .region(Region::new("us-east-1"))
        .build();

    let client = Client::from_conf(config);



    let b_res = client.list_buckets().send().await;
    match b_res {
        Ok(b) => {
            println!("{:?}", b)
        }
        Err(err) => {
            panic!("{}", err)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_client() {
        new_client().await
    }


}


