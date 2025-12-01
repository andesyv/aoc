use num::Integer; // Useful funcions, but weird implementations
use std::rc::Rc;

#[derive(Debug)]
enum NumberType {
    Regular(u64),
    Pair(Rc<Number>),
}

use NumberType::{Pair, Regular};

#[derive(Debug)]
struct Number {
    l: NumberType,
    r: NumberType,
}

#[derive(Debug)]
enum ReductionResult {
    Explotion(u64, u64),
    Split,
    Nothing,
}

impl Number {
    fn reduce(&mut self, depth: i64) -> ReductionResult {
        use ReductionResult::{Explotion, Nothing, Split};
        let mut should_explode = false;
        if let Pair(childref) = &mut self.l {
            if let Some(child) = Rc::get_mut(childref) {
                if 2 < depth {
                    if let (Regular(_), Regular(_)) = (&child.l, &child.r) {
                        should_explode = true;
                    }
                }

                if !should_explode {
                    match child.reduce(depth + 1) {
                        Explotion(le, re) => {
                            // Pass the right value right and the left value upwards
                            if re != 0 {
                                match &mut self.r {
                                    Pair(childref) => {
                                        pass_value_left(Rc::get_mut(childref).unwrap(), re)
                                    }
                                    Regular(n) => *n += re,
                                }
                            }
                            return Explotion(le, 0);
                        }
                        Split => return Split,
                        Nothing => (),
                    }
                }
            }
        }

        if should_explode {
            if let Explotion(a, b) = self.explode() {
                return Explotion(a, b);
            }
        }

        if let Pair(childref) = &mut self.r {
            if let Some(child) = Rc::get_mut(childref) {
                if 2 < depth {
                    if let (Regular(_), Regular(_)) = (&child.l, &child.r) {
                        should_explode = true;
                    }
                }

                if !should_explode {
                    match child.reduce(depth + 1) {
                        Explotion(le, re) => {
                            // Pass the left value left and the right value upwards
                            if le != 0 {
                                match &mut self.l {
                                    Pair(childref) => {
                                        pass_value_right(Rc::get_mut(childref).unwrap(), le)
                                    }
                                    Regular(n) => *n += re,
                                }
                            }
                            return Explotion(0, re);
                        }
                        Split => return Split,
                        Nothing => (),
                    }
                }
            }
        }

        if should_explode {
            if let Explotion(a, b) = self.explode() {
                return Explotion(a, b);
            }
        }

        // Split:
        self.split()
    }

    fn explode(&mut self) -> ReductionResult {
        use ReductionResult::{Explotion, Nothing};
        let mut explode_maybe = None;

        if let Pair(childref) = &mut self.l {
            if let Some(child) = Rc::get_mut(childref) {
                if let (Regular(a), Regular(b)) = (&child.l, &child.r) {
                    explode_maybe = Some(Explotion(*a, 0));
                    if let Regular(n) = &mut self.r {
                        *n += *b;
                    }
                }
            }
        }
        if let Some(expl) = explode_maybe {
            self.l = Regular(0);
            return expl;
        }

        if let Pair(childref) = &mut self.r {
            if let Some(child) = Rc::get_mut(childref) {
                if let (Regular(a), Regular(b)) = (&child.l, &child.r) {
                    explode_maybe = Some(Explotion(0, *b));
                    if let Regu
                    self.l = Regular(*a);
                }
            }
        }
        if let Some(expl) = explode_maybe {
            self.r = Regular(0);
            return expl;
        }

        Nothing
    }

    fn split(&mut self) -> ReductionResult {
        use ReductionResult::{Nothing, Split};
        if let Regular(n) = self.l {
            self.l = Pair(Rc::new(Number {
                l: Regular(Integer::div_floor(&n, &2)),
                r: Regular(Integer::div_ceil(&n, &2)),
            }));
            return Split;
        }
        if let Regular(n) = self.r {
            self.r = Pair(Rc::new(Number {
                l: Regular(Integer::div_floor(&n, &2)),
                r: Regular(Integer::div_ceil(&n, &2)),
            }));
            return Split;
        }

        Nothing
    }
}

fn pass_value_left(number: &mut Number, value: u64) {
    match &mut number.l {
        Regular(n) => *n += value,
        Pair(childref) => pass_value_left(Rc::get_mut(childref).unwrap(), value)
    }
}

fn pass_value_right(number: &mut Number, value: u64) {
    match &mut number.r {
        Regular(n) => *n += value,
        Pair(childref) => pass_value_right(Rc::get_mut(childref).unwrap(), value)
    }
}

fn main() {
    // [[[[[9,8],1],2],3],4]
    let mut num = Number {
        l: Pair(Rc::new(Number {
            l: Pair(Rc::new(Number {
                l: Pair(Rc::new(Number {
                    l: Pair(Rc::new(Number {
                        l: Regular(9),
                        r: Regular(8),
                    })),
                    r: Regular(1),
                })),
                r: Regular(2),
            })),
            r: Regular(3),
        })),
        r: Regular(4),
    };

    println!("Number reduction returned {:?}, and it is now: {:?}", num.reduce(0), num);
}
