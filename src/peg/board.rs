
pub struct Board {
    cols: u8,
    rows: u8,
    slots: Vec<Slot>,
}

impl Board {
    pub fn c(&self) -> u8 {
        self.cols
    }

    pub fn r(&self) -> u8 {
        self.rows
    }

    pub fn slot(&self, col: u8, row: u8) -> Slot {
        let i = self.to_index(col, row);
        self.slots[i]
    }

    fn to_index(&self, col: u8, row: u8) -> usize {
        ((row * self.cols) + col) as usize
    }

    fn to_coords(&self, index: usize) -> Coords {
        let row: usize = index / self.cols as usize;
        let col: usize = index % self.cols as usize;

        (col, row)
    }
    
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Slot {
    Empty,
    Filled(Option<Tile>),
}

pub type Coords = (usize, usize);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Tile {}

pub struct BoardBuilder {
    cols: u8,
    rows: u8,
    slots: Vec<Slot>, // You can initialize this with a default if needed
}

impl BoardBuilder {
    // Create a new BoardBuilder
    pub fn new(cols: u8, rows: u8) -> Self {
        BoardBuilder {
            cols,
            rows,
            slots: vec![Slot::Empty; (cols * rows) as usize],
        }
    }

    fn to_index(&self, col: u8, row: u8) -> usize {
        ((row * self.cols) + col) as usize
    }

    fn to_coords(&self, index: usize) -> Coords {
        let row: usize = index / self.cols as usize;
        let col: usize = index % self.cols as usize;

        (col, row)
    }

    // Method to add a slot
    pub fn slot(&mut self, slot: Slot, col: u8, row: u8) -> &mut Self {
        let i = self.to_index(col, row);
        self.slots.insert(i, slot);
        self
    }

    // Build the Board
    pub fn build(self) -> Board {
        Board {
            cols: self.cols,
            rows: self.rows,
            slots: self.slots,
        }
    }
}
