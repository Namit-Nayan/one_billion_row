use std::{
    env, fs,
    io::{self, Write},
};

use rand::Rng;
fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    let rows: usize = args[0].split('_').collect::<String>().parse().unwrap();
    let path = if args.len() == 2 {
        args[1].clone()
    } else {
        "data/measurements.txt".to_string()
    };
    let weather_file_map = unsafe {
        memmap2::MmapOptions::new()
            .map(&fs::File::open("data/weather_stations.csv").unwrap())
            .unwrap()
    };
    let file = fs::File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();
    let temp_dist = rand::distributions::Uniform::new(0, 2000);
    let stations = get_station_list(weather_file_map, rows.min(10_000));
    let station_dist = rand::distributions::Uniform::new(0, stations.len());
    let mut rng = rand::thread_rng();
    let mut writer = io::BufWriter::new(file);
    for _ in 0..rows {
        let (station_id, temp_id) = (rng.sample(station_dist), rng.sample(temp_dist));
        let temp = ((temp_id - 999) as f32) / 10.0;
        for buf in [stations[station_id].as_slice(), &[b';'], temp.to_string().as_bytes(), &[b'\n']] {
            let _ = writer.write(buf);
        }
    }
}

fn get_station_list(mmap: memmap2::Mmap, cnt: usize) -> Vec<Vec<u8>> {
    let mut stations = Vec::<Vec<u8>>::with_capacity(cnt);
    let mut pos = 0;
    while stations.len() < cnt {
        let mut last_new_line = pos;
        while mmap[pos] != b';' {
            if mmap[pos] == b'\n' {
                last_new_line = pos;
            }
            pos += 1;
        }
        stations.push(mmap[(last_new_line + 1)..pos].to_vec());
        pos += 1;
    }
    stations.into_iter().collect()
}
