use std::collections::HashMap;
use rayon::prelude::*;

pub fn run(data: String) {
    println!("Part 1 : {:?}", part1(&data));
    println!("Part 2 : {:?}", part2(&data));
}


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum TileType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start // Cross
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn get_possible_tile_from_dir(&self) -> Vec<TileType> {
        use TileType::*;
        match self {
            Direction::North => vec![Vertical, SouthWest, SouthEast, Start],
            Direction::East => vec![Horizontal, SouthWest, NorthWest, Start],
            Direction::South => vec![Vertical, NorthWest, NorthEast, Start],
            Direction::West => vec![Horizontal, NorthEast, SouthEast, Start]
        }
    }
}

impl TileType {
    fn from_char(c: char) -> TileType {
        use TileType::*;
        match c {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            '.' => Ground,
            'S' => Start,
            _ => {panic!("Should not happen")}
        }
    }

    fn direction_from_tile(&self) -> Vec<Direction> {
        use Direction::*;
        match self {
            TileType::Vertical => vec![North, South],
            TileType::Horizontal => vec![East, West],
            TileType::NorthEast => vec![North, East],
            TileType::NorthWest => vec![North, West],
            TileType::SouthWest => vec![South, West],
            TileType::SouthEast => vec![South, East],
            TileType::Ground => vec![],
            TileType::Start => vec![North, East, South, West]
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Tile {
    x: i32,
    y: i32,
    tile: TileType
}

type Maze = HashMap<(i32, i32), Tile>;

fn parse_map(input: &str) -> Maze {
    let map = input.split('\n').enumerate().flat_map(|(y, r)| {
        r.chars().enumerate().map(|(x, t)| {
            Tile {
                x: x as i32,
                y: y as i32,
                tile: TileType::from_char(t)
            }
        }).collect::<Vec<Tile>>()
    }).collect::<Vec<Tile>>();
    let mut hashmap: Maze = HashMap::new();
    map.iter().for_each(|t| {
        hashmap.insert((t.x, t.y), *t);
    });
    hashmap
}

fn get_tile_from_dir<'a>(current: &Tile, maze: &'a Maze, direction: &Direction) -> Option<&'a Tile> {
    match direction {
        Direction::North => maze.get(&(current.x, current.y - 1)),
        Direction::East => maze.get(&(current.x + 1, current.y)),
        Direction::South => maze.get(&(current.x, current.y + 1)),
        Direction::West => maze.get(&(current.x - 1, current.y))
    }
}

fn recursive_travel(current: &Tile, maze: &Maze, visited: &mut Vec<Tile>) -> Option<Vec<Tile>> {
    use TileType::*;
    visited.push(*current);

    let possible_directions = current.tile.direction_from_tile();
    let valid_tiles : Vec<Tile> = possible_directions.into_iter().map(|d| {
        match get_tile_from_dir(current, maze, &d) {
            None => None,
            Some(tile) => {
                if d.get_possible_tile_from_dir().contains(&tile.tile) {
                    return Some(tile)
                }
                None
            }
        }
    }).filter(|t| t.is_some() && ((t.unwrap().tile == Start && visited.len() > 2 )|| !visited.contains(t.unwrap()))).map(|x| *x.unwrap()).collect();

    if valid_tiles.is_empty() {
        None
    } else if valid_tiles.iter().any(|t| t.tile == Start) {
        Some(visited.clone())
    } else {
        stacker::maybe_grow(1024 * 1024, 1024 * 1024, || {
            for tile in valid_tiles {
                if let Some(x) = recursive_travel(&tile, maze, visited) {
                    return Some(x);
                }
            }
            visited.pop();
            None
        })
    }
}

fn find_loop(maze: &Maze) -> Vec<Tile> {
    let start = maze.values().find(|x| x.tile == TileType::Start).unwrap();
    let mut vec = vec![];
    recursive_travel(start, maze, &mut vec).unwrap()
}

fn part1(data: &String) -> usize {
    let maze = parse_map(data.as_str());
    let maze_loop = find_loop(&maze);
    ((maze_loop.len()-1) / 2) + 1
}

fn axis_crossing(tile: Tile, polygon: &[Tile]) -> bool {
    use TileType::*;
    if polygon.iter().any(|t| t.x ==tile.x && t.y == tile.y) {
        return false
    }

    let mut walls : Vec<&Tile> = polygon.iter().filter(|x| x.y == tile.y).collect();
    walls.sort_by(|a,b| a.x.cmp(&b.x));
    if walls.is_empty() || walls.first().unwrap().x > tile.x {
        return false
    }

    let ray_count = walls.iter().fold(0, |w,t| {
        if t.x > tile.x || [NorthEast, NorthWest, Horizontal, Ground].contains(&t.tile) {
            w
        } else {
            w + 1
        }
    });

    ray_count % 2 == 1
}

fn replace_start_tile(maze_loop: &[Tile]) -> TileType {
    use TileType::*;
    let (a,b) = (maze_loop.get(1).unwrap(), maze_loop.last().unwrap());
    match (a.x - b.x,a.y - b.y) {
        (0, _) => Vertical,
        (_, 0) => Horizontal,
        (1, -1) => NorthEast,
        (-1, -1) => NorthWest,
        (1, 1) => SouthEast,
        (-1, 1) => SouthWest,
        _ => panic!("Should not happen")
    }
}

fn part2(data: &String) -> usize {
    let maze = parse_map(data.as_str());
    let mut maze_loop = find_loop(&maze);
    maze_loop.get_mut(0).unwrap().tile = replace_start_tile(&maze_loop);
    maze.values().collect::<Vec<&Tile>>().into_par_iter().map(|t| (t, axis_crossing(*t, &maze_loop))).filter(|a| a.1).collect::<Vec<_>>().len()
}