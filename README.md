# Fractalrs
Created this project so I can study Rust while programming fractals!
I have always wanted to learn fractal generation.

## Fractals
[Fractals](https://en.wikipedia.org/wiki/Fractal) are a very complex subject, there many ways to generate one and many different fractals with different characteristics.

### Sierpiński carpet
<img align="right" alt="GIF" height="150px" src="https://upload.wikimedia.org/wikipedia/commons/2/28/Animated_Sierpinski_carpet.gif" />

The [Sierpinski](https://en.wikipedia.org/wiki/Sierpi%C5%84ski_carpet) carpet is a very simple fractal. A fun thing about them is that when they are actually fractals (not just finite representations such as the one generated on this projetct) they have 0 area and infinite perimeter.

#### How to use it:
- Run `cargo run --release` inside the cloned repo.
- There are two possible variables that can be passed, otherwise the default values will be used:
    1. Steps -> The amount of steps that will be taken to create the image.
    2. Pattern -> I've added some nice patterns that the fractal can be generated with, try them out:
        1. Classic Contrast
        2. Inverted Classic
        3. Gold Velvet
        4. Earthy Tones
        5. Contrast Harmony
        6. Bold Neutrals

## Benchmark
Use the env BENCHMARK to run a test for speed on the generation.
``` bash
BENCHMARK cargo run --release
```
