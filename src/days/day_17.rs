use std::ops::Neg;

struct Projectile {
    x: i32,
    y: i32,
    x_velocity: i32,
    y_velocity: i32,
    start_x_vel: i32,
    start_y_vel: i32,
}

pub fn solve_day_17() {
    //target area: x=150..171, y=-129..-70
    let lo_x = 150;
    let hi_x = 171;
    let lo_y = -129;
    let hi_y = -70;

    let mut solutions = vec![];

    for x in -200..200 {
        for y in -200..200 {
            let mut p = Projectile {
                x: 0,
                y: 0,
                x_velocity: x,
                y_velocity: y,
                start_x_vel: x,
                start_y_vel: y,
            };

            let sol = p.get_steps_until_out_of_bounds(lo_x, hi_x, lo_y, hi_y);
            if sol.is_some() {
                solutions.push(sol.unwrap());
            }
        }
    }

    println!("Found {} solutions", solutions.len());
}

impl Projectile {
    fn step(&mut self) {
        self.x = self.x + self.x_velocity;
        self.y = self.y + self.y_velocity;

        let new_x_velocity = if self.x_velocity == 0 {
            0
        } else if self.x_velocity > 0 {
            self.x_velocity - 1
        } else {
            self.x_velocity + 1
        };

        self.x_velocity = new_x_velocity;
        self.y_velocity = self.y_velocity - 1;
    }

    fn get_steps_until_out_of_bounds(&mut self, lo_x: i32, hi_x: i32, lo_y: i32, hi_y: i32) -> Option<Vec<(i32, i32)>> {
        let mut steps = Vec::new();

        while self.x <= hi_x && self.y >= lo_y {
            steps.push((self.x, self.y));
            self.step();
        }

        let is_correct = steps.iter().any(|(x, y)| *x >= lo_x && *x <= hi_x && *y <= hi_y && *y >= lo_y);
        if is_correct {
            println!("-----------------");
            println!("Solution found with x_velocity: {} and y_velocity: {}", self.start_x_vel, self.start_y_vel);
            println!("Max y: {}", steps.iter().max_by_key(|(x, y)| *y).unwrap().1);
            //print_trajectory(lo_x, lo_y, hi_x, hi_y, &steps);
            Some(steps)
        } else {
            None
        }
    }
}

fn print_trajectory(lo_x: i32, lo_y: i32, hi_x: i32, hi_y: i32, steps: &[(i32, i32)]) {
    let mut max_y = steps.iter().map(|(_, y)| *y).max().unwrap();
    if lo_y.abs() > max_y {
        max_y = lo_y.abs();
    }
    for y in max_y.neg()..=max_y {
        for x in 0..=hi_x {
            if steps.iter().any(|(x_p, y_p)| *x_p == x && *y_p == y) {
                print!("O");
            } else if lo_x <= x && x <= hi_x && lo_y <= y && y <= hi_y {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}