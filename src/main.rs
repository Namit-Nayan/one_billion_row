use std::{env, fmt, fs, ops, sync, thread};

type MapType = sync::Arc<dashmap::DashMap<String, Record, ahash::RandomState>>;

fn main() {
    let num_threads = thread::available_parallelism().unwrap().get();
    let file_path = match env::args().nth(1) {
        Some(f) => f,
        None => "data/measurements.txt".to_owned(),
    };
    let file = fs::File::open(file_path.clone()).expect("Measurement file not found in path.");
    let mmap = unsafe { memmap2::MmapOptions::new().map(&file).unwrap() };
    let file_size = mmap.len();
    let chunk_size = file_size / num_threads;
    let stations_measures: MapType = sync::Arc::new(
        dashmap::DashMap::with_capacity_and_hasher(10_000, ahash::RandomState::new()),
    );
    let mut start_pos = vec![0; num_threads];
    let mut end_pos = vec![0; num_threads];
    end_pos[num_threads - 1] = file_size - 1;
    for (i, pos) in start_pos.iter_mut().skip(1).enumerate() {
        let mut s = chunk_size * (i + 1);
        while mmap[s] != b'\n' {
            s += 1;
        }
        *pos = s + 1;
        end_pos[i] = s;
    }
    thread::scope(|s| {
        for (&start, &end) in start_pos.iter().zip(end_pos.iter()) {
            let buffer = &mmap;
            let stations_measures = stations_measures.clone();
            s.spawn(move || {
                process_chunk(buffer, start, end, stations_measures);
            });
        }
    });
    let output_vec = output(stations_measures.clone());
    print!("{{{}}}", output_vec.join(", "));
}

fn process_chunk(mmap: &memmap2::Mmap, mut start: usize, end: usize, stations_measures: MapType) {
    // Guranteed that start - 1 is '\n' and end is '\n'
    while start < end {
        let mut name_end = start;
        while mmap[name_end] != b';' {
            name_end += 1;
        }
        let mut measure_end = name_end + 1;
        while mmap[measure_end] != b'\n' {
            measure_end += 1;
        }
        let (name, measure) = (
            std::str::from_utf8(&mmap[start..name_end]).unwrap(),
            std::str::from_utf8(&mmap[name_end + 1..measure_end])
                .unwrap()
                .parse()
                .unwrap(),
        );
        let record = Record::new(measure);
        stations_measures
            .entry(name.to_owned())
            .and_modify(|rec| *rec += record)
            .or_insert(record);
        start = measure_end + 1;
    }
}

fn output(stations_measures: MapType) -> Vec<String> {
    let mut keys = Vec::<String>::with_capacity(stations_measures.len());
    for x in stations_measures.iter() {
        keys.push(x.key().clone())
    }
    keys.sort_unstable();
    let mut output_vec = Vec::with_capacity(keys.len());
    for name in keys {
        output_vec.push(format!(
            "{}={}",
            name,
            *stations_measures.get(&name).unwrap()
        ));
    }
    output_vec
}

#[derive(Clone, Copy, Debug)]
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
