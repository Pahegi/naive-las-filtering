use std::io::Write as _;
use las::{Read, Reader, Writer, Write, Builder};
use las::point::{Classification, Format, ScanDirection};
use laz::las::point6::Point6;

fn main() {

    // measure start time
    let start = std::time::Instant::now();

    // create reader
    let mut reader = Reader::from_path("data/frankfurt.las").unwrap();

    // create writer
    let mut builder = Builder::from((1, 4));
    builder.point_format = Format::new(3).unwrap();
    let header = builder.into_header().unwrap();
    let mut writer = Writer::from_path("data/filtered.las", header).unwrap();
    let num_points = reader.header().number_of_points() as usize;
    println!("Number of points: {}", num_points);

    // iterate over points and write them to the new file
    let mut i = 0;
    for point in reader.points() {
        // print percentage
        if i % (num_points/100) == 0 {
            print_fancy_progress_bar((i * 100 / num_points) as u8);
        }
        i += 1;

        // read point, check filter and write point to new file
        let point = point.unwrap();
        if point.classification == Classification::new(11).unwrap() {
            writer.write(point).unwrap();
        }
    }
    writer.close().unwrap();

    // measure end time
    let end = std::time::Instant::now();

    // print results
    println!("\nTime elapsed: {:?}", end.duration_since(start));
    println!("Number of filtered points: {:?}", writer.header().number_of_points());

}

fn print_fancy_progress_bar(percent: u8) {
    let bar_length = 100;
    let num_bars = (percent as f32 / 100f32 * bar_length as f32) as usize;
    let mut bar = String::from("[");
    for _ in 0..num_bars {
        bar.push('=');
    }
    for _ in num_bars..bar_length {
        bar.push(' ');
    }
    bar.push_str("]");
    print!("\r{}% {}", percent, bar);
    std::io::stdout().flush().unwrap();
}
