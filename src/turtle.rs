use unsvg::{get_end_coordinates, Image, COLORS};

pub struct Turtle {
    pub coords: (f32, f32),
    pub colour: usize,
    pen_state: PenState,
    pub heading: i32,
}

#[derive(Debug, PartialEq)]
pub enum PenState {
    Up,
    Down,
}

impl Turtle {
    pub fn new(coords: (f32, f32)) -> Self {
        Turtle {
            coords,
            colour: 7,
            pen_state: PenState::Up,
            heading: 0,
        }
    }

    pub fn change_penstate(&mut self, new_value: PenState) {
        self.pen_state = new_value;
    }

    pub fn change_colour(&mut self, new_value: usize) {
        self.colour = new_value;
    }

    pub fn change_x(&mut self, new_value: f32) {
        self.coords.0 = new_value;
    }

    pub fn change_y(&mut self, new_value: f32) {
        self.coords.1 = new_value;
    }

    pub fn change_heading(&mut self, new_value: i32) {
        self.heading = new_value;
    }

    pub fn draw(&mut self, image: &mut Image, mut direction: i32, mut distance: f32) {
        // invert the direction if required
        direction += self.heading;
        if distance < 0.0 {
            distance *= -1.0;
            direction += 180;
        }
        let (x, y) = self.coords;
        // draw the line and move the turtle OR just move the turtle
        if self.pen_state == PenState::Down {
            self.coords = image
                .draw_simple_line(x, y, direction, distance, COLORS[self.colour])
                .unwrap();
        } else {
            self.coords = get_end_coordinates(x, y, direction, distance);
        }
    }

    pub fn turn(&mut self, turn: i32) {
        self.heading += turn;
    }
}
