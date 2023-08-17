# Cache Benchmark

This program aims to impliment various cache eviction techniques and write policies. There will be tests in order to test the best performing technique for various testcases.

> [!WARNING]
> The project will undergo drastic changes in it's design, structure and featues as I'm still learning rust.

## Design
### `storage` module
- Abstracts the working of cache and disk storage
- Impliments various write policies

### `storage/cache` module 
-   Impliments various eviction techniques

### `storage/disk` module 
-   Mimics simple disk read/write

## Cache Eviction Techniques
1. LRU
1. MRU
1. LFU

## Cache Write Policies
1. Write Back
1. Write Through
1. Write Around?

## Results
WIP
