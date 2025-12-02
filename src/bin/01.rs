use std::fs::read_to_string;
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Left(u16),
    Right(u16),
}

impl Rotation {
    fn amount(&self) -> u16 {
        match self {
            Rotation::Left(rot) => *rot,
            Rotation::Right(rot) => *rot,
        }
    }

    fn reduce(&mut self) -> u16 {
        let revolutions = self.amount() / 100;
        match self {
            Rotation::Left(rot) => {
                *self = Rotation::Left(*rot % 100);
            }
            Rotation::Right(rot) => {
                *self = Rotation::Right(*rot % 100);
            }
        }

        revolutions
    }
}

impl AddAssign<Rotation> for u16 {
    fn add_assign(&mut self, rhs: Rotation) {
        *self = match rhs {
            Rotation::Left(rot) => *self + 100 - (rot % 100),
            Rotation::Right(rot) => *self + (rot % 100),
        } % 100;
    }
}

impl TryFrom<&str> for Rotation {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (dir, rot_str) = value.trim().split_at(1);
        let rot = u16::from_str(rot_str).map_err(|_| "Unable to parse rotation")?;

        match dir {
            "L" => Ok(Rotation::Left(rot)),
            "R" => Ok(Rotation::Right(rot)),
            _ => Err("Unknown direction"),
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Rotation> {
    input
        .lines()
        .map(|l| Rotation::try_from(l).expect("Unable to parse input"))
}

fn part_1(rotation_iter: impl Iterator<Item = Rotation>) -> u16 {
    let mut password = 0;
    let mut dial = 50;

    for rotation in rotation_iter {
        dial += rotation;

        if dial == 0 {
            password += 1
        }
    }

    password
}

fn part_2(rotation_iter: impl Iterator<Item = Rotation>) -> u16 {
    let mut password = 0;
    let mut dial = 50;

    for mut rotation in rotation_iter {
        password += rotation.reduce();

        if dial == 0 {
            password += 1;
        } else {
            match rotation {
                Rotation::Left(rot) => {
                    if dial < rot {
                        password += 1;
                    }
                }
                Rotation::Right(rot) => {
                    if 100 < dial + rot {
                        password += 1;
                    }
                }
            }
        }

        dial += rotation;
    }

    if dial == 0 { password + 1 } else { password }
}

fn main() {
    let input = read_to_string("input/01.txt").unwrap();

    let rotation_iter = parse_input(&input);
    let password_1 = part_1(rotation_iter);
    println!("Part 1: {}", password_1);

    let rotation_iter = parse_input(&input);
    let password_2 = part_2(rotation_iter);
    println!("Part 2: {}", password_2);
}

#[cfg(test)]
mod tests {
    use crate::*;

    static TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn example_part_1() {
        let input = TEST_INPUT.to_string();
        let rotation_iter = parse_input(&input);
        let password = part_1(rotation_iter);

        assert_eq!(password, 3);
    }

    #[test]
    fn example_part_2() {
        let input = TEST_INPUT.to_string();
        let rotation_iter = parse_input(&input);
        let password = part_2(rotation_iter);

        assert_eq!(password, 6);
    }
}
