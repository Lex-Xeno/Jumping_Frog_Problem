#![allow(dead_code)]
use indoc::indoc;
use rand::random;

/// Represents a frog
/// # Fields
/// - `id` is an ID given to each frog to distinguish them from one and other if multiple frogs are used
/// - `position` represents the lilly pad that the frog is currently on
/// - `jumps` is the number of jumps the frog has made
/// - `distance` - is the number of the furthest lilly pad that the frog has jumped to
#[derive(getset::Getters)]
pub struct Frog {
    id: Option<usize>,
    position: isize,
    jumps: usize,
    furthest: isize,
    heading: FrogHeading,
}

pub trait FrogTrait: Send + 'static {
    fn start(id: Option<usize>) -> Self;
    fn jump(&mut self) -> Option<()>;
    fn result(&self) -> String;
}

impl FrogTrait for Frog {
    /// Creates a new frog, sets its heading, and gives it an ID number
    fn start(id: Option<usize>) -> Self {
        let heading = if random() {
            FrogHeading::Left
        } else {
            FrogHeading::Right
        };

        Frog {
            id,
            position: 1,
            jumps: 1,
            furthest: 1,
            heading,
        }
    }
    /// Makes the frog jump
    fn jump(&mut self) -> Option<()> {
        self.position += if random() { -1 } else { 1 };
        self.jumps += 1;

        if self.furthest < self.position {
            self.furthest = self.position
        }

        if self.position == 0 {
            Some(())
        } else {
            None
        }
    }
    /// Returns the frog's data in the for of a csv entry
    fn result(&self) -> String {
        let distance = match self.heading {
            FrogHeading::Left => -self.furthest,
            FrogHeading::Right => self.furthest,
        };

        if let Some(id) = self.id {
            format!("{},{},{}\n", id, self.jumps, distance)
        } else {
            format!("{},{}\n", self.jumps, distance)
        }
    }
}

/// Enums that represent which side of the center the frog jumped to
#[derive(PartialEq, Debug)]
pub enum FrogHeading {
    Left,
    Right,
}

impl std::fmt::Display for FrogHeading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrogHeading::Left => write!(f, "left"),
            FrogHeading::Right => write!(f, "right"),
        }
    }
}
impl std::clone::Clone for FrogHeading {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for FrogHeading {}
impl std::fmt::Debug for Frog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = if let Some(id) = self.id {
            format!("Some({})", id)
        } else {
            "None".to_string()
        };

        write!(
            f,
            indoc! {
            "frog:
                id: {},
                position: {},
                jumps: {},
                distance: {},
                heading: {},"
            },
            id, self.position, self.jumps, self.furthest, self.heading,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump() {
        let mut frog = Frog {
            id: None,
            position: 2,
            jumps: 2,
            furthest: 2,
            heading: FrogHeading::Right,
        };

        frog.jump();

        assert!(frog.position == 3 || frog.position == 1);
        assert!(frog.furthest == 3 || frog.furthest == 2);
        assert_eq!(frog.jumps, 3);
        assert_eq!(frog.heading, FrogHeading::Right);
    }

    #[test]
    fn result() {
        let no_id = Frog {
            id: None,
            position: 1,
            jumps: 3,
            furthest: 2,
            heading: FrogHeading::Left,
        };

        let has_id = Frog {
            id: Some(3),
            position: 1,
            jumps: 3,
            furthest: 2,
            heading: FrogHeading::Right,
        };

        assert_eq!(no_id.result(), "3,-2\n");
        assert_eq!(has_id.result(), "3,3,2\n");
    }
}
