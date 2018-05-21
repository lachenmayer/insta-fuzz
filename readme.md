# insta-fuzz

- generated `fuzz/artifacts/fuzz_target_1/crash-f7526ab6f00b6bfb89dd41ee6bba512dfa916e16`
  - size: 32x32
  - metric: pixel average ∈ [95, 100]
- result: some random pixels in top left corner, rest of image is grey
- grey pixels probably caused by output length being too small
  - end of file, but width/height pre-specified
- going to use `cargo fuzz run fuzz_target_1 -- -max_len=65536` from now on to try get bigger output
- repeated runs give same result, even though seed is different every time
  - though not sure of this, copied the output artifact, re-ran & `shasum`
  - the hash at the end of the file name is actually the shasum
- started logging different error types (jpeg, format mismatch, metric mismatch) to see where it fails most often
