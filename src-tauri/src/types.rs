pub struct Position {
    x: i32,
    y: i32
}

pub impl Position {
    // Create new Position with arguments
    fn new(x: i32, y: i32) -> Position {
        Self {x: (x), y: (y)}
    }

    // Set position to new arguments
    fn set_pos(self, x: i32, y: i32) -> void {
        self.x = x;
        self.y = y; 
    }

    // Set X coordinate to new argument
    fn set_x(self, x: i32) -> void {
        self.x = x;
    }

    // Set Y coordinate to new argument
    fn set_y(self, y: i32) -> void {
        self.y = y;
    }
}
