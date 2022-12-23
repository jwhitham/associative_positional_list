
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::collections::HashMap;

const DEBUG: bool = true;
type Word = i16;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Location {
    x: Word,
    y: Word,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Item {
    Open,
    Wall,
    Nothing,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Move {
    Forward(Word),
    Acw,
    Cw,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Facing {
    Up, Down, Right, Left,
}


type World = HashMap<Location, Item>;
type Moves = Vec<Move>;

struct Problem {
    world: World,
    moves: Moves,
    width: Word,
    height: Word,
    cube_size: Word,
}


fn load(filename: &str) -> Problem {
    let file = File::open(filename).unwrap();
    let mut p: Problem = Problem {
        world: World::new(),
        moves: Moves::new(),
        width: 0,
        height: 0,
        cube_size: 0,
    };
    let mut y: Word = 0;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_string) = line {
            let bytes = Vec::from_iter(line_string.bytes());
            if bytes.len() == 0 {
                continue;
            }

            match bytes.get(0).unwrap() {
                b' ' | b'.' | b'#' => {
                    parse_line(&mut p, y, &bytes);
                    y += 1;
                },
                _ => {
                    parse_directions(&mut p, &bytes);
                },
            }
        }
    }
    p.height = y;
    return p;
}

fn parse_line(p: &mut Problem, y: Word, bytes: &Vec<u8>) {
    let mut x: Word = 0;
    for b in bytes.iter() {
        p.world.insert(Location {
            x: x,
            y: y,
        }, match b {
            b'.' => Item::Open,
            b'#' => Item::Wall,
            _ => Item::Nothing,
        });
        x += 1;
    }
    p.width = Word::max(x, p.width);
}

fn parse_directions(p: &mut Problem, bytes: &Vec<u8>) {
    let mut number: Word = 0;
    for b in bytes.iter() {
        if (b'0' <= *b) && (*b <= b'9') {
            number *= 10;
            number += (*b - b'0') as Word;
        } else {
            if number != 0 {
                p.moves.push(Move::Forward(number));
                number = 0;
            }
            match b {
                b'L' => {
                    p.moves.push(Move::Acw);
                },
                b'R' => {
                    p.moves.push(Move::Cw);
                },
                _ => {
                    panic!();
                },
            }
        }
    }
    if number != 0 {
        p.moves.push(Move::Forward(number));
    }
}

fn rotate_acw(facing: Facing) -> Facing {
    return match facing {
        Facing::Up =>    Facing::Left,
        Facing::Left =>  Facing::Down,
        Facing::Down =>  Facing::Right,
        Facing::Right => Facing::Up,
    }
}

fn move_forward(loc: Location, facing: Facing) -> Location {
    let mut loc2 = loc;
    match facing {
        Facing::Up =>    { loc2.y -= 1; },
        Facing::Down =>  { loc2.y += 1; },
        Facing::Left =>  { loc2.x -= 1; },
        Facing::Right => { loc2.x += 1; },
    }
    return loc2;
}

fn get_loc(p: &Problem, loc: &Location) -> Item {
    return *p.world.get(&loc).unwrap_or(&Item::Nothing);
}

fn move_forward_and_wrap_part1(p: &Problem, loc: Location, facing: Facing) -> Location {
    let mut loc2 = move_forward(loc, facing);
    if get_loc(p, &loc2) == Item::Nothing {
        // step into nothingness - wrap around
        match facing {
            Facing::Up => { loc2.y = p.height - 1; },
            Facing::Down => { loc2.y = 0; },
            Facing::Left =>  { loc2.x = p.width - 1; },
            Facing::Right =>  { loc2.x = 0; },
        }
        while get_loc(p, &loc2) == Item::Nothing {
            loc2 = move_forward(loc2, facing);
        }
    }
    match get_loc(p, &loc2) {
        Item::Open => {
            // accept
            return loc2;
        },
        Item::Wall => {
            // hit a wall - nothing happens
            return loc;
        },
        Item::Nothing => {
            // Should be impossible
            panic!();
        }
    }
}

fn part1(filename: &str) -> u64 {
    let p = load(filename);
    let mut loc = Location { x: 0, y: 0 };

    // find the start point
    let mut found = false;
    for x in 0 .. p.width {
        loc.x = x;
        if get_loc(&p, &loc) == Item::Open {
            found = true;
            break;
        }
    }
    assert!(found);

    // follow directions
    let mut facing = Facing::Right;
    let mut trace: HashMap<Location, Facing> = HashMap::new();

    for d in &p.moves {
        match d {
            Move::Acw => {
                facing = rotate_acw(facing);
            },
            Move::Cw => {
                for _ in 0 .. 3 {
                    facing = rotate_acw(facing);
                }
            },
            Move::Forward(n) => {
                for _ in 0 .. *n {
                    trace.insert(loc, facing);
                    loc = move_forward_and_wrap_part1(&p, loc, facing);
                }
            },
        }
    }
    trace.insert(loc, facing);

    // Where do we end up?
    let mut result = (1000 * (1 + (loc.y as u64))) + (4 * (1 + (loc.x as u64)));
    match facing {
        Facing::Right =>  { result += 0; },
        Facing::Down => { result += 1; },
        Facing::Left =>  { result += 2; },
        Facing::Up => { result += 3; },
    }

    // Draw it
    if DEBUG {
        for y in 0 .. p.height {
            for x in 0 .. p.width {
                let loc = Location { x: x, y: y };
                let t = trace.get(&loc);
                let item = get_loc(&p, &loc);
                if t.is_some() {
                    assert!(item == Item::Open);
                    match t.unwrap() {
                        Facing::Right => { print!(">"); },
                        Facing::Down =>  { print!("v"); },
                        Facing::Left =>  { print!("<"); },
                        Facing::Up =>    { print!("^"); },
                    }
                } else {
                    match item {
                        Item::Open =>    { print!("."); },
                        Item::Wall =>    { print!("#"); },
                        Item::Nothing => { print!(" "); },
                    }
                }
            }
            println!();
        }
    }
    return result;
}

#[test]
fn test_part1() {
    assert_eq!(part1(&"test"), 6032);
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Location3D {
    x: Word,
    y: Word,
    z: Word,
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Vector {
    dx: i8,
    dy: i8,
    dz: i8,
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Face {
    loc_2d: Location,
    loc_3d: Location3D,
    vec_x: Vector,
    vec_y: Vector,
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Voxel {
    face: usize,
    loc_2d: Location,
    item: Item,
}

fn is_valid_vector(v: &Vector) -> bool {
    return ((i8::abs(v.dx) == 1) && (v.dy == 0) && (v.dz == 0))
        || ((i8::abs(v.dy) == 1) && (v.dx == 0) && (v.dz == 0))
        || ((i8::abs(v.dz) == 1) && (v.dx == 0) && (v.dy == 0));
}

fn negative(v: &Vector) -> Vector {
    return Vector { dx: -v.dx, dy: -v.dy, dz: -v.dz };
}

fn rotate_axis(to_rotate: &Vector, around: &Vector) -> Vector {
    assert!(is_valid_vector(to_rotate));
    assert!(is_valid_vector(around));
    if around.dx > 0 {
        return Vector { dx: to_rotate.dx, dy: -to_rotate.dz, dz: to_rotate.dy };
    } else if around.dy > 0 {
        return Vector { dy: to_rotate.dy, dz: to_rotate.dx, dx: -to_rotate.dz };
    } else if around.dz > 0 {
        return Vector { dz: to_rotate.dz, dy: to_rotate.dx, dx: -to_rotate.dy };
    } else {
        let mut copy: Vector = *to_rotate;
        for _ in 0 .. 3 {
            copy = rotate_axis(&copy, &negative(around));
        }
        return copy;
    }
}

fn part2(filename: &str) -> u64 {
    let p = load(filename);

    // determine the cube size
    let smaller = Word::min(p.width, p.height);
    let larger = Word::max(p.width, p.height);
    assert!((larger % 4) == 0);
    assert!((smaller % 3) == 0);
    let cube_size = smaller / 3;
    assert!((larger / 4) == cube_size);

    // Find faces in flat representation
    let mut faces: Vec<Face> = Vec::new();
    for fy in 0 .. 4 {
        for fx in 0 .. 4 {
            let loc = Location { x: fx * cube_size, y: fy * cube_size };
            if get_loc(&p, &loc) != Item::Nothing {
                // This face exists in the flat representation
                // The location in the 3D representation is not yet known
                faces.push(Face {
                    loc_2d: loc,
                    loc_3d: Location3D { x: 0, y: 0, z: 0 },
                    vec_x: Vector { dx: i8::MAX, dy: i8::MAX, dz: i8::MAX },
                    vec_y: Vector { dx: i8::MAX, dy: i8::MAX, dz: i8::MAX },
                });
            }
        }
    }

    assert!(faces.len() == 6);

    // Face 0 represents the top of the cube
    faces.get_mut(0).unwrap().vec_x = Vector {
        dx: 1, dy: 0, dz: 0,
    };
    faces.get_mut(0).unwrap().vec_y = Vector {
        dx: 0, dy: 1, dz: 0,
    };
    faces.get_mut(0).unwrap().loc_3d = Location3D {
        x: 0, y: 0, z: cube_size + 1,
    };

    // Find other faces in 3D representation based on adjacency in the 2D representation
    let mut unmapped: u8 = 5;
    while unmapped > 0 {
        let mut progress = false;
        for a in 0 .. 6 {
            // Find an unmapped face "fb" that's adjacent to "fa" in the 2D representation
            let fa = *faces.get(a).unwrap();
            if fa.vec_x.dx == i8::MAX {
                continue; // fa not mapped yet
            }

            for b in 0 .. 6 {
                let mut fb = *faces.get(b).unwrap();
                if fb.vec_x.dx != i8::MAX {
                    continue; // fb already mapped
                }

                fb.vec_x = fa.vec_x;
                fb.vec_y = fa.vec_y;
                fb.loc_3d = fa.loc_3d;
                assert!(is_valid_vector(&fa.vec_x));
                assert!(is_valid_vector(&fa.vec_y));

                if fa.loc_2d.y == fb.loc_2d.y {
                    // Same Y location in 2D plane
                    if fa.loc_2d.x + cube_size == fb.loc_2d.x {
                        // Cw side (X dimension)
                        fb.vec_x = rotate_axis(&fb.vec_x, &negative(&fb.vec_y));
                        fb.loc_3d = Location3D {
                            x: fa.loc_3d.x + ((fa.vec_x.dx as Word) * (cube_size + 1)),
                            y: fa.loc_3d.y + ((fa.vec_x.dy as Word) * (cube_size + 1)),
                            z: fa.loc_3d.z + ((fa.vec_x.dz as Word) * (cube_size + 1)),
                        };
                    } else if fa.loc_2d.x - cube_size == fb.loc_2d.x {
                        // Acw side (X dimension)
                        fb.vec_x = rotate_axis(&fb.vec_x, &negative(&fb.vec_y));
                        fb.loc_3d = Location3D {
                            x: fa.loc_3d.x - ((fb.vec_x.dx as Word) * (cube_size + 1)),
                            y: fa.loc_3d.y - ((fb.vec_x.dy as Word) * (cube_size + 1)),
                            z: fa.loc_3d.z - ((fb.vec_x.dz as Word) * (cube_size + 1)),
                        };
                    } else {
                        continue;
                    }
                } else if fa.loc_2d.x == fb.loc_2d.x {
                    // Same X location in 2D plane
                    if fa.loc_2d.y + cube_size == fb.loc_2d.y {
                        // Bottom side (Y dimension)
                        fb.vec_y = rotate_axis(&fb.vec_y, &negative(&fb.vec_x));
                        fb.loc_3d = Location3D {
                            x: fa.loc_3d.x + ((fa.vec_y.dx as Word) * (cube_size + 1)),
                            y: fa.loc_3d.y + ((fa.vec_y.dy as Word) * (cube_size + 1)),
                            z: fa.loc_3d.z + ((fa.vec_y.dz as Word) * (cube_size + 1)),
                        };
                    } else if fa.loc_2d.y - cube_size == fb.loc_2d.y {
                        // Top side (X dimension)
                        fb.vec_y = rotate_axis(&fb.vec_y, &negative(&fb.vec_x));
                        fb.loc_3d = Location3D {
                            x: fa.loc_3d.x - ((fb.vec_y.dx as Word) * (cube_size + 1)),
                            y: fa.loc_3d.y - ((fb.vec_y.dy as Word) * (cube_size + 1)),
                            z: fa.loc_3d.z - ((fb.vec_y.dz as Word) * (cube_size + 1)),
                        };
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }

                if DEBUG {
                    println!("plane {} (adjacent to {}) is at 2d x={} y={}",
                             b, a, fb.loc_2d.x, fb.loc_2d.y);
                    println!("    x in the 2D plane dx={} dy={} dz={}",
                             fb.vec_x.dx, fb.vec_x.dy, fb.vec_x.dz);
                    println!("    y in the 2D plane dx={} dy={} dz={}",
                             fb.vec_y.dx, fb.vec_y.dy, fb.vec_y.dz);
                    println!("    3d x={} y={} z={}",
                             fb.loc_3d.x, fb.loc_3d.y, fb.loc_3d.z);
                }

                assert!(is_valid_vector(&fb.vec_x));
                assert!(is_valid_vector(&fb.vec_y));
                *faces.get_mut(b).unwrap() = fb;
                unmapped -= 1;
                progress = true;
            }
        }
        assert!(progress);
    }

    // Generate voxel map
    let mut voxel: HashMap<Location3D, Voxel> = HashMap::new();
    for a in 0 .. 6 {
        let fa = faces.get(a).unwrap();
        assert!(is_valid_vector(&fa.vec_x));
        assert!(is_valid_vector(&fa.vec_y));
        for y in 0 .. cube_size {
            for x in 0 .. cube_size {
                let loc_2d = Location {
                    x: fa.loc_2d.x + x,
                    y: fa.loc_2d.y + y,
                };
                let item = get_loc(&p, &loc_2d);
                assert!(item != Item::Nothing);

                let loc_3d = Location3D {
                    x: fa.loc_3d.x + ((fa.vec_x.dx as Word) * (x + 1))
                                   + ((fa.vec_y.dx as Word) * (y + 1)),
                    y: fa.loc_3d.y + ((fa.vec_x.dy as Word) * (x + 1))
                                   + ((fa.vec_y.dy as Word) * (y + 1)),
                    z: fa.loc_3d.z + ((fa.vec_x.dz as Word) * (x + 1))
                                   + ((fa.vec_y.dz as Word) * (y + 1)),
                };
                if DEBUG {
                    println!("3d location for {} x={} y={} z={}",
                             a, loc_3d.x, loc_3d.y, loc_3d.z);
                }
                assert!(!voxel.contains_key(&loc_3d));
                assert!(loc_3d.x >= 0);
                assert!(loc_3d.y >= 0);
                assert!(loc_3d.z >= 0);
                assert!(loc_3d.x <= (cube_size + 1));
                assert!(loc_3d.y <= (cube_size + 1));
                assert!(loc_3d.z <= (cube_size + 1));
                voxel.insert(loc_3d, Voxel {
                    face: a,
                    loc_2d: loc_2d,
                    item: item,
                });
            }
        }
    }

    // Draw voxel map
    if DEBUG {
        for z in 0 .. cube_size + 2 {
            println!();
            println!("-------------- z = {}", z);
            for y in 0 .. cube_size + 2 {
                for x in 0 .. cube_size + 2 {
                    let v = voxel.get(&Location3D { x: x, y: y, z: z, });
                    if v.is_none() {
                        print!(" ");
                        continue;
                    }
                    match v.unwrap().item {
                        Item::Open =>    { print!("."); },
                        Item::Wall =>    { print!("#"); },
                        Item::Nothing => { print!(" "); },
                    }
                }
                println!();
            }
        }
    }

    // follow directions
    let mut trace: HashMap<Location, Facing> = HashMap::new();
    let mut facing = Facing::Right;
    let mut loc = Location3D {
        x: 1, y: 1, z: cube_size + 1,
    };

    let mut move_forward_part_2 = |facing: Facing| {
        let dx: i8;
        let dy: i8;
        match facing {
            Facing::Right => { dx = 1; dy = 0; },
            Facing::Down =>  { dx = 0; dy = 1; },
            Facing::Left =>  { dx = -1; dy = 0; },
            Facing::Up =>    { dx = 0; dy = -1; },
        }
        let v1 = voxel.get(&loc).unwrap();
        let f1 = faces.get(v1.face).unwrap();
        let mut loc2 = Location3D {
            x: loc.x + (((dx * f1.vec_x.dx) + (dy * f1.vec_y.dx)) as Word),
            y: loc.y + (((dx * f1.vec_x.dy) + (dy * f1.vec_y.dy)) as Word),
            z: loc.z + (((dx * f1.vec_x.dz) + (dy * f1.vec_y.dz)) as Word),
        };
        trace.insert(v1.loc_2d, facing);

        if !voxel.contains_key(&loc2) {
            // Moved to a new face - but which one?
            let check: [Vector; 6] = [
                Vector { dx: -1, dy: 0, dz: 0 },
                Vector { dx:  1, dy: 0, dz: 0 },
                Vector { dy: -1, dx: 0, dz: 0 },
                Vector { dy:  1, dx: 0, dz: 0 },
                Vector { dz: -1, dx: 0, dy: 0 },
                Vector { dz:  1, dx: 0, dy: 0 },
            ];
            let mut found: Option<Location3D> = None;
            for v in check {
                let loc3 = Location3D {
                    x: loc2.x + (v.dx as Word),
                    y: loc2.y + (v.dy as Word),
                    z: loc2.z + (v.dz as Word),
                };
                let v2 = voxel.get(&loc3);
                if v2.is_some() && (v2.unwrap().face != v1.face) {
                    assert!(found.is_none());
                    found = Some(loc3);
                }
            }
            assert!(found.is_some());
            loc2 = found.unwrap();
        }
        // Detect wall
        if voxel.get(&loc2).unwrap().item == Item::Open {
            loc = loc2;
        }
    };

    for d in &p.moves {
        match d {
            Move::Acw => {
                facing = rotate_acw(facing);
            },
            Move::Cw => {
                for _ in 0 .. 3 {
                    facing = rotate_acw(facing);
                }
            },
            Move::Forward(n) => {
                for _ in 0 .. *n {
                    move_forward_part_2(facing);
                }
            },
        }
    }

    // Where do we end up?
    let v = voxel.get(&loc).unwrap();
    trace.insert(v.loc_2d, facing);
    let mut result = (1000 * (1 + (v.loc_2d.y as u64))) + (4 * (1 + (v.loc_2d.x as u64)));
    match facing {
        Facing::Right => { result += 0; },
        Facing::Down =>  { result += 1; },
        Facing::Left =>  { result += 2; },
        Facing::Up =>    { result += 3; },
    }

    // Draw it
    if DEBUG {
        for y in 0 .. p.height {
            for x in 0 .. p.width {
                let loc = Location { x: x, y: y };
                let t = trace.get(&loc);
                let item = get_loc(&p, &loc);
                if t.is_some() {
                    assert!(item == Item::Open);
                    match t.unwrap() {
                        Facing::Right => { print!(">"); },
                        Facing::Down =>  { print!("v"); },
                        Facing::Left =>  { print!("<"); },
                        Facing::Up =>    { print!("^"); },
                    }
                } else {
                    match item {
                        Item::Open =>    { print!("."); },
                        Item::Wall =>    { print!("#"); },
                        Item::Nothing => { print!(" "); },
                    }
                }
            }
            println!();
        }
    }

    return result;
}
#[test]
fn test_part2() {
    assert_eq!(part2(&"test"), 5031);
}

fn main() {
    println!("{}", part1(&"input"));
    println!("{}", part2(&"test"));
}
