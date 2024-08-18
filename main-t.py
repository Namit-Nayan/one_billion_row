import math
import mmap
import concurrent.futures
import os
num_threads: int = os.process_cpu_count()


def main():
    file_path = "data/measurements_medium.txt"
    with open(file=file_path, mode="rb") as file_obj:
        with mmap.mmap(fileno=file_obj.fileno(), length=0, access=mmap.ACCESS_READ) as mmap_obj:
            start_end_pos = get_start_and_end_pos(mmap_obj=mmap_obj)
            output = get_output(mmap_obj, start_end_pos)
            print(output)


def get_output(mmap_obj: mmap.mmap, start_end_pos: list[list[int]]):
    station_measures: dict[bytes, list[int]] = {}
    futures = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=num_threads) as exec:
        for args in start_end_pos:
            futures.append(exec.submit(process_chunk, mmap_obj, *args))
    for future in concurrent.futures.as_completed(futures):
        combine_res(station_measures, future.result())
    output = output_list(station_measures)
    return f"{{{', '.join(output)}}}"


def output_list(station_measures: dict[bytes, list[int]]) -> list[str]:
    stations = sorted(station_measures.keys())
    output = []
    for station in stations:
        record = station_measures[station]
        record_mean = math.ceil(record[2] / record[3]) / 10
        output.append(f"{station.decode()}={record[0] / 10:.1f}/{record_mean:.1f}/{record[1] / 10:.1f}")
    return output


def combine_record(record1: list[int], record2: list[int]):
    # low, high, total, count
    if record1[0] > record2[0]:
        record1[0] = record2[0]
    if record1[1] < record2[1]:
        record1[1] = record2[1]
    record1[2] += record2[2]
    record1[3] += record2[3]


def process_chunk(mmap_obj: mmap.mmap, start: int, end: int) -> dict[bytes, list[int]]:
    station_measures: dict[bytes, list[int]] = {}
    while start < end:
        name_end = mmap_obj.find(b";", start)
        measure_end = mmap_obj.find(b"\n", name_end)
        name = mmap_obj[start: mmap_obj.find(b";", start)]
        val = int(float(mmap_obj[name_end + 1: measure_end]) * 10)
        record2 = [val, val, val, 1]
        record1 = station_measures.get(name)
        if record1:
            combine_record(record1, record2)
        else:
            station_measures[name] = record2
        start = measure_end + 1
    return station_measures


def combine_res(acc: dict[bytes, list[int]], curr: dict[bytes, list[int]]):
    for name, record2 in curr.items():
        try:
            combine_record(acc[name], record2)
        except KeyError:
            acc[name] = record2

def get_start_and_end_pos(mmap_obj: mmap.mmap) -> list[list[int]]:
    file_size = mmap_obj.size()
    chunk_size = file_size // num_threads
    start_end_pos = [[0, 0]]
    for i in range(1, num_threads):
        s = mmap_obj.find(b"\n", chunk_size * i)
        start_end_pos[-1][1] = s
        start_end_pos.append([s + 1, 0])
    start_end_pos[-1][-1] = file_size - 1
    return start_end_pos

if __name__ == "__main__":
    main()
