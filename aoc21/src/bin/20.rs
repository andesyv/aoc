use std::iter;

fn main() {
//     const INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
// #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
// .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
// .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
// .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
// ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
// ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

// #..#.
// #....
// ##..#
// ..#..
// ..###";
    const INPUT: &str = include_str!("../inputs/20.txt");

    let (image_enhance_pattern, image) = INPUT.split_once("\n\n").unwrap();
    let image_enhance_pattern = &image_enhance_pattern.lines().fold(String::new(), |l,r|l + &r.to_string());
    let image = image.lines().map(|s|s.to_string()).collect::<Vec<String>>();
    println!("Enhancement pattern: {}", image_enhance_pattern);

    println!("Image: \n{}", image.join("\n"));

    let enhanced = enhance_n(&image_enhance_pattern, &image, 50);
    println!("Image enhanced: \n{}", enhanced.join("\n"));
    println!("Which is {} lit pixels", enhanced.join("\n").chars().map(|c|if c == '#' { 1 } else { 0 }).sum::<u64>());
}

fn enhance_n(enhance_pattern: &str, image: &Vec<String>, n: u32) -> Vec<String> {
    let mut enhanced_image = image_expand(&image, '.');
    for i in 0..n {
        let replacement_token = get_infinite_pixel(&enhance_pattern, i);
        enhanced_image = image_expand(&image_expand(&enhanced_image, replacement_token), replacement_token);
        enhanced_image = enhance(&enhance_pattern, &enhanced_image);
        enhanced_image = image_reduce(&enhanced_image);
    }
    enhanced_image
}

fn image_expand(image: &Vec<String>, replacement_token: char) -> Vec<String> {
    let image_width = image.iter().next().unwrap().len();
    let mut new_image = Vec::with_capacity(image.len()+2);
    new_image.push(iter::repeat(replacement_token).take(image_width+2).collect::<String>());
    for line in image {
        let mut new_line = String::with_capacity(image_width + 2);
        new_line.push(replacement_token);
        new_line += line;
        new_line.push(replacement_token);
        new_image.push(new_line);
    }
    new_image.push(iter::repeat(replacement_token).take(image_width+2).collect::<String>());
    new_image
}

fn image_reduce(image: &Vec<String>) -> Vec<String> {
    let mut new_image = Vec::with_capacity(image.len() - 2);
    let image_width = image.iter().next().unwrap().len();
    for line in image.iter().take(image.len() - 1).skip(1) {
        new_image.push(line.chars().take(image_width - 1).skip(1).collect());
    }
    new_image
}

fn get_infinite_pixel(enhance_pattern: &str, step: u32) -> char {
    let mut infinite_space = '0';
    for _ in 0..step {
        let pat: String = [infinite_space; 9].into_iter().collect();
        let index = usize::from_str_radix(&pat[..], 2).unwrap();
        infinite_space = if enhance_pattern.chars().nth(index).unwrap() == '#' { '1' } else { '0' };
    }
    if infinite_space == '1' { '#' } else { '.' }
}

fn enhance(pattern: &str, image: &Vec<String>) -> Vec<String> {
    let image_width = image.iter().next().unwrap().len();
    let mut enhanced_image: Vec<Vec<char>> = image.iter().map(|s|s.chars().collect()).collect();
    for (y, line) in enhanced_image.iter_mut().enumerate() {
        if y == 0 || image.len()-1 == y {
            continue;
        }
        for (x, pixel) in line.iter_mut().enumerate() {
            if x == 0 || image_width-1 == x {
                continue;
            }
            
            let indices = to_kernel_indices((x,y));
            let binary_pattern = indices.iter().map(|coord|{
                let (i, j) = coord.unwrap();
                let c = image.get(j).unwrap().chars().nth(i).unwrap();
                match c {
                    '#' => '1',
                    _ => '0'
                }
            }).collect::<String>();

            let replacement = pattern.chars().nth(usize::from_str_radix(&binary_pattern[..], 2).unwrap()).unwrap();
            *pixel = replacement;
        }
    }
    enhanced_image.iter().map(|v|v.iter().collect()).collect()
}

fn to_kernel_indices((i, j): (usize, usize)) -> [Option<(usize, usize)>; 9] {
    let mut out = [None; 9];
    let mut k = 0;
    for y in -1..2 {
        for x in -1..2 {
            out[k] = combine_option(safe_add(i, x), safe_add(j, y));
            k += 1;
        }
    }
    out
}

fn combine_option<T>(a: Option<T>, b: Option<T>) -> Option<(T, T)> {
    Some((a?, b?))
}

fn safe_add(a: usize, b: i64) -> Option<usize> {
    usize::try_from(i64::try_from(a).ok()? + b).ok()
}

#[test]
fn test_expand() {
    const INPUT: &str = "#..#.
#....
##..#
..#..
..###";
    let image = INPUT.lines().map(|s|s.to_string()).collect::<Vec<String>>();

    assert_eq!(&image_expand(&image, '.').join("\n")[..], ".......
.#..#..
.#.....
.##..#.
...#...
...###.
.......");
}

#[test]
fn test_enhance() {
    const INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    let (image_enhance_pattern, image) = INPUT.split_once("\n\n").unwrap();
    let image_enhance_pattern = &image_enhance_pattern.lines().fold(String::new(), |l,r|l + &r.to_string());
    let image = image.lines().map(|s|s.to_string()).collect::<Vec<String>>();

    assert_eq!(&enhance_n(&image_enhance_pattern, &image, 2).join("\n")[..], "...........
........#..
..#..#.#...
.#.#...###.
.#...##.#..
.#.....#.#.
..#.#####..
...#.#####.
....##.##..
.....###...
...........");
}

#[test]
fn test_reddit_data() {
    // Using data from another kind redditor:
    const INPUT: &str = include_str!("../inputs/20-b.txt");

    let (image_enhance_pattern, image) = INPUT.split_once("\n\n").unwrap();
    let image_enhance_pattern = &image_enhance_pattern.lines().fold(String::new(), |l,r|l + &r.to_string());
    let image = image.lines().map(|s|s.to_string()).collect::<Vec<String>>();

    let enhanced = enhance_n(&image_enhance_pattern, &image, 2);
    assert_eq!(enhanced.join("").chars().map(|c|if c == '#' { 1 } else { 0 }).sum::<u64>(), 5326);
}