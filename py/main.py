import time

start_time = time.time()

for _ in range(1000):
    sum_of_squares = sum(x * x for x in range(1, 1_000_001))

elapsed_time = time.time() - start_time
print(f"Total time for 1,000 runs: {elapsed_time} seconds")
