use crate::record::Record;
use std::{sync, thread};

pub fn get_output(mmap: &memmap2::Mmap, start_pos: Vec<usize>, end_pos: Vec<usize>) -> String {
    let mut stations_measures = ahash::AHashMap::<String, Record>::with_capacity(10_000);
    let (tx, rx) = sync::mpsc::channel();
    thread::scope(|s| {
        for (&start, &end) in start_pos.iter().zip(end_pos.iter()) {
            let tx1 = tx.clone();
            let buffer = &mmap;
            let mut handles = vec![];
            let handle = s.spawn(move || {
                let chunk_res = process_chunk(buffer, start, end);
                tx1.send(chunk_res)
            });
            handles.push(handle);
        }
    });
    drop(tx);
    for chunk_res in rx {
        combine_res(&mut stations_measures, chunk_res);
    }
    let output_vec = output_vec(&stations_measures);
    format!("{{{}}}", output_vec.join(", "))
}

fn combine_res(acc: &mut ahash::AHashMap<String, Record>, curr: ahash::AHashMap<String, Record>) {
    for (name, record) in curr {
        acc.entry(name)
            .and_modify(|rec| *rec += record)
            .or_insert(record);
    }
}

fn process_chunk(
    mmap: &memmap2::Mmap,
    mut start: usize,
    end: usize,
) -> ahash::AHashMap<String, Record> {
    // Guranteed that start - 1 is '\n' and end is '\n'
    let mut stations_measures = ahash::AHashMap::<String, Record>::with_capacity(10_000);
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
    stations_measures
}

fn output_vec(stations_measures: &ahash::AHashMap<String, Record>) -> Vec<String> {
    let mut keys: Vec<_> = stations_measures.keys().collect();
    keys.sort_unstable();
    let mut output_vec = Vec::with_capacity(keys.len());
    for name in keys {
        output_vec.push(format!("{}={}", name, stations_measures.get(name).unwrap()));
    }
    output_vec
}
