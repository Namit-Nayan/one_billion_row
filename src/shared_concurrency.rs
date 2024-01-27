use std::{sync, thread};
use crate::record::Record;

type MapType = sync::Arc<dashmap::DashMap<String, Record, ahash::RandomState>>;

pub fn get_output(mmap: &memmap2::Mmap, start_pos: Vec<usize>, end_pos: Vec<usize>) -> String {
    let stations_measures: MapType = sync::Arc::new(
        dashmap::DashMap::with_capacity_and_hasher(10_000, ahash::RandomState::new()),
    );
    thread::scope(|s| {
        for (&start, &end) in start_pos.iter().zip(end_pos.iter()) {
            let buffer = &mmap;
            let stations_measures = stations_measures.clone();
            s.spawn(move || {
                process_chunk(buffer, start, end, stations_measures);
            });
        }
    });
    let output_vec = output_vec(stations_measures.clone());
    format!("{{{}}}", output_vec.join(", "))
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

fn output_vec(stations_measures: MapType) -> String {
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
