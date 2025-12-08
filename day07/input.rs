use std::fmt::Display;

use common::grid::Grid;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Element {
    #[default]
    Empty,
    Start,
    Splitter,
    Beam,
}
impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Empty => '.'.fmt(f),
            Element::Start => 'S'.fmt(f),
            Element::Splitter => '^'.fmt(f),
            Element::Beam => '|'.fmt(f),
        }
    }
}
impl TryFrom<char> for Element {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            'S' => Ok(Self::Start),
            '^' => Ok(Self::Splitter),
            '|' => Ok(Self::Beam),
            _ => Err(format!("Unrecognized element, '{value}'")),
        }
    }
}

pub type Input = Grid<Element>;

pub fn parse_input(input_str: &str) -> Input {
    let width = input_str.find("\n").expect("Couldn't find linefeed");

    let data = input_str
        .lines()
        .flat_map(|line| line.chars().map(|c| Element::try_from(c).unwrap()))
        .collect();

    Grid::from_vec(data, width)
}
