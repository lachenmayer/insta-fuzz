# insta-fuzz

## reading

JFIF spec https://www.w3.org/Graphics/JPEG/jfif3.pdf

## experiment 1: generate a 32x32 image

aim: generate any kind of image using `cargo fuzz`

- metric: pixel average ∈ [95, 100
- corpus input: `fuzz/corpus/_1.jpg`
  - jpeg image scaled down to 32x32 using imagemagick `convert`
- generated `fuzz/artifacts/fuzz_target_1/crash-f7526ab6f00b6bfb89dd41ee6bba512dfa916e16`
  - size: 32x32
- result: some random pixels in top left corner, rest of image is grey
- grey pixels probably caused by output length being too small
  - end of file, but width/height pre-specified
- going to use `cargo fuzz run fuzz_target_1 -- -max_len=65536` from now on to try get bigger output
- repeated runs give same result, even though seed is different every time
  - though not sure of this, copied the output artifact, re-ran & `shasum`
  - the hash at the end of the file name is actually the shasum
- started logging different error types (jpeg `J`, format mismatch `F`, metric mismatch `M`) to see where it fails most often
- `cov: 38265` on success

## experiment 2: generate a 64x64 image

aim: generate a bigger image

- tried to change required size to 64x64
- result: stuck at format (no `M` errors, only `J`/`F`) - finding the exact value seems pretty hard
  - `cov` at ~13000
  - thought: would be amazing to see the coverage visualized
- gave up
- generated new 64x64 image `fuzz/corpus/_2.jpg` (from same source image as in #1)
- observed `M`, `cov` to ~38000
- successful run: generated `fuzz/artifacts/fuzz_target_1/crash-ffe8cb1c1efa03c503fd9b81617e1c380b6be70b`
  - start time: `10:46:13`
  - end time: `10:51:13`
  - size: 32x32
  - metric: pixel average ∈ [95, 100]
  - `cov: 38599` at last fail
  - file size: `3296`
- result:
  - top 16 rows of pixels recognisably from source image, next 8px row pink/red noise
  - last block of that row does not have color
  - rest of file is grey
- started logging resulting average value
- repeated run generates a new image `crash-d96bcc5af20d9378d78c6df574d87ed37e41d458`
  - `11:06:02` - `11:07:54`
  - only first row of blocks recognisable
  - file size: 2118 (67%)
  - average value: `91.954833984375`
- third run: `crash-c244bb058e1531e4d47175e26ed20f68031d4e1a`
  - `11:08:00` - `11:10:16`
  - two and a half rows recognisable, brown noise (similar to t shirt colors in image)
  - file size `4467`
  - `cov: 38650`

## experiment 3: more red pixels than others

aim: modify the success metric to generate more visually interesting output