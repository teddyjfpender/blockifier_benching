use std::fs::File;
use std::io::{BufReader, Write};
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_directory = "target/criterion/transfers/";
    //"target/criterion/transfers/transfers_n_workers_1_chunk_size_100_concurrency/new/"
    let files_to_read = vec![
        "transfers_n_workers_1_chunk_size_50_concurrency",
        "transfers_n_workers_1_chunk_size_100_concurrency",
        "transfers_n_workers_1_chunk_size_200_concurrency",
        "transfers_n_workers_1_chunk_size_400_concurrency",
        "transfers_n_workers_2_chunk_size_50_concurrency",
        "transfers_n_workers_2_chunk_size_100_concurrency",
        "transfers_n_workers_2_chunk_size_200_concurrency",
        "transfers_n_workers_2_chunk_size_400_concurrency",
        "transfers_n_workers_4_chunk_size_50_concurrency",
        "transfers_n_workers_4_chunk_size_100_concurrency",
        "transfers_n_workers_4_chunk_size_200_concurrency",
        "transfers_n_workers_4_chunk_size_400_concurrency",
        "transfers_n_workers_8_chunk_size_50_concurrency",
        "transfers_n_workers_8_chunk_size_100_concurrency",
        "transfers_n_workers_8_chunk_size_200_concurrency",
        "transfers_n_workers_8_chunk_size_400_concurrency",
        "transfers_n_workers_16_chunk_size_50_concurrency",
        "transfers_n_workers_16_chunk_size_100_concurrency",
        "transfers_n_workers_16_chunk_size_200_concurrency",
        "transfers_n_workers_16_chunk_size_400_concurrency",
        "transfers_n_workers_32_chunk_size_50_concurrency",
        "transfers_n_workers_32_chunk_size_100_concurrency",
        "transfers_n_workers_32_chunk_size_200_concurrency",
        "transfers_n_workers_32_chunk_size_400_concurrency",
    ];
    // print the current working directory
    let current_dir = std::env::current_dir()?;

    // data collection struct
    #[derive(serde::Serialize, serde::Deserialize)]
    struct Data {
        n_workers: i32,
        chunk_size: i32,
        mean: f64,
        std_err: f64,
    }

    let mut data_collection: Vec<Data> = Vec::new();
    
    for f in files_to_read {
        let file_path = format!("{}/{}{}/new/estimates.json", current_dir.to_str().unwrap(), file_directory, f);
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        // Parse the JSON contents of the file as an instance of `serde_json::Value`.
        let data: Value = serde_json::from_reader(reader)?;

        // Access parts of the data
        let mean = &data["mean"]["point_estimate"].as_f64().unwrap();
        let std_err = &data["mean"]["standard_error"].as_f64().unwrap();
        // get the number of workers and chunk size, file is a &str 
        let n_workers = f.split("_").collect::<Vec<&str>>()[3];
        let chunk_size = f.split("_").collect::<Vec<&str>>()[6];

        let data = Data {
            n_workers: n_workers.parse::<i32>().unwrap(),
            chunk_size: chunk_size.parse::<i32>().unwrap(),
            mean: *mean,
            std_err: *std_err,
        };

        // push the data to the data collection
        data_collection.push(data);

        println!("Workers: {}, Chunk Size: {}, Time to Execute 1000 transfers -- Mean: {}ms (Standard Error: {}ms)", n_workers, chunk_size, mean / 1e6, std_err / 1e6);

        // save the data to a json file
        let data_json = serde_json::to_string(&data_collection)?;
        let file_path = format!("./data.json");
        let mut file = File::create(file_path)?;
        file.write_all(data_json.as_bytes())?;
    }

    Ok(())
}