use std::{env, fs, thread};

mod shared_concurrency;
mod record;

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
    
    let output = shared_concurrency::get_output(&mmap, start_pos, end_pos);
    print!("{}", output);
}
