enum SyntaxStatus {
    Perfect,
    Error(char),
    Incomplete(Vec<char>),
}

fn main() {
    // const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
    //             [(()[<>])]({[<{<<[]>>(
    //             {([(<{}[<>[]}>{[]{[(<()>
    //             (((({<>}<{<{<>}{[]{[]{}
    //             [[<[([]))<([[{}[[()]]]
    //             [{[{({}]{}}([{[{{{}}([]
    //             {<[[]]>}<{[{[{[]{()[[[]
    //             [<(<(<(<{}))><([]([]()
    //             <{([([[(<>()){}]>(<<{{
    //             <{([{{}}[<[[[<>{}]]]>[]]";
    const INPUT: &str = include_str!("../inputs/10.txt");

    let lines: Vec<&str> = INPUT.lines().map(|s| s.trim()).collect();
    let syntax_results: Vec<SyntaxStatus> = lines.into_iter().map(syntax_check).collect();
    let corrupt_score = syntax_results
        .iter()
        .map(|status| {
            if let SyntaxStatus::Error(illegal) = status {
                return match illegal {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => 0,
                };
            }
            0
        })
        .sum::<u32>();
    println!("Total syntax error score: {}", corrupt_score);

    let mut incomplete_scores: Vec<u64> = syntax_results
        .iter()
        .filter_map(|status| {
            if let SyntaxStatus::Incomplete(rest) = status {
                Some(rest)
            } else {
                None
            }
        })
        .map(|xs| {
            xs.into_iter()
                .map(|x| {
                    return match x {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => 0,
                    };
                })
                .fold(0, |l, r| l * 5 + r)
        })
        .collect();
    incomplete_scores.sort();

    println!("Middle incomplete score is {}", incomplete_scores.get(incomplete_scores.len() / 2).unwrap());
}

fn get_matching(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => c
    }
}

fn syntax_check(line: &str) -> SyntaxStatus {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if let Some(top) = stack.last() {
                    if c != get_matching(*top) {
                        return SyntaxStatus::Error(c);
                    }
                    stack.pop();
                }
            }
            _ => (),
        }
    }

    if stack.is_empty() {
        SyntaxStatus::Perfect
    } else {
        SyntaxStatus::Incomplete(stack.into_iter().rev().map(get_matching).collect())
    }
}
