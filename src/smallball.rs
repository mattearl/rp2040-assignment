//!
//! This file defines the SmallBall game. SmallBall is a game where you control a small ball
//! on a small screen via an mpu sensor. The goal is to move the ball around the screen to
//! visit all goals on the screen in the minimum amount of time. The game keeps track of the
//! lowest score achieved.
//!
//! The version of SmallBall defined below is configured for a screen of size 128x64 and
//! relies on user control input from an mpu sensor's pitch and roll measurements.
//!

use crate::math::intersects;
use embedded_graphics::prelude::{Point, Size};
use heapless::Vec;

// the boundaries of the game space
const X_MIN: i32 = 0;
const X_MAX: i32 = 118;
const Y_MIN: i32 = 10;
const Y_MAX: i32 = 56;

// the top left coordinate of the screen outline during game play
const SCREEN_OUTLINE_TOP_LET: Point = Point::new(0, 9);

// the size of the screen outline during game play
const SCREEN_OUTLINE_SIZE: Size = Size::new(127, 55);

// the pitch/roll angle threshold, above which the ball is moved in the corresponding direction
const ANGLE_THRESHOLD: f32 = 0.6;

// the distance the ball moves each loop if pitch/roll angle is above threshold
const BALL_DELTA: i32 = 2;

// the initial location of each goal
const GOAL_LOCATIONS: [Point; 4] = [
    Point::new(10, 10),
    Point::new(100, 50),
    Point::new(50, 20),
    Point::new(10, 50),
];

// the initial location of the ball
const BALL_LOCATION: Point = Point::new(88, 20);

/// The size of each goal
const GOAL_SIZE: u32 = 8;

// The size of the ball
const BALL_SIZE: u32 = 8;

/// The mode the game is in.
pub enum Mode {
    /// Introduce the game with a splash screen
    Intro,
    /// Actively playing the game
    Play,
    /// The game is over, show the score and the low score
    Over,
}

/// The Ball is the entity that the user controls on the screen
/// trying to visit goals as quickly as possible.
pub struct Ball {
    /// the current location of this ball
    location: Point,
}

impl Ball {
    /// Return a new ball.
    /// # Arguments
    /// * `location` - the initial location of the ball
    fn new(location: Point) -> Self {
        Ball { location }
    }

    /// Return the current location of this ball.
    pub fn location(&self) -> Point {
        self.location
    }

    /// Return the size of this ball.
    pub fn size(&self) -> u32 {
        BALL_SIZE
    }
}

/// A goal is a box on the screen that the ball needs to visit.
#[derive(Debug)]
pub struct Goal {
    /// The current location of the goal.
    location: Point,
    /// The goal is alive if it has yet to be visited by the ball.
    alive: bool,
}

impl Goal {
    /// Return a new goal.
    /// # Arguments
    /// * `location` - the initial location of the goal
    fn new(location: Point) -> Self {
        Goal {
            location,
            alive: true,
        }
    }

    /// Return the current location of the goal.
    pub fn location(&self) -> Point {
        self.location
    }

    /// Return the size of the goal.
    pub fn size(&self) -> u32 {
        GOAL_SIZE
    }
}

/// The SmallBall game state.
pub struct State {
    /// the current score
    score: i32,
    /// the lowest score achieved in a completed game
    low_score: i32,
    /// the current state of the ball
    ball: Ball,
    /// the current state of the goals
    goals: Vec<Goal, 4>,
    /// the current game mode
    mode: Mode,
}

impl State {
    /// Return a new game State with default initial state.
    pub fn new() -> Self {
        State {
            score: 0,
            low_score: i32::max_value(),
            ball: State::initial_ball(),
            goals: State::initial_goals(),
            mode: Mode::Intro,
        }
    }

    /// Return the current state of the ball.
    pub fn ball(&self) -> &Ball {
        &self.ball
    }

    /// Return the current score.
    pub fn score(&self) -> i32 {
        self.score
    }

    /// Return the lowest score achieved in a completed game.
    pub fn low_score(&self) -> i32 {
        self.low_score
    }

    /// Return the current game mode.
    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    /// Return the default initial ball state.
    fn initial_ball() -> Ball {
        Ball::new(BALL_LOCATION)
    }

    /// Return the default initial goal states.
    fn initial_goals() -> Vec<Goal, 4> {
        let mut goals = Vec::new();
        for location in GOAL_LOCATIONS {
            goals.push(Goal::new(location)).unwrap();
        }
        goals
    }

    /// Update the state of the game based on the latest pitch and roll input from the mpu.
    /// # Arguments
    /// * `pitch` - the pitch reading from the mpu sensor
    /// * `roll` - the roll reading from the mpu sensor
    pub fn update(&mut self, pitch: &f32, roll: &f32) {
        self.update_ball(pitch, roll);
        self.update_score();
        self.update_mode();
        self.update_goals();
    }

    // Update the game mode.
    fn update_mode(&mut self) {
        match self.mode {
            Mode::Intro => self.mode = Mode::Play,
            Mode::Play => {
                if self.goals.iter().all(|goal| !goal.alive) {
                    self.mode = Mode::Over;
                    if self.score < self.low_score {
                        self.low_score = self.score;
                    }
                }
            }
            Mode::Over => {
                self.mode = Mode::Play;
                self.score = 0;
                self.ball = State::initial_ball();
                self.goals = State::initial_goals();
            }
        }
    }

    /// Update the game score.
    fn update_score(&mut self) {
        // score is based on time so the longer it takes to reach each goal,
        // the higher your score. Lower scores are better.
        self.score += 1;
    }

    /// Update the ball state based on mpu pitch and roll input.
    /// # Arguments
    /// * `pitch` - the pitch reading from the mpu sensor
    /// * `roll` - the roll reading from the mpu sensor
    fn update_ball(&mut self, pitch: &f32, roll: &f32) {
        let mut x = self.ball.location.x;
        let mut y = self.ball.location.y;

        if *pitch > ANGLE_THRESHOLD && y > Y_MIN {
            // if the sensor is pitched down then the ball moves up the screen until it hits the top boundary
            y -= BALL_DELTA;
        } else if *pitch < -ANGLE_THRESHOLD && y < Y_MAX {
            // if the sensor is pitched up then the ball moves down the screen until it hits the bottom boundary
            y += BALL_DELTA;
        }

        if *roll > ANGLE_THRESHOLD && x < X_MAX {
            // if the sensor is rolled up then the ball moves right on the screen until it hits the right boundary
            x += BALL_DELTA;
        } else if *roll < -ANGLE_THRESHOLD && x > X_MIN {
            // if the sensor is rolled down then the ball moves left on the screen until it hits the left boundary
            x -= BALL_DELTA;
        }

        self.ball = Ball::new(Point::new(x, y));
    }

    /// Update the goal states based on whether or not they have been newly visited by the ball.
    /// Once visited the goal is dead.
    fn update_goals(&mut self) {
        for goal in self.goals.iter_mut() {
            if goal.alive
                && intersects(
                    goal.location,
                    goal.size(),
                    self.ball.location,
                    self.ball.size(),
                )
            {
                goal.alive = false;
            }
        }
    }

    /// Return the top left point that defines the screen outline rectangle.
    pub fn screen_outline_top_left(&self) -> Point {
        SCREEN_OUTLINE_TOP_LET
    }

    /// Return the size of the screen outline rectangle.
    pub fn screen_outline_size(&self) -> Size {
        SCREEN_OUTLINE_SIZE
    }

    /// Return the vector of goals that are still alive.
    pub fn goals_alive(&self) -> Vec<&Goal, 4> {
        let mut goals_alive = Vec::new();

        for goal in self.goals.iter() {
            if goal.alive {
                goals_alive.push(goal).unwrap();
            }
        }
        goals_alive
    }
}
