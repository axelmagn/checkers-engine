const SQUARE_BIT_SIZE: u8 = 2;
const SQUARE_BIT_MASK: u64 = (1 << (SQUARE_BIT_SIZE-1)) - 1;
const BOARD_HEIGHT: u8 = 8;
const BOARD_WIDTH: u8 = 4; // every other square

// Assumes 8x8 board
struct BoardState {
    // encode board as 8 high, 4 wide, 4 state bit array organize by row-first
    // index, where bottom left corner is (0,0)
    state: u64,
}

impl BoardState {
    fn get(&self, row: u64, col: u64) -> u64 {
        let offset: u64 = (row * BOARD_WIDTH + col) * SQUARE_BIT_SIZE;
        (self.state >> offset) & SQUARE_BIT_MASK
    }

    fn set(&mut self, row: u64, col: u64, value: u64) {
        let mut value = value & SQUARE_BIT_MASK; // reduce value to its scope
        let offset: u64 = (row * BOARD_WIDTH + col) * SQUARE_BIT_SIZE;
        let clear_mask = !(SQUARE_BIT_MASK << offset);
        value = value << offset;
        self.state = (self.state & clear_mask) | value;
    }
}

#[test]
fn it_works() {
}
