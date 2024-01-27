use std::{env, fs, thread};

mod message_concurrency;
mod record;
mod shared_concurrency;

fn main() -> Result<(), std::io::Error> {
    let flags = parse_args();
    let file = fs::File::open(flags.file_path.clone())?;
    let mmap = unsafe { memmap2::MmapOptions::new().map(&file)? };
    let (start_pos, end_pos) = get_start_end_pos(&mmap);
    let output = match flags.conc_type {
        ConcType::Message => message_concurrency::get_output(&mmap, start_pos, end_pos),
        ConcType::Shared => shared_concurrency::get_output(&mmap, start_pos, end_pos),
    };
    print!("{}", output);
    Ok(())
}

enum ConcType {
    Shared,
    Message,
}

struct Flags {
    conc_type: ConcType,
    file_path: String,
}

fn parse_args() -> Flags {
    let arg_map = env::args().skip(1).collect::<Vec<String>>();
    let mut flags = Flags {
        conc_type: ConcType::Message,
        file_path: "data/measurements.txt".to_owned(),
    };
    for arg in arg_map {
        if let Some((key, value)) = arg.split_once(':') {
            match key {
                "conc_type" if value == "shared" => flags.conc_type = ConcType::Shared,
                "file_path" => flags.file_path = value.to_owned(),
                _ => {}
            }
        }
    }
    flags
}

fn get_start_end_pos(mmap: &memmap2::Mmap) -> (Vec<usize>, Vec<usize>) {
    let num_threads = thread::available_parallelism().unwrap().get();
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
    (start_pos, end_pos)
}
