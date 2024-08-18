import math
import mmap
import dataclasses
from typing import Self
import concurrent.futures
num_threads = 12
@dataclasses.dataclass
class Record:
    low: int
    high: int
    total: int
    count: int = 1

    def __init__(self, val: int):
        self.low = val
        self.high = val
        self.total = val

    def __iadd__(self, other: Self):
        self.low = min(self.low, other.low)
        self.high = max(self.high, other.high)
        self.total += other.total
        self.count += other.count
        return self

    def _mean(self) -> float:
        mean = self.total / self.count
        return math.ceil(mean) / 10

    def __str__(self) -> str:
        return f"{self.low / 10:.1f}/{self._mean():.1f}/{self.high / 10:.1f}"

def main():
    file_path = "data/measurements.txt"
    with open(file=file_path, mode="rb") as file_obj:
        with mmap.mmap(fileno=file_obj.fileno(), length=0, access=mmap.ACCESS_READ) as mmap_obj:
            start_end_pos = get_start_and_end_pos(mmap_obj=mmap_obj)
            output = get_output(mmap_obj, start_end_pos)
            print(output)


def get_output(mmap_obj: mmap.mmap, start_end_pos: list[list[int]]):
    station_measures: dict[bytes, Record] = {}
    futures = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=num_threads) as exec:
        for args in start_end_pos:
            futures.append(exec.submit(process_chunk, mmap_obj, *args))
    for future in concurrent.futures.as_completed(futures):
        combine_res(station_measures, future.result())
    output = output_list(station_measures)
    return f"{{{', '.join(output)}}}"


def output_list(station_measures: dict[bytes, Record]) -> list[str]:
    stations = sorted(station_measures.keys())
    output = []
    for station in stations:
        output.append(f"{station.decode()}={str(station_measures[station])}")
    return output


def process_chunk(mmap_obj: mmap.mmap, start: int, end: int) -> dict[bytes, Record]:
    station_measures: dict[bytes, Record] = {}
    while start < end:
        name_end = start
        name_end = mmap_obj.find(b";", name_end)
        measure_end = mmap_obj.find(b"\n", name_end)
        name = mmap_obj[start: name_end]
        decimal_pos = mmap_obj.find(b".", name_end)
        decimal_val = mmap_obj[decimal_pos + 1] - ord(b"0")
        val = int(mmap_obj[name_end + 1: decimal_pos]) * 10
        val = val + decimal_val if val >= 0 else val - decimal_val
        record = Record(val)
        if name in station_measures:
            station_measures[name] += record
        else:
            station_measures[name] = record
        start = measure_end + 1
    return station_measures


def combine_res(acc: dict[bytes, Record], curr: dict[bytes, Record]):
    for name, record in curr.items():
        if name in acc:
            acc[name] += record
        else:
            acc[name] = record

def get_start_and_end_pos(mmap_obj: mmap.mmap) -> list[list[int]]:
    file_size = mmap_obj.size()
    chunk_size = file_size // num_threads
    start_end_pos = [[0, 0]]
    for i in range(num_threads - 1):
        s = chunk_size * (i + 1)
        s = mmap_obj.find(b"\n", s)
        start_end_pos[-1][1] = s
        start_end_pos.append([s + 1, s + 1])
    start_end_pos[-1][-1] = file_size - 1
    return start_end_pos

if __name__ == "__main__":
    main()
