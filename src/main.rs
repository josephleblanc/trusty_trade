use csv::Reader;
use std::error::Error;
use trusty_trade::{get_band_vec, Data};

fn main() -> Result<(), Box<dyn Error>> {
    let data_filepath = "/home/brasides/programming/data/BTC_historic_minute/master/2022-07-21_to_2022-08-09_16:22:00.csv";
    let mut rdr =
        Reader::from_path(data_filepath).expect("Could not load .csv file from data_filepath");
    let mut time_vec: Vec<u64> = vec![];
    let mut tp_vec: Vec<f64> = vec![];
    for row in rdr.deserialize().zip((0..100).into_iter()) {
        let data: Data = row.0?;
        let tp = (data.low + data.high + data.close) / 3.0;
        tp_vec.push(tp.into());
        time_vec.push(data.time);
    }

    let bands_vec = get_band_vec(&tp_vec, &time_vec);
    for band in bands_vec {
        println!("{:?}", band);
    }

    Ok(())
}
