use csv::Reader;
use std::error::Error;
use trusty_trade::{candle_chart_simple, get_band_vec, Data};

fn main() -> Result<(), Box<dyn Error>> {
    let data_filepath = "/home/brasides/programming/data/BTC_historic_minute/master/2022-07-21_to_2022-08-09_16:22:00.csv";
    let out_filepath = "plotters-doc-data/stock.png";
    candle_chart_simple(data_filepath, out_filepath)?;
    Ok(())
}
