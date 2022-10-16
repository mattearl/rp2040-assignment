//!
//! This file defines the SmallBall game. SmallBall is a game where you control a small ball
//! on a small screen via an mpu sensor. The goal is to move the ball around the screen to
//! visit all goals on the screen in the minimum amount of time. The game keeps track of the
//! lowest score achieved.
//!

use crate::{
    config::{
        ANGLE_THRESHOLD, BALL_DELTA, BALL_LOCATION, BALL_SIZE, GOAL_LOCATIONS, GOAL_SIZE,
        SCREEN_OUTLINE_SIZE, SCREEN_OUTLINE_TOP_LET, X_MAX, X_MIN, Y_MAX, Y_MIN,
    },
    math::intersects,
};
use embedded_graphics::prelude::{Point, Size};
use heapless::Vec;

/// The mode the game is in.
#[cfg_attr(test, derive(Debug, PartialEq))]
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
#[cfg_attr(test, derive(Debug, PartialEq))]
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

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::State;
    use crate::{
        config::{ANGLE_THRESHOLD, BALL_DELTA, SCREEN_OUTLINE_SIZE, SCREEN_OUTLINE_TOP_LET},
        smallball::Mode,
    };
    use embedded_graphics::prelude::Point;

    #[test]
    fn transition_from_intro_to_play_test() {
        // GIVEN game state in intro mode
        let mut state = State::new();
        assert_eq!(*state.mode(), Mode::Intro);
        // WHEN update is called
        state.update(&0.0, &0.0);
        // THEN game transitions to play mode
        assert_eq!(*state.mode(), Mode::Play);
    }

    #[test]
    fn transition_from_play_to_over_test() {
        // GIVEN game state in play mode
        let mut state = game_state_in_play_mode();

        // GIVEN all goals dead
        for goal in state.goals.iter_mut() {
            goal.alive = false;
        }

        // WHEN update is called
        state.update(&0.0, &0.0);

        // THEN game transitions to over mode and the low score is updated
        assert_eq!(*state.mode(), Mode::Over);
        assert_eq!(state.low_score(), 2);
    }

    #[test]
    fn transition_from_over_to_play_test() {
        // GIVEN game state in over mode
        let mut state = game_state_in_play_mode();
        for goal in state.goals.iter_mut() {
            goal.alive = false;
        }
        state.update(&0.0, &0.0);
        assert_eq!(*state.mode(), Mode::Over);

        // WHEN update is called
        state.update(&0.0, &0.0);

        // THEN game transitions to play mode
        assert_eq!(*state.mode(), Mode::Play);
    }

    #[test]
    fn ball_moves_right_test() {
        // GIVEN game state in play mode
        let mut state = game_state_in_play_mode();

        // WHEN the roll is positive and above threshold
        state.update(&0.0, &(ANGLE_THRESHOLD + 0.1));

        // THEN the ball moves to the right
        assert_eq!(state.ball().location(), ball_location_delta(BALL_DELTA, 0));
    }

    #[test]
    fn ball_moves_left_test() {
        // GIVEN game state in play mode
        let mut state = game_state_in_play_mode();

        // WHEN the roll is negative and above threshold
        state.update(&0.0, &-(ANGLE_THRESHOLD + 0.1));

        // THEN the ball moves to the left
        assert_eq!(state.ball().location(), ball_location_delta(-BALL_DELTA, 0));
    }

    #[test]
    fn ball_moves_up_test() {
        // GIVEN game state in play mode
        let mut state = game_state_in_play_mode();

        // WHEN the pitch is positive and above threshold
        state.update(&(ANGLE_THRESHOLD + 0.1), &0.0);

        // THEN the ball moves to up
        assert_eq!(state.ball().location(), ball_location_delta(0, -BALL_DELTA));
    }

    #[test]
    fn ball_moves_down_test() {
        // GIVEN game state in play mode
        let mut state = game_state_in_play_mode();

        // WHEN the pitch is negative and above threshold
        state.update(&-(ANGLE_THRESHOLD + 0.1), &0.0);

        // THEN the ball moves to down
        assert_eq!(state.ball().location(), ball_location_delta(0, BALL_DELTA));
    }

    #[test]
    fn ball_moves_diagonally_test() {
        // GIVEN game state in play mode
        let mut state = game_state_in_play_mode();

        // WHEN the pitch and roll are both positive and above threshold
        state.update(&(ANGLE_THRESHOLD + 0.1), &(ANGLE_THRESHOLD + 0.1));

        // THEN the ball moves to diagonally
        assert_eq!(
            state.ball().location(),
            ball_location_delta(BALL_DELTA, -BALL_DELTA)
        );
    }

    #[test]
    fn ball_stays_put_test() {
        // GIVEN game state in play mode
        let mut state = game_state_in_play_mode();

        // WHEN the pitch and roll are both positive and below threshold
        state.update(&(ANGLE_THRESHOLD - 0.1), &(ANGLE_THRESHOLD - 0.1));

        // THEN the ball stays put
        assert_eq!(state.ball().location(), State::initial_ball().location());
    }

    #[test]
    fn ball_visits_goal_test() {
        // GIVEN game state in play mode with first goal alive
        let mut state = game_state_in_play_mode();
        assert!(state.goals[0].alive);
        assert_eq!(state.goals_alive().len(), State::initial_goals().len());

        // WHEN the ball moves to visit the goal
        state.ball.location = State::initial_goals()[0].location();
        state.update(&0.0, &0.0);

        // THEN the goal is dead
        assert!(!state.goals[0].alive);
        assert_eq!(state.goals_alive().len(), State::initial_goals().len() - 1);
    }

    #[test]
    fn update_score_test() {
        // GIVEN game state in play mode
        let mut state = game_state_in_play_mode();
        assert_eq!(state.score(), 1);
        // WHEN update is called
        state.update(&0.0, &0.0);
        // THEN the score increments
        assert_eq!(state.score(), 2);
        // WHEN update is called
        state.update(&0.0, &0.0);
        // THEN the score increments
        assert_eq!(state.score(), 3);
    }

    #[test]
    fn screen_outline_test() {
        let state = game_state_in_play_mode();
        assert_eq!(state.screen_outline_top_left(), SCREEN_OUTLINE_TOP_LET);
        assert_eq!(state.screen_outline_size(), SCREEN_OUTLINE_SIZE);
    }

    fn ball_location_delta(delta_x: i32, delta_y: i32) -> Point {
        Point::new(
            State::initial_ball().location.x + delta_x,
            State::initial_ball().location.y + delta_y,
        )
    }

    fn game_state_in_play_mode() -> State {
        let mut state = State::default();
        assert_eq!(state.score(), 0);
        assert_eq!(*state.mode(), Mode::Intro);
        state.update(&0.0, &0.0);
        assert_eq!(*state.ball(), State::initial_ball());
        assert_eq!(state.score(), 1);
        state
    }
}
