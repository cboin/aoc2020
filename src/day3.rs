use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
enum FieldError {
    #[error("invalid character")]
    CharacterError,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Asset {
    Opensquare,
    Tree,
}

#[derive(Debug)]
struct Field {
    assets: Vec<Asset>,
}

#[derive(Debug)]
struct Map {
    fields: Vec<Field>,
}

impl Map {
    fn new(fields: Vec<Field>) -> Self {
        Map { fields }
    }
}

impl FromStr for Field {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let assets = s
            .chars()
            .map(|a| match a {
                '.' => Ok(Asset::Opensquare),
                '#' => Ok(Asset::Tree),
                _ => return Err(FieldError::CharacterError),
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { assets })
    }
}

fn count_trees(world: &Map, direction: (usize, usize)) -> usize {
    let width = world.fields[0].assets.len();

    world
        .fields
        .iter()
        .enumerate()
        .filter(|(y, _)| (y % direction.1) == 0)
        .map(|(y, x)| x.assets[(y * direction.0) % width])
        .filter(|x| *x == Asset::Tree)
        .count()
}

fn main() {
    let f = match File::open("inputs/day2_scotow") {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("{}", e),
    };

    let world = Map::new(
        f.lines()
            .filter_map(|l| l.ok())
            .filter_map(|f| f.parse().ok())
            .collect::<Vec<Field>>(),
    );

    println!("[part1] trees: {}", count_trees(&world, (1, 2)));

    let count = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|dir| count_trees(&world, *dir))
        .product::<usize>();

    println!("[part1] trees: {}", count);
}
