use regex::Regex;
use std::collections::HashMap;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::model::{
    CompressionType, CsvInput, CsvOutput, ExpressionType, FileHeaderInfo, InputSerialization,
    OutputSerialization, SelectObjectContentEventStream,
};
use aws_sdk_s3::{Client, Error, Region, PKG_VERSION};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// The name of the bucket containing the object (CSV file).
    #[structopt(short, long)]
    bucket: String,

    /// The object key to scan. This example expects the object to be an uncompressed CSV file with:

    /// Name,PhoneNumber,City,Occupation
    /// Person1,(nnn) nnn-nnnn,City1,Occupation1
    /// ...
    /// PersonN,(nnn) nnn-nnnn,CityN,OccupationN
    #[structopt(short, long)]
    object: String,

    /// The name of the person to scan for.
    #[structopt(short, long)]
    name: String,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// Get object content.
// snippet-start:[s3.rust.select-object-content]
async fn get_content(client: &Client, bucket: &str, object: &str, name: &str) -> Result<(), Error> {
    let mut person: String = "SELECT * FROM s3object s WHERE s.\"Name\" = '".to_owned();
    person.push_str(name);
    person.push('\'');

    let mut output = client
        .select_object_content()
        .bucket(bucket)
        .key(object)
        .expression_type(ExpressionType::Sql)
        .expression(person)
        .input_serialization(
            InputSerialization::builder()
                .csv(
                    CsvInput::builder()
                        .file_header_info(FileHeaderInfo::Use)
                        .build(),
                )
                .compression_type(CompressionType::None)
                .build(),
        )
        .output_serialization(
            OutputSerialization::builder()
                .csv(CsvOutput::builder().build())
                .build(),
        )
        .send()
        .await?;

    while let Some(event) = output.payload.recv().await? {
        match event {
            SelectObjectContentEventStream::Records(records) => {
                println!(
                    "Record: {}",
                    records
                        .payload()
                        .map(|p| std::str::from_utf8(p.as_ref()).unwrap())
                        .unwrap_or("")
                );
            }
            SelectObjectContentEventStream::Stats(stats) => {
                println!("Stats: {:?}", stats.details().unwrap());
            }
            SelectObjectContentEventStream::Progress(progress) => {
                println!("Progress: {:?}", progress.details().unwrap());
            }
            SelectObjectContentEventStream::Cont(_) => {
                println!("Continuation Event");
            }
            SelectObjectContentEventStream::End(_) => {
                println!("End Event");
            }
            otherwise => panic!("Unknown event type: {:?}", otherwise),
        }
    }

    Ok(())
}

pub async fn get_main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let bucket = "bigdatareviews";
    let object = "data.csv";
    let name = "stmt1";
    let verbose = true;
    //let region="us-east-1";
    /*let region_provider = RegionProviderChain::first_try(region.map(Region::new))
    .or_default_provider()
    .or_else(Region::new("us-east-1"));*/
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    println!();

    if verbose {
        println!("S3 client version: {}", PKG_VERSION);
        println!("Region:            {}", shared_config.region().unwrap());
        println!("Bucket:            {}", &bucket);
        println!("Object:            {}", &object);
        println!("Name:              {}", &name);

        println!();
    }

    get_content(&client, &bucket, &object, &name).await
}
pub fn tokenize(text: &str) -> HashMap<String, usize> {
    let re = Regex::new(r"[^\p{L}]+").unwrap(); // matches any non-letter character
                                                //re.split(text).collect()
    let mut bag_of_words = HashMap::new();
    for token in re.split(text) {
        if !token.is_empty() {
            *bag_of_words.entry(token.to_lowercase()).or_insert(0) += 1;
        }
    }
    return bag_of_words;
}
