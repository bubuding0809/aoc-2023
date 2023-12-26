use std::{collections::HashSet, fs, ops::ControlFlow};

fn main() {
    let input = include_str!("./input-02.txt");
    let output = p2(input);
    println!("Solution for part-02: {}", output)
}

#[derive(Debug)]
enum TileType {
    Normal,
    Loop,
    Within,
}
#[derive(Debug)]
struct MapTile {
    tile: Tile,
    tile_type: TileType,
    coords: (usize, usize),
    deltas: HashSet<(i32, i32)>,
}

impl MapTile {
    fn new(tile: Tile, coords: (usize, usize), deltas: HashSet<(i32, i32)>) -> Self {
        MapTile {
            tile,
            tile_type: TileType::Normal,
            coords,
            deltas,
        }
    }

    fn mark_loop(&mut self) {
        self.tile_type = TileType::Loop;
    }

    fn mark_within(&mut self) {
        self.tile_type = TileType::Within;
    }

    fn direction_from(&self, tile: &MapTile) -> Direction {
        let to: (i32, i32) = (
            self.coords.0.try_into().unwrap_or_default(),
            self.coords.1.try_into().unwrap_or_default(),
        );
        let from: (i32, i32) = (
            tile.coords.0.try_into().unwrap_or_default(),
            tile.coords.1.try_into().unwrap_or_default(),
        );
        let delta = (from.0 - to.0, from.1 - to.1);

        match delta {
            (1, 0) => Direction::Down,
            (-1, 0) => Direction::Up,
            (0, 1) => Direction::Left,
            (0, -1) => Direction::Right,
            _ => Direction::Invalid,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Invalid,
}

impl Direction {
    fn not_opposite(&self, compare: &Direction) -> bool {
        !matches!(
            (self, compare),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        )
    }
}

#[derive(Debug)]
enum Tile {
    Start,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
}

fn parse_input(input: &str) -> Vec<Vec<MapTile>> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    'S' => MapTile::new(
                        Tile::Start,
                        (i, j),
                        HashSet::from([(1, 0), (0, 1), (-1, 0), (0, -1)]),
                    ),
                    '|' => MapTile::new(Tile::Vertical, (i, j), HashSet::from([(1, 0), (-1, 0)])),
                    '-' => MapTile::new(Tile::Horizontal, (i, j), HashSet::from([(0, -1), (0, 1)])),
                    'L' => MapTile::new(Tile::NorthEast, (i, j), HashSet::from([(-1, 0), (0, 1)])),
                    'J' => MapTile::new(Tile::NorthWest, (i, j), HashSet::from([(-1, 0), (0, -1)])),
                    '7' => MapTile::new(Tile::SouthWest, (i, j), HashSet::from([(1, 0), (0, -1)])),
                    'F' => MapTile::new(Tile::SouthEast, (i, j), HashSet::from([(1, 0), (0, 1)])),
                    _ => MapTile::new(Tile::Ground, (i, j), HashSet::from([])),
                })
                .collect()
        })
        .collect()
}

fn get_start_coords(map: &[Vec<MapTile>]) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if let Tile::Start = map[i][j].tile {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn set_start_tile_type(start: (usize, usize), map: &mut [Vec<MapTile>]) {
    // Get row size and col size
    let rows: i32 = map.len().try_into().unwrap();
    let cols: i32 = map[0].len().try_into().unwrap();

    // Find next tile to traverse to from the starting tile
    let deltas = map[start.0][start.1]
        .deltas
        .clone()
        .into_iter()
        .filter(|(dy, dx)| {
            // Get neighbour coords
            let (r, c) = (
                (start.0 as i32 + dy).clamp(0, rows - 1),
                (start.1 as i32 + dx).clamp(0, cols - 1),
            );

            // Check if neighbour is a valid destination
            map[r as usize][c as usize].deltas.iter().any(|(dy, dx)| {
                let k = (r + dy).clamp(0, rows - 1);
                let l = (c + dx).clamp(0, cols - 1);
                matches!(&map[k as usize][l as usize].tile_type, TileType::Loop)
            })
        })
        .collect::<HashSet<(i32, i32)>>();

    if deltas.is_superset(&HashSet::from([(1, 0), (-1, 0)])) {
        map[start.0][start.1].tile = Tile::Vertical;
    } else if deltas.is_superset(&HashSet::from([(0, -1), (0, 1)])) {
        map[start.0][start.1].tile = Tile::Horizontal;
    } else if deltas.is_superset(&HashSet::from([(-1, 0), (0, 1)])) {
        map[start.0][start.1].tile = Tile::NorthWest;
    } else if deltas.is_superset(&HashSet::from([(-1, 0), (0, -1)])) {
        map[start.0][start.1].tile = Tile::NorthEast;
    } else if deltas.is_superset(&HashSet::from([(1, 0), (0, 1)])) {
        map[start.0][start.1].tile = Tile::SouthWest;
    } else if deltas.is_superset(&HashSet::from([(1, 0), (0, -1)])) {
        map[start.0][start.1].tile = Tile::SouthEast;
    }

    // Save correct deltas
    map[start.0][start.1].deltas = deltas;
}

fn get_curr_coords(start: (usize, usize), map: &[Vec<MapTile>]) -> (usize, usize) {
    // Get row size and col size
    let rows: i32 = map.len().try_into().unwrap();
    let cols: i32 = map[0].len().try_into().unwrap();

    // Find next tile to traverse to from the starting tile
    let deltas = &map[start.0][start.1].deltas.iter().filter(|(dy, dx)| {
        // Get neighbour coords
        let (r, c) = (
            (start.0 as i32 + dy).clamp(0, rows - 1),
            (start.1 as i32 + dx).clamp(0, cols - 1),
        );

        // Check if neighbour is a valid destination
        map[r as usize][c as usize].deltas.iter().any(|(dy, dx)| {
            if let (Some(r), Some(c)) = (r.checked_add(*dy), c.checked_add(*dx)) {
                return matches!(&map[r as usize][c as usize].tile_type, TileType::Loop);
            };
            false
        })
    });

    let (dy, dx) = deltas.clone().next().unwrap();

    (
        (start.0 as i32 + dy) as usize,
        (start.1 as i32 + dx) as usize,
    )
}

fn next_tile_coords(
    prev: (usize, usize),
    curr: (usize, usize),
    map: &mut [Vec<MapTile>],
) -> Option<(usize, usize)> {
    // Mark curr tile as loop
    map[curr.0][curr.1].mark_loop();

    let prev_tile = &map[prev.0][prev.1];
    let curr_tile = &map[curr.0][curr.1];
    let direction_prev = curr_tile.direction_from(prev_tile);

    curr_tile
        .deltas
        .iter()
        .map(|(dy, dx)| (curr.0 as i32 + dy, curr.1 as i32 + dx))
        .find(|(r, c)| {
            let to_tile = &map[*r as usize][*c as usize];
            let direction_from = to_tile.direction_from(curr_tile);
            direction_from.not_opposite(&direction_prev)
                && !matches!(to_tile.tile_type, TileType::Loop)
        })
        .as_ref()
        .map(|(r, c)| (*r as usize, *c as usize))
}

fn stamp_loop(start: (usize, usize), mut map: Vec<Vec<MapTile>>) -> Vec<Vec<MapTile>> {
    // Get beginning tile to start loop from
    let mut prev = start;
    let mut curr = get_curr_coords(start, &map);

    // Traverse through loop and get number of steps taken
    let _steps = (1..).try_for_each(|s| match next_tile_coords(prev, curr, &mut map) {
        Some(next) => {
            prev = curr;
            curr = next;

            ControlFlow::Continue(())
        }
        None => ControlFlow::Break(s),
    });

    map
}

fn stamp_inner_tiles(mut map: Vec<Vec<MapTile>>) -> Vec<Vec<MapTile>> {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let tile = &map[i][j];

            if let (TileType::Normal, true) = (
                &tile.tile_type,
                via_ray_cast(map[i][j].coords, &map) % 2 != 0,
            ) {
                map[i][j].mark_within();
            }
        }
    }

    map
}

fn via_ray_cast(coords: (usize, usize), map: &[Vec<MapTile>]) -> u32 {
    (0..coords.1).rev().fold(0, |acc, j| {
        let map_tile = &map[coords.0][j];
        if !matches!(map_tile.tile_type, TileType::Loop) {
            return acc;
        }

        match map_tile.tile {
            Tile::Vertical | Tile::NorthEast | Tile::NorthWest => acc + 1,
            _ => acc,
        }
    })
}

fn write_loop_to_txt(map: &[Vec<MapTile>]) -> Result<(), std::io::Error> {
    let map_string: String = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|map_tile| match map_tile {
                    MapTile {
                        tile: Tile::Start,
                        tile_type: TileType::Loop,
                        ..
                    } => '#',
                    MapTile {
                        tile_type: TileType::Loop,
                        ..
                    } => '*',
                    MapTile {
                        tile_type: TileType::Within,
                        ..
                    } => 'I',
                    _ => '0',
                })
                .collect::<String>()
                + "\n"
        })
        .collect();

    fs::write("./mapped.txt", map_string.trim())
}

fn p2(input: &str) -> String {
    let mut map = parse_input(input);

    // Get coords of the starting tile
    let start = get_start_coords(&map);

    map[start.0][start.1].mark_loop();

    // Properly set start tile type
    set_start_tile_type(start, &mut map);

    dbg!(&map[start.0][start.1]);

    // Stamp all the MapTiles that is identified as part of the loop
    let map = stamp_loop(start, map);

    // Travese loop and stamp all tiles that are enclosed within the loop
    let map = stamp_inner_tiles(map);

    write_loop_to_txt(&map).expect("Unable to write map to txt file");

    // Count area that is enclosed by the loop
    let area: u32 = map
        .iter()
        .map(|row| {
            row.iter().fold(0, |area, tile| match tile.tile_type {
                TileType::Within => area + 1,
                _ => area,
            })
        })
        .sum();

    area.to_string()
}

#[cfg(test)]
mod tests {
    use std::io::Error;

    use super::*;

    #[test]
    fn shit_works() -> Result<(), Error> {
        let test_input = include_str!("./test-input-02.txt");
        let test_output = p2(test_input);
        println!("p2 test: {}", test_output);
        assert_eq!(test_output, "10");
        Ok(())
    }

    #[test]
    fn test_stamp_loop() -> Result<(), Error> {
        let test_input = include_str!("./input-02.txt");
        let map = parse_input(test_input);

        // Get coords of the starting tile
        let start = get_start_coords(&map);

        // Stamp all the MapTiles that is identified as part of the loop
        let map = stamp_loop(start, map);

        write_loop_to_txt(&map).expect("Unable to write map to txt file");

        let steps: u32 = map
            .iter()
            .map(|row| {
                row.iter().fold(0, |acc, curr| match curr.tile_type {
                    TileType::Loop => acc + 1,
                    _ => acc,
                })
            })
            .sum();

        assert_eq!(steps / 2, 6875);

        Ok(())
    }

    #[test]
    fn test_ray_cast() -> Result<(), Error> {
        let test_input = include_str!("./test-input-02.txt");
        let map = parse_input(test_input);
        let start = get_start_coords(&map);
        let map = stamp_loop(start, map);
        let intersections = via_ray_cast((6, 9), &map);
        assert!(intersections % 2 != 0);
        Ok(())
    }
}
