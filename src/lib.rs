use rustatistics::mean_and_variance;
use serde::{Deserialize, Serialize};

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
    pub bolu: f64,
    // Lower Bollinger Band
    pub bold: f64,
    // Moving average over n days
    pub ma: f64,
    // number of days in smoothing period
    pub n: u32,
    // number of standard deviations (typically 2)
    pub m: f64,
    // Standard Deviation over last n periods of tp
    pub sd: f64,
}

impl BollingerBands {
    // Makes a vec of Bollinger Bands struct from an array of n elements, where
    // n is the window of the Bollinger Band. This also uses an array of
    // timestamps in Unix Epoch, taken from the last timestamp in the array.
    pub fn from_tp(data_array: &[f64], time_array: &[u64]) -> BollingerBands {
        let mut bands: BollingerBands = BollingerBands {
            n: data_array.len().try_into().unwrap(),
            time: *time_array.last().unwrap(),
            ..Default::default()
        };
        let sample_variation: f64;
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
pub fn get_band_vec(tp_array: &[f64], time_array: &[u64]) -> Vec<BollingerBands> {
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
