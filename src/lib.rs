use chrono::{DateTime, Duration, TimeZone, Utc};
use csv::Reader;
use plotters::prelude::*;
use rustatistics::mean_and_variance;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Data {
    pub time: u64,
    pub high: f32,
    pub low: f32,
    pub open: f32,
    pub volumefrom: f32,
    pub volumeto: f32,
    pub close: f32,
    pub conversionType: String,
    pub conversionSymbol: Option<String>,
}

#[derive(Debug)]
pub struct BollingerBands {
    // time in Unix Epoch
    pub time: u64,
    // Upper Bollinger Band
    pub bolu: f32,
    // Lower Bollinger Band
    pub bold: f32,
    // Moving average over n days
    pub ma: f32,
    // number of days in smoothing period
    pub n: u32,
    // number of standard deviations (typically 2)
    pub m: f32,
    // Standard Deviation over last n periods of tp
    pub sd: f32,
}

impl BollingerBands {
    // Makes a vec of Bollinger Bands struct from an array of n elements, where
    // n is the window of the Bollinger Band. This also uses an array of
    // timestamps in Unix Epoch, taken from the last timestamp in the array.
    pub fn from_tp(data_array: &[f32], time_array: &[u64]) -> BollingerBands {
        let mut bands: BollingerBands = BollingerBands {
            n: data_array.len().try_into().unwrap(),
            time: *time_array.last().unwrap(),
            ..Default::default()
        };
        let sample_variation: f32;
        (bands.ma, _, sample_variation) = mean_and_variance(data_array).unwrap();
        bands.sd = sample_variation.sqrt();
        bands.bolu = bands.ma + bands.m * bands.sd;
        bands.bold = bands.ma - bands.m * bands.sd;
        bands
    }
}

// Defaults for Bollinger Bands.
// Currently uses n = 20, though this is not a factor in my use so far, since
// the function from_tp() above sets n to the size of the input array.
impl Default for BollingerBands {
    fn default() -> BollingerBands {
        BollingerBands {
            time: 0,
            bolu: 0.0,
            bold: 0.0,
            ma: 0.0,
            n: 20,
            m: 2.0,
            sd: 0.0,
        }
    }
}

// Takes an array of typical values and derives the bollinger band values.
// The current implementation calculates the mean and variance for each value,
// but this might change in the future as I add other trading indicators, which
// can draw from a pool of already calculated mean and variance values.
pub fn get_band_vec(tp_array: &[f32], time_array: &[u64]) -> Vec<BollingerBands> {
    let n = 20;
    if tp_array.len() < 20 {
        panic!("Your band is too small bro");
    }
    let mut bands_vec = vec![];
    for i in n..tp_array.len() {
        bands_vec.push(BollingerBands::from_tp(
            &tp_array[(i - 20)..=i],
            &time_array[(i - 20)..=i],
        ));
    }
    bands_vec
}

pub fn candle_chart_simple(data_filepath: &str, out_filepath: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr =
        Reader::from_path(data_filepath).expect("Could not load .csv file from data_filepath");
    let mut time_vec: Vec<u64> = vec![];
    let mut data_vec: Vec<Data> = vec![];
    for row in rdr.deserialize().zip((0..100).into_iter()) {
        let data: Data = row.0?;
        time_vec.push(data.time);
        data_vec.push(data);
    }

    let root = BitMapBackend::new(out_filepath, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let time_range: Vec<DateTime<Utc>> = time_vec
        .iter()
        .map(|t| Utc.timestamp((*t).try_into().unwrap(), 0))
        .collect();
    let (lower_time, upper_time) = (
        time_range[0] - Duration::minutes(2),
        *time_range.last().unwrap() + Duration::minutes(2),
    );
    let (upper_value, lower_value) = data_vec.iter().map(|data| (data.low, data.high)).fold(
        (f32::MAX, f32::MIN),
        |(old_low, old_high), (new_low, new_high)| (old_low.min(new_low), (old_high.max(new_high))),
    );

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .caption("BTC-USD Price", ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(lower_time..upper_time, lower_value..upper_value)?;

    chart.configure_mesh().light_line_style(&WHITE).draw()?;

    chart.draw_series(data_vec.iter().map(|x| {
        CandleStick::new(
            Utc.timestamp(x.time.try_into().unwrap(), 0),
            x.open,
            x.high,
            x.low,
            x.close,
            GREEN.filled(),
            RED,
            15,
        )
    }))?;
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", out_filepath);
    Ok(())
}
