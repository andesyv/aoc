use std::collections::HashSet;

fn main() {
//     const INPUT: &str = "v...>>.vv>
// .vv>>.vv..
// >>.>v>...v
// >>v>>.>.v.
// v>v.vv.v..
// >.>>..v...
// .vv..>.>v.
// v.v..>>v.v
// ....v..v.>";
    const INPUT: &str = include_str!("../inputs/25.txt");

    let mut grid = parse_cucumbers(&INPUT);
    println!("Initial grid:");
    print_cucumbers(&grid);

    for i in 1.. {
        let new_grid = move_cucumbers(&grid);
        if grid_equals(&grid, &new_grid) {
            println!("Cucumbers stable after {} iterations", i);
            break;
        }
        grid = new_grid;
    }

    println!("Final grid:");
    print_cucumbers(&grid);
}

type Coord = (i64, i64);

#[derive(Clone, Copy, PartialEq)]
enum Entity {
    EastCucumber,
    SouthCucumber,
    Nothing,
}

fn parse_cucumbers(input: &str) -> Vec<Vec<Entity>> {
    use Entity::{EastCucumber, Nothing, SouthCucumber};
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '>' => EastCucumber,
                    'v' => SouthCucumber,
                    _ => Nothing,
                })
                .collect()
        })
        .collect()
}

fn print_cucumbers(grid: &Vec<Vec<Entity>>) {
    use Entity::{EastCucumber, Nothing, SouthCucumber};
    for line in grid {
        let ln: String = line
            .iter()
            .map(|e| match e {
                EastCucumber => '>',
                SouthCucumber => 'v',
                Nothing => '.',
            })
            .collect();
        println!("{}", ln);
    }
}

fn move_cucumbers(grid: &Vec<Vec<Entity>>) -> Vec<Vec<Entity>> {
    let (grid_width, grid_height) = (grid.iter().next().unwrap().len(), grid.len());
    let mut new_grid = grid.clone();
    for entity_coord in get_entity_coords(grid, &Entity::EastCucumber) {
        let forward_coord: (i64, i64) = (
            (entity_coord.0 + 1).try_into().unwrap(),
            entity_coord.1.try_into().unwrap(),
        );
        if index(grid, forward_coord).unwrap() == &Entity::Nothing {
            *index_mut(&mut new_grid, forward_coord).unwrap() = Entity::EastCucumber;
            *index_mut(&mut new_grid, entity_coord).unwrap() = Entity::Nothing;
        }
    }

    let mut new_grid_2 = new_grid.clone();
    for entity_coord in get_entity_coords(&new_grid, &Entity::SouthCucumber) {
        let forward_coord: (i64, i64) = (
            entity_coord.0.try_into().unwrap(),
            (entity_coord.1 + 1).try_into().unwrap(),
        );
        if index(&new_grid, forward_coord).unwrap() == &Entity::Nothing {
            *index_mut(&mut new_grid_2, forward_coord).unwrap() = Entity::SouthCucumber;
            *index_mut(&mut new_grid_2, entity_coord).unwrap() = Entity::Nothing;
        }
    }

    new_grid_2
}

fn get_entity_coords(grid: &Vec<Vec<Entity>>, entity: &Entity) -> HashSet<Coord> {
    // Could just do some filters and maps on the list,
    // but more idiomic, faster and easy to read this way:
    let mut set = HashSet::new();
    for (j, xs) in grid.iter().enumerate() {
        for (i, x) in xs.iter().enumerate() {
            if x == entity {
                set.insert((i.try_into().unwrap(), j.try_into().unwrap()));
            }
        }
    }
    set
}

fn index(grid: &Vec<Vec<Entity>>, (i, j): (i64, i64)) -> Option<&Entity> {
    let (grid_width, grid_height) = (grid.iter().next()?.len(), grid.len());
    let bounded_x: usize = (if i < 0 {
        i64::try_from(grid_width).ok()? - i % i64::try_from(grid_width).ok()?
    } else {
        i % i64::try_from(grid_width).ok()?
    })
    .try_into()
    .ok()?;
    let bounded_y: usize = (if j < 0 {
        i64::try_from(grid_height).ok()? - j % i64::try_from(grid_height).ok()?
    } else {
        j % i64::try_from(grid_height).ok()?
    })
    .try_into()
    .ok()?;
    grid.get(bounded_y)?.get(bounded_x)
}

fn index_mut(grid: &mut Vec<Vec<Entity>>, (i, j): (i64, i64)) -> Option<&mut Entity> {
    let (grid_width, grid_height) = (grid.iter().next()?.len(), grid.len());
    let bounded_x: usize = (if i < 0 {
        i64::try_from(grid_width).ok()? - i % i64::try_from(grid_width).ok()?
    } else {
        i % i64::try_from(grid_width).ok()?
    })
    .try_into()
    .ok()?;
    let bounded_y: usize = (if j < 0 {
        i64::try_from(grid_height).ok()? - j % i64::try_from(grid_height).ok()?
    } else {
        j % i64::try_from(grid_height).ok()?
    })
    .try_into()
    .ok()?;
    grid.get_mut(bounded_y)?.get_mut(bounded_x)
}

fn grid_equals(a: &Vec<Vec<Entity>>, b: &Vec<Vec<Entity>>) -> bool {
    !a.iter()
        .zip(b.iter())
        .any(|(ls, rs)| ls.iter().zip(rs.iter()).any(|(l, r)| l != r))
}

#[test]
fn test_move() {
    const INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    const RESULT: &str = "....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v";

    let grid = parse_cucumbers(&INPUT);
    assert!(grid_equals(
        &move_cucumbers(&grid),
        &parse_cucumbers(&RESULT)
    ));
}
