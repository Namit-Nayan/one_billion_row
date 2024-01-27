use std::{
    collections, env, fmt, fs,
    io::{self, BufRead},
    ops,
};

fn main() {
    let file_path = match env::args().nth(1) {
        Some(f) => f,
        None => "data/measurements.txt".to_owned(),
    };
    let file = fs::File::open(file_path.clone())
        .unwrap_or_else(|_| panic!("Measurement file not found in path {:?}", file_path));
    let mut reader = io::BufReader::new(file);
    let mut stations_pos = collections::HashMap::<String, usize>::with_capacity(10_000);
    let mut measures: Vec<Record> = Vec::with_capacity(10_000);
    let mut line_buff = String::with_capacity(110);
    while reader.read_line(&mut line_buff).unwrap() > 0 {
        let (name, measure) = line_buff.split_once(';').unwrap();
        let measure: f32 = measure.trim().parse().unwrap();
        if let Some(&id) = stations_pos.get(name) {
            measures[id] += Record::new(id, measure);
        } else {
            stations_pos.insert(name.to_owned(), measures.len());
            measures.push(Record::new(measures.len(), measure));
        }
        line_buff.clear();
    }
    let mut id_name = vec![""; measures.len()];
    stations_pos.iter().for_each(|(s, i)| id_name[*i] = s);
    measures.sort_unstable_by_key(|r| id_name[r.id]);
    print!("{{");
    for (i, r) in measures.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}={r}", id_name[r.id]);
    }
    print!("}}");
}

struct Record {
    id: usize,
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
            id: self.id,
            low: self.low.min(rhs.low),
            high: self.high.max(rhs.high),
            sum: self.sum + rhs.sum,
            count: self.count + rhs.count,
        }
    }
}

impl Record {
    fn new(id: usize, val: f32) -> Self {
        Self {
            id,
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
