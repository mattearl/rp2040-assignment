//!
//! In this file the configuration for the SmallBall game is defined.   The version
//! of SmallBall defined below is configured for a screen of size 128x64 and
//! relies on user control input from an mpu sensor's pitch and roll measurements.
//!

use embedded_graphics::prelude::{Point, Size};

// Delay time between game modes in milliseconds.
pub const DELAY_MS: u32 = 3000;

// The top left point for the rectangle that outlines the entire screen.
pub const FULL_SCREEN_OUTLINE_TOP_LET: Point = Point::new(0, 0);

// The size of the rectangle that outlines the entire screen.
pub const FULL_SCREEN_OUTLINE_SIZE: Size = Size::new(127, 63);

// The name of the game displayed on the splash screen.
pub const GAME_NAME: &str = "Small Ball";

// The location of the screen where the game name is drawn
pub const GAME_NAME_LOCATION: Point = Point::new(35, 4);

// The size of the shapes drawn on the splash screen
pub const SPLASH_SCREEN_SHAPE_SIZE: Size = Size::new_equal(16);

// The locations of the shapes drawn on the splash screen
pub const SPLASH_SCREEN_SHAPE_LOCATIONS: [Point; 3] =
    [Point::new(20, 25), Point::new(52, 25), Point::new(88, 25)];

// The text to draw for the score during game play and game over
pub const SCORE_TEXT: &str = "score: ";

// the location of the score text
pub const SCORE_LOCATION: Point = Point::new(1, 0);

// The text to draw on the game over screen
pub const GAME_OVER_TEXT: &str = "Game Over";

// The location of the game over text
pub const GAME_OVER_LOCATION: Point = Point::new(38, 2);

// the location of the score text during game over
pub const GAME_OVER_SCORE_LOCATION: Point = Point::new(2, 20);

// The text to draw for the low score during game over
pub const LOW_SCORE_TEXT: &str = "low score: ";

// the location of the low score text during game over
pub const GAME_OVER_LOW_SCORE_LOCATION: Point = Point::new(2, 40);

// the boundaries of the game space
pub const X_MIN: i32 = 0;
pub const X_MAX: i32 = 118;
pub const Y_MIN: i32 = 10;
pub const Y_MAX: i32 = 56;

// the top left coordinate of the screen outline during game play
pub const SCREEN_OUTLINE_TOP_LET: Point = Point::new(0, 9);

// the size of the screen outline during game play
pub const SCREEN_OUTLINE_SIZE: Size = Size::new(127, 55);

// the pitch/roll angle threshold, above which the ball is moved in the corresponding direction
pub const ANGLE_THRESHOLD: f32 = 0.6;

// the distance the ball moves each loop if pitch/roll angle is above threshold
pub const BALL_DELTA: i32 = 2;

// the initial location of each goal
pub const GOAL_LOCATIONS: [Point; 4] = [
    Point::new(10, 12),
    Point::new(100, 50),
    Point::new(50, 20),
    Point::new(10, 50),
];

// the initial location of the ball
pub const BALL_LOCATION: Point = Point::new(88, 20);

/// The size of each goal
pub const GOAL_SIZE: u32 = 8;

// The size of the ball
pub const BALL_SIZE: u32 = 8;
