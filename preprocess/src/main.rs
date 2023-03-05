use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;

use rusoto_core::Region;
use rusoto_s3::{S3, S3Client, PutObjectRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the .pkl file into memory
    let mut file = File::open("img_links_df.pkl")?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Connect to the AWS S3 service
    let region = Region::default();
    let client = S3Client::new(region);

    // Define the bucket and object key to use
    let bucket_name = "my-bucket";
    let object_key = "img_links_df.pkl";

    // Create a PutObjectRequest and upload the .pkl file to S3
    let req = PutObjectRequest {
        bucket: bucket_name.to_owned(),
        key: object_key.to_owned(),
        body: Some(Cursor::new(contents)),
        ..Default::default()
    };
    let _result = client.put_object(req).sync()?;

    Ok(())
}
