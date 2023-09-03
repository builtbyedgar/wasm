use std::time::Duration;

struct Cell {
  channel: u8,
  note: u8,
  active: bool,
}

impl Cell {
  fn new(note: u8) -> Cell {
      Cell {
          channel: 0,
          note: note,
          active: false,
      }
  }
  
  fn bang(&mut self) -> bool {
      false
  }
}

// 
pub struct Sequencer {
  cells: Vec<Cell>,
  notes: Vec<(u8, u8)>,
  ctr: u32,
  pub tick: Duration,
}

pub fn new(bpm: f32) -> Sequencer {
  Sequencer {
    cells: vec![],
    notes: vec![],
    ctr: 0,
    tick: Duration::from_millis((60_000.0 / (bpm * 24.0)) as u64),
  }
}

impl Sequencer {
  fn note_off(&mut self) {
    println!("note off");
  }
  
  pub fn tick(&mut self) {
    self.ctr = (self.ctr + 1) % 24;

    self.note_off();

    // note on
    let x = rand::random::<usize>() % self.cells.len();
    let x = &self.cells[x];

    // let note: u8 = 60;
    // let event = MidiMessage { ... }
    self.notes.push((x.channel, x.note));

    // Send the event to midi
  }
}

pub fn from_string() -> Sequencer {
    let y = 12 * 3;

    Sequencer {
        cells: vec![Cell::new(y + 0)],
        notes: vec![],
        ctr: 0,
        tick: Duration::from_millis(1000 / 24),
    }
}