pub struct Position {
    x: i32,
    y: i32
}

pub impl Position {
    // Create new Position with arguments
    fn new(x: i32, y: i32) -> Position {
        Position {x, y}
    }

    // Set position to new arguments
    fn set_pos(x: i32, y: i32) -> void {
        self.x = x;
        self.y = y;
    }

    // Set X coordinate to new argument
    fn set_x(x: i32) -> void {
        self.x = x;
    }

    // Set Y coordinate to new argument
    fn set_y(y: i32) -> void {
        self.y = y;
    }
}
