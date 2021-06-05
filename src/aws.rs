use rusoto_s3::S3Client;
use rusoto_s3::S3;
use rusoto_core;

// const CLIENT: S3Client = S3Client::new(rusoto_core::Region::UsEast2);
// const BUCKET: String = String::from("mycowbucket");

pub struct Aws {
    client: S3Client,
    bucket: String,
}

impl Aws {
    pub fn new(region: rusoto_core::Region) -> Self {
        let client = S3Client::new(region);
        let bucket = String::from("mycowbucket");
        Self {
            client,
            bucket,
        }
    }

    pub async fn ls(&self) -> Vec<rusoto_s3::Object>{


        let result = self.client.list_objects_v2(rusoto_s3::ListObjectsV2Request {
            bucket: self.bucket.clone(),
            continuation_token: None,
            delimiter: None,
            encoding_type: None,
            expected_bucket_owner: None,
            fetch_owner: None,
            max_keys: None, // Some(24)
            prefix: None,
            request_payer: None,
            start_after: None,
        }).await.unwrap();
        result.contents.unwrap()
        // for object in vector {
        //     println!("{}", object.key.unwrap());
        // }
    }
    
    pub async fn download(&self, path: String) -> rusoto_core::ByteStream {
        let result = self.client.get_object(rusoto_s3::GetObjectRequest {
            bucket: self.bucket.clone(),
            expected_bucket_owner: None,
            if_match: None,
            if_modified_since: None,
            if_none_match: None,
            if_unmodified_since: None,
            key: path,
            part_number: None, // do this later
            range: None,  // do this later
            request_payer: None,
            response_cache_control: None,
            response_content_disposition: None,
            response_content_encoding: None,
            response_content_language: None,
            response_content_type: None,
            response_expires: None,
            sse_customer_key: None,
            sse_customer_key_md5: None,
            sse_customer_algorithm: None,
            version_id: None,
        }).await.unwrap();
        return result.body.unwrap();
    }

}

