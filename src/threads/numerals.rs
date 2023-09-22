use std::env;

use svg::Document;
use svg::node::element::{Path, Rectangle};
use svg::node::element::path::{Command, Data, Position};

use crate::threads::numerals::Operation::{Forward, Home, Noop, TurnLeft, TurnRight};
use crate::threads::numerals::Orientation::{East, North, South, West};

fn numeral_main() {
    let args = env::args().collect::<Vec<String>>();
    let input = args.get(1).unwrap();
    let default = format!("{}.svg", input);
    let save_to = args.get(2).unwrap_or(&default);

    let operations = parse(input);
    let path_data = convert(&operations);
    let document = generate_svg(path_data);
    svg::save(save_to, &document).unwrap()
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(isize),
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    North,
    East,
    West,
    South,
}


const HEIGHT: isize = 400;
const WIDTH: isize = HEIGHT;
const HOME_X: isize = HEIGHT / 2;
const HOME_Y: isize = WIDTH / 2;

const STROKE_WIDTH: usize = 5;


fn parse(input: &str) -> Vec<Operation> {
    let mut steps = Vec::<Operation>::new();
    for byte in input.bytes() {
        let step = match byte {
            b'0' => Home,
            b'1'..=b'9' => {
                let distance = (byte - 0x30) as isize;
                Forward(distance * HEIGHT / 10)
            }
            b'a' | b'b' | b'c' => TurnLeft,
            b'd' | b'e' | b'f' => TurnRight,
            _ => Noop(byte as isize)
        };
        steps.push(step);
    };
    steps
}

fn convert(operations: &Vec<Operation>) -> Vec<Command> {
    let mut turtle = Artist::new();
    let mut path_data: Vec<Command> = vec![];

    let start_at_home = Command::Move(
        Position::Absolute, (HOME_X, HOME_Y).into(),
    );
    path_data.push(start_at_home);
    //for op in operations {
    let mut op1: Vec<Command> = operations.iter().map(|op| {
        match *op {
            Forward(distance) => turtle.forward(distance),
            TurnLeft => turtle.turn_left(),
            TurnRight => turtle.turn_right(),
            Home => turtle.home(),
            Noop(byte) => {
                eprintln!("warning: illegal byte encoutnered: {:?}", byte);
            }
        };

        let path_segment: Command = Command::Line(
            Position::Absolute, (turtle.x, turtle.y).into(),
        );
        turtle.wrap();
        path_segment
    }).collect();
    path_data.append(&mut op1);
    path_data
}

fn generate_svg(path_data: Vec<Command>) -> Document {
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .set("fill","#ffffff");

    let border = background
        .clone()
        .set("fill-opacity", "0.0")
        .set("stroke", "#cccccc")
        .set("stroke-width", 3 * STROKE_WIDTH);

    let sketch = Path::new()
        .set("fill", "none")
        .set("stroke", "#2f2f2f")
        .set("stroke-width", STROKE_WIDTH)
        .set("stroke-opacity", "0.9")
        .set("d", Data::from(path_data));

    let document = Document::new()
        .set("viewBox", (0,0, HEIGHT, WIDTH))
        .set("height", HEIGHT)
        .set("width", WIDTH)
        .set("style", "style=\"outline: 5px solid #800000;\"")
        .add(background)
        .add(sketch)
        .add(border);
    document
}
#[derive(Debug)]
struct Artist {
    x: isize,
    y: isize,
    heading: Orientation,
}

/*macro_rules! reset {
    ($a: expr) => {

    }
}*/
impl Artist {
    fn new() -> Artist {
        Artist {
            heading: North,
            x: HOME_X,
            y: HOME_Y,
        }
    }

    fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }

    fn forward(&mut self, distance: isize) {
        match self.heading {
            North => self.y += distance,
            South => self.y -= distance,
            West => self.x += distance,
            East => self.x -= distance,
        }
    }

    fn turn_right(&mut self) {
        self.heading = match self.heading {
            North => East,
            South => West,
            West => North,
            East => South
        }
    }

    fn turn_left(&mut self) {
        self.heading = match self.heading {
            North => West,
            South => East,
            West => South,
            East => North
        }
    }
    fn reset_x(&mut self, orient: Orientation) {
        self.x = HOME_X;
        self.heading = orient;
    }
    fn reset_y(&mut self, orient: Orientation) {
        self.y = HOME_Y;
        self.heading = orient;
    }

    fn wrap(&mut self) {
        self.reset_x(if self.x < 0 { West } else { East });
        self.reset_x(if self.y < 0 { North } else { South });
    }
}