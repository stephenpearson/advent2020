use std::fs::File;
use std::io::{self, BufRead};

type HyperCubes = Vec<Vec<Vec<Vec<bool>>>>;
type Cube = Vec<Vec<Vec<bool>>>;

fn iterate(cubes: HyperCubes) -> HyperCubes {
    let mut newcube: HyperCubes = Vec::new();
    for x in 0..cubes.len() {
        let mut plane: Vec<Vec<Vec<bool>>> = Vec::new();
        for y in 0..cubes[x].len() {
            let mut row: Vec<Vec<bool>> = Vec::new();
            for z in 0..cubes[x][y].len() {
                let mut subrow: Vec<bool> = Vec::new();
                for a in 0..cubes[x][y][z].len() {
                    let count = count_neighbours(&cubes, x, y, z, a);
                    let mut cell = false;
                    if cubes[x][y][z][a] {
                        if count == 2 || count == 3 {
                            cell = true;
                        }
                    } else {
                        if count == 3 {
                            cell = true;
                        }
                    }
                    subrow.push(cell);
                }
                row.push(subrow);
            }
            plane.push(row);
        }
        newcube.push(plane);
    }
    newcube
}

fn count_neighbours(cubes: &HyperCubes, x: usize, y: usize, z: usize, a: usize) -> i32 {
    let mut count = 0;
    for i in -1..2 {
        for j in -1..2 {
            for k in -1..2 {
                for l in -1..2 {
                    if i == 0 && j == 0 && k == 0 && l == 0 {
                        continue;
                    }
                    let xp: i32 = x as i32 + i;
                    let yp: i32 = y as i32 + j;
                    let zp: i32 = z as i32 + k;
                    let ap: i32 = a as i32 + l;
                    if xp < 0
                        || xp >= cubes.len() as i32
                        || yp < 0
                        || yp >= cubes[xp as usize].len() as i32
                        || zp < 0
                        || zp >= cubes[xp as usize][yp as usize].len() as i32
                        || ap < 0
                        || ap >= cubes[xp as usize][yp as usize][zp as usize].len() as i32
                    {
                        continue;
                    }
                    if cubes[xp as usize][yp as usize][zp as usize][ap as usize] {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn count_all(cubes: &HyperCubes) -> i32 {
    let mut count = 0;
    for x in 0..cubes.len() {
        for y in 0..cubes[x].len() {
            for z in 0..cubes[x][y].len() {
                for a in 0..cubes[x][y][z].len() {
                    if cubes[x][y][z][a] {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn hit_edge(cubes: &HyperCubes) -> bool {
    for x in 0..cubes.len() {
        for y in 0..cubes[x].len() {
            for z in 0..cubes[x][y].len() {
                for a in 0..cubes[x][y][z].len() {
                    if cubes[x][y][z][a] && (x == 0 || y == 0 || z == 0 || a == 0) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn expand(cubes: HyperCubes) -> HyperCubes {
    if !hit_edge(&cubes) {
        return cubes;
    }
    let mut hypercube: HyperCubes = Vec::new();
    hypercube.push(vec![
        vec![
            vec![false; cubes[0][0][0].len() + 2];
            cubes[0][0].len() + 2
        ];
        cubes[0].len() + 2
    ]);
    for w in 0..cubes.len() {
        let mut cube: Cube = Vec::new();
        cube.push(vec![
            vec![false; cubes[0][0].len() + 2];
            cubes[0][0].len() + 2
        ]);
        for x in 0..cubes[w].len() {
            let mut plane: Vec<Vec<bool>> = Vec::new();
            plane.push(vec![false; cubes[w][x].len() + 2]);
            for y in 0..cubes[w][x].len() {
                let mut row: Vec<bool> = Vec::new();
                row.push(false);
                for z in 0..cubes[w][x][y].len() {
                    row.push(cubes[w][x][y][z]);
                }
                row.push(false);
                plane.push(row);
            }
            plane.push(vec![false; cubes[w][x].len() + 2]);
            cube.push(plane);
        }
        cube.push(vec![
            vec![false; cubes[0][0].len() + 2];
            cubes[0][0].len() + 2
        ]);
        hypercube.push(cube);
    }
    hypercube.push(vec![
        vec![
            vec![false; cubes[0][0][0].len() + 2];
            cubes[0][0].len() + 2
        ];
        cubes[0].len() + 2
    ]);
    hypercube
}

fn print(cubes: &HyperCubes) {
    for x in 0..cubes.len() {
        for y in 0..cubes[x].len() {
            for z in 0..cubes[x][y].len() {
                for a in 0..cubes[x][y][z].len() {
                    let ch = if cubes[x][y][z][a] { "#" } else { "." };
                    print!("{}", ch);
                }
                println!("");
            }
            println!("");
        }
        println!("");
    }
}

fn main() -> std::io::Result<()> {
    let mut hypercubes: HyperCubes = Vec::new();
    let file = File::open("./input.txt")?;
    let mut rows: Vec<Vec<bool>> = Vec::new();
    let mut cube: Cube = Vec::new();
    for line in io::BufReader::new(file).lines() {
        if let Ok(data) = line {
            let cols: Vec<bool> = data.chars().into_iter().map(|x| x == '#').collect();
            rows.push(cols);
        }
    }
    cube.push(rows);
    hypercubes.push(cube);

    for _ in 0..6 {
        hypercubes = expand(hypercubes);
        hypercubes = iterate(hypercubes);
    }
    print(&hypercubes);
    println!("{}", count_all(&hypercubes));
    Ok(())
}
