Concurrent implementation of [One Billion Row challenge](https://www.morling.dev/blog/one-billion-row-challenge/) in [Rust](https://www.rust-lang.org/).

[Github Link](https://github.com/gunnarmorling/1brc)

Support command line arguments:
1. file_path:/path/to/measurement.txt
   To specify your source file.
   Defaults to "data/measurements.txt".
   e.g. 
   
   ```cargo run -r file_path:data/measurements.txt```

3. conc_type:shared
   To use Shared-State concurrency.
   Defaults to Communicating by message passing.
   e.g. 
   
   ```cargo run -r conc_type:shared```

Runs under a minute on "Intel® Core™ i5-10210U CPU @ 1.60GHz × 8"

Memory usage under 1 MB for shared-state, under 10 MB for message passing.
