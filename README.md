Concurrent implementation of One Billion Row challenge in rust -> https://www.morling.dev/blog/one-billion-row-challenge/
Github: https://github.com/gunnarmorling/1brc

Support command line arguments:
1. file_path:/path/to/measurement.txt
   To specify your source file.
   Defaults to "data/measurements.txt".
   e.g. cargo run -r file_path:data/measurements.txt
3. conc_type:shared
   To use Shared-State concurrency.
   Defaults to Communicating by message passing.
   e.g. cargo run -r conc_type:shared
