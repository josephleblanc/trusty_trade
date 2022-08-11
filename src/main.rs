use chrono::{DateTime, Offset, TimeZone, Utc};
use csv::Reader;
use plotters::prelude::*;
use std::error::Error;
use trusty_trade::{get_band_vec, Data};

const OUT_FILE_NAME: &'static str = "plotters-doc-data/stock.png";

fn main() -> Result<(), Box<dyn Error>> {
    let data_filepath = "/home/brasides/programming/data/BTC_historic_minute/master/2022-07-21_to_2022-08-09_16:22:00.csv";
    let mut rdr =
        Reader::from_path(data_filepath).expect("Could not load .csv file from data_filepath");
    let mut time_vec: Vec<u64> = vec![];
    let mut tp_vec: Vec<f32> = vec![];
    let mut data_vec: Vec<Data> = vec![];
    for row in rdr.deserialize().zip((0..100).into_iter()) {
        let data: Data = row.0?;
        let tp = (data.low + data.high + data.close) / 3.0;
        tp_vec.push(tp);
        time_vec.push(data.time);
        data_vec.push(data);
    }

    let bands_vec = get_band_vec(&tp_vec, &time_vec);
    //for band in bands_vec {
    //    println!("{:?}", band);
    //}

    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let time_range: Vec<DateTime<Utc>> = time_vec
        .iter()
        .step_by(5)
        .map(|t| Utc.timestamp((*t).try_into().unwrap(), 0))
        .collect();
    let (lower_time, upper_time) = (time_range[0], time_range.last().unwrap());
    let (upper_value, lower_value) = data_vec.iter().map(|data| (data.low, data.high)).fold(
        (f32::MAX, f32::MIN),
        |(old_low, old_high), (new_low, new_high)| (old_low.min(new_low), (old_high.max(new_high))),
    );
    //let (lower_bound, upper_bound) = bands_vec.iter().map(|band| (band.bold, band.bolu)).fold(
    //    (f32::MIN, f32::MAX),
    //    |(old_bold, old_bolu), (new_bold, new_bolu)| {
    //        (old_bold.min(new_bold), (old_bolu.max(new_bolu)))
    //    },
    //);

    //let value_range = (lower_bound.floor() as u64)..(upper_bound.ceil() as u64);

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("BTC-USD Price", ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(lower_time..*upper_time, lower_value..upper_value)?;

    chart.configure_mesh().light_line_style(&WHITE).draw()?;

    chart.draw_series(data_vec.iter().map(|x| {
        CandleStick::new(
            parse_time(x.time),
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
    println!("Result has been saved to {}", OUT_FILE_NAME);
    println!("lower_value: {}, upper_value: {}", lower_value, upper_value);
    Ok(())
}

fn parse_time(t: u64) -> DateTime<Utc> {
    Utc.timestamp(t.try_into().unwrap(), 0)
}
