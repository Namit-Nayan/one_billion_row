use std::{
    collections, env, fmt, fs, io::{self, BufRead, Seek}, ops, sync, thread
};

fn main() {
    let num_threads = thread::available_parallelism().unwrap().get() as u64;
    let file_path = match env::args().nth(1) {
        Some(f) => f,
        None => "data/measurements.txt".to_owned(),
    };
    let file = fs::File::open(file_path.clone())
            .expect("Measurement file not found in path.");
    let file_size = file.metadata().unwrap().len();
    let chunk_size = file_size / num_threads;
    let mut stations_measures = collections::HashMap::<String, Record>::with_capacity(10_000);
    let mut handles = vec![];
    let (tx, rx) = sync::mpsc::channel();
    let mut start = 0;
    let mut buf = String::with_capacity(200);
    while start < file_size {
        let file = fs::File::open(file_path.clone())
            .expect("Measurement file not found in path.");
        let mut reader = io::BufReader::new(file);
        let mut end = file_size.min(start + chunk_size);
        let _ = reader.seek(io::SeekFrom::Start(end));
        let _ = reader.read_line(&mut buf);
        end = reader.stream_position().unwrap();
        buf.clear();
        let tx1 = tx.clone();
        let handle = thread::spawn(move || {
            let chunk_res = process_chunk(reader, start, end);
            tx1.send(chunk_res)
        });
        handles.push(handle);
        start = end + 1;
    }
    for handle in handles {
        let _ = handle.join().unwrap();
    }
    drop(tx);
    for chunk_res in rx {
        combine_res(&mut stations_measures, chunk_res)
    }
    let output_vec = output(&stations_measures);
    print!("{{{}}}", output_vec.join(", "));
}

fn combine_res(acc: &mut collections::HashMap<String, Record>, curr: collections::HashMap<String, Record>) {
    for (name, record) in curr {
        acc.entry(name).and_modify(|rec| *rec += record).or_insert(record);
    }
}

fn process_chunk(mut reader: io::BufReader<fs::File>, start: u64, end: u64) -> collections::HashMap<String, Record> {
    // Guranteed that start - 1 is '\n' and end is '\n'
    let mut stations_measures = collections::HashMap::<String, Record>::with_capacity(10_000);
    let _ = reader.seek(io::SeekFrom::Start(start));
    let mut buf = String::with_capacity(120);
    while reader.stream_position().unwrap() < end {
        let _ = reader.read_line(&mut buf);
        let (name, measure) = buf.split_once(';').unwrap();
        let measure: f32 = measure.trim().parse().unwrap();
        let record = Record::new(measure);
        stations_measures.entry(name.to_owned()).and_modify(|rec| *rec += record).or_insert(record);
        buf.clear();
    }
    stations_measures
}

fn output(stations_measures: &collections::HashMap<String, Record>) -> Vec<String> {
    let mut keys: Vec<_> = stations_measures.keys().collect();
    keys.sort_unstable();
    let mut output_vec = Vec::with_capacity(keys.len());
    for  name in keys {
        output_vec.push(format!("{}={}", name, stations_measures.get(name).unwrap()));
    }
    output_vec
}

#[derive(Clone, Copy)]
struct Record {
    low: f32,
    high: f32,
    sum: f64,
    count: u32,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}/{:.1}/{:.1}", self.low, self.mean(), self.high)
    }
}

impl ops::AddAssign for Record {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            low: self.low.min(rhs.low),
            high: self.high.max(rhs.high),
            sum: self.sum + rhs.sum,
            count: self.count + rhs.count,
        }
    }
}

impl Record {
    fn new(val: f32) -> Self {
        Self {
            low: val,
            high: val,
            sum: val as f64,
            count: 1,
        }
    }

    fn mean(&self) -> f32 {
        (((self.sum) / (self.count as f64) * 10.0).ceil() / 10.0) as f32
    }
}
