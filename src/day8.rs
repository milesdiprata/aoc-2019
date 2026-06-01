use std::fmt::Write;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::bail;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Pixel {
    Black,
    White,
    Transparent,
}

#[derive(Debug)]
struct Image {
    layers: Vec<Vec<Pixel>>,
}

impl TryFrom<u8> for Pixel {
    type Error = Error;

    fn try_from(pixel: u8) -> Result<Self> {
        match pixel {
            0 => Ok(Self::Black),
            1 => Ok(Self::White),
            2 => Ok(Self::Transparent),
            _ => bail!("invalid pixel '{pixel}'"),
        }
    }
}

impl FromStr for Image {
    type Err = Error;

    fn from_str(data: &str) -> Result<Self> {
        let mut layers = Vec::new();
        for layer in data.as_bytes().chunks(WIDTH * HEIGHT) {
            layers.push(
                layer
                    .iter()
                    .map(|&byte| byte - b'0')
                    .map(Pixel::try_from)
                    .collect::<Result<_>>()?,
            );
        }

        Ok(Self { layers })
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::White => f.write_fmt(format_args!("{}", *self as u8)),
            Self::Black | Self::Transparent => f.write_char(' '),
        }
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut flat = [Pixel::Transparent; WIDTH * HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let mut top = Pixel::Transparent;

                for layer in &self.layers {
                    let pixel = layer[(y * WIDTH) + x];
                    match pixel {
                        Pixel::Black | Pixel::White => {
                            top = pixel;
                            break;
                        }
                        Pixel::Transparent => {}
                    }
                }

                flat[(y * WIDTH) + x] = top;
            }
        }

        for y in 0..HEIGHT {
            if y > 0 {
                f.write_char('\n')?;
            }

            for x in 0..WIDTH {
                f.write_fmt(format_args!("{}", flat[(y * WIDTH) + x]))?;
            }
        }

        Ok(())
    }
}

fn part1(image: &Image) -> usize {
    fn count_pixels(pixels: &[Pixel], target: Pixel) -> usize {
        #[allow(clippy::naive_bytecount)]
        pixels.iter().filter(|&&pixel| pixel == target).count()
    }

    let mut min_count = usize::MAX;
    let mut min_layer = 0;

    for (i, layer) in image.layers.iter().enumerate() {
        let black_pixels = count_pixels(layer, Pixel::Black);
        if black_pixels < min_count {
            min_count = black_pixels;
            min_layer = i;
        }
    }

    count_pixels(&image.layers[min_layer], Pixel::White)
        * count_pixels(&image.layers[min_layer], Pixel::Transparent)
}

fn main() -> Result<()> {
    let image = Image::from_str(&fs::read_to_string("in/day8.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&image);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 1_820);
    };

    {
        let start = Instant::now();
        let part2 = image.to_string();
        let elapsed = start.elapsed();

        println!("Part 2:\n{part2}\n({elapsed:?})");
    };

    Ok(())
}
