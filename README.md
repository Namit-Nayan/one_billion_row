Concurrent implementation of [One Billion Row challenge](https://www.morling.dev/blog/one-billion-row-challenge/) in [Rust](https://www.rust-lang.org/).

[Github Link](https://github.com/gunnarmorling/1brc)

Support command line arguments:
1. To generate data
   
   ```cargo run -r --bin generate_data 1_000_000_000 data/measurements.txt```

   where `` 1_000_000_000_`` number of rows, and ``data/measurements.txt`` is CSV file path to weather station names.

   File should be in this format -> https://github.com/gunnarmorling/1brc/blob/main/data/weather_stations.csv

3. ``file_path:/path/to/measurement.txt``
   To specify your source file.
   Defaults to "data/measurements.txt".
   e.g. 
   
   ```cargo run -r --bin one_billion_row file_path:data/measurements.txt```

4. ``conc_type:shared``
   To use Shared-State concurrency.
   Defaults to Communicating by message passing.
   e.g. 
   
   ```cargo run -r --bin one_billion_row conc_type:shared```

Runs under a minute on "Intel® Core™ i5-10210U CPU @ 1.60GHz × 8"

Memory usage under 1 MB for shared-state, under 10 MB for message passing.
