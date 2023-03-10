// Imports for rectangle window and graphics context
use piston_window::{rectangle, Context, G2d};
// Import for color components.
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

/** Converts a coordinate to a float
 * and multiplies it by block size.
 * @param game_coord The given coordinate.
 * @return the scaled coordinate 
 */
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

/**
 * Draws a block of the snake.
 * @param color Color of the block.
 * @param x X coordinate of the block.
 * @param y Y coordinate of the block.
 * @param con Pointer to context struct.
 * @param g Mutable 2D graphics buffer.
 */
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);
    // Call rectangle function
    rectangle(
        color, 
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

/**
 * Draws the a rectangle of given dimensions.
 * @param color Color of the area.
 * @param x X coordinate of the block.
 * @param y Y coordinate of the block.
 * @param width Width of the rectangle.
 * @param height Height of the rectangle.
 * @param con Pointer to context struct.
 * @param g Mutable 2D graphics buffer.
 */
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);
    // Call rectangle function
    rectangle(
        color, 
        [gui_x, gui_y, BLOCK_SIZE  * (width as f64), BLOCK_SIZE * (height as f64)],
        con.transform,
        g,
    );
}


