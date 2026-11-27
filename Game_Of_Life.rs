//! Conways Game of Life in Rust (ohne externe Abhängigkeiten).
//!
//! Implementiert einen zweidimensionalen zellulären Automaten auf einem
//! toroidalen Gitter (periodische Randbedingungen) mit Moore-Nachbarschaft
//! für eine Besondere Lernleistung (BLL) in Mathematik zum Thema Emergenz.
//!
//! Kompilieren & Ausführen:
//!   rustc -O game_of_life.rs -o game_of_life && ./game_of_life --pattern glider_gun --steps 120
//! Tests ausführen:
//!   rustc --test game_of_life.rs -o game_of_life_test && ./game_of_life_test

use std::env;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Repräsentiert ein zweidimensionales Spielfeld für Conways Game of Life.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Grid {
    /// Erzeugt ein leeres Gitter mit gegebener Breite und Höhe.
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0 && height > 0, "Gitterdimensionen müssen positiv sein.");
        Self {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    /// Wandelt 2D-Koordinaten in einen linearen Index um (mit Wrap-Around / Torus-Topologie).
    fn index(&self, x: isize, y: isize) -> usize {
        let wx = x.rem_euclid(self.width as isize) as usize;
        let wy = y.rem_euclid(self.height as isize) as usize;
        wy * self.width + wx
    }

    /// Setzt den Zustand einer Zelle (true = lebendig, false = tot).
    pub fn set(&mut self, x: usize, y: usize, alive: bool) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.cells[idx] = alive;
        }
    }

    /// Gibt den Zustand einer Zelle zurück (mit periodischen Randbedingungen).
    pub fn get(&self, x: isize, y: isize) -> bool {
        let idx = self.index(x, y);
        self.cells[idx]
    }

    /// Zählt die lebenden Nachbarn in der 8er-Moore-Nachbarschaft von (x, y).
    pub fn live_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0u8;
        let ix = x as isize;
        let iy = y as isize;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if self.get(ix + dx, iy + dy) {
                    count += 1;
                }
            }
        }
        count
    }

    /// Berechnet die nächste Generation gemäß der lokalen Übergangsregel (B3/S23):
    /// - Lebende Zelle mit 2 oder 3 lebenden Nachbarn überlebt.
    /// - Tote Zelle mit genau 3 lebenden Nachbarn wird geboren.
    /// - Alle anderen Zellen sterben oder bleiben tot.
    pub fn step(&mut self) {
        let mut next = vec![false; self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let alive = self.cells[idx];
                let neighbors = self.live_neighbors(x, y);
                next[idx] = matches!((alive, neighbors), (true, 2) | (true, 3) | (false, 3));
            }
        }
        self.cells = next;
    }

    /// Zählt die Gesamtzahl aktuell lebender Zellen (makroskopische Messgröße).
    pub fn population(&self) -> usize {
        self.cells.iter().filter(|&&c| c).count()
    }

    /// Fügt ein Muster als Liste relativer (dx, dy)-Koordinaten ab (ox, oy) ein.
    pub fn place_pattern(&mut self, ox: usize, oy: usize, coords: &[(usize, usize)]) {
        for &(dx, dy) in coords {
            let x = (ox + dx) % self.width;
            let y = (oy + dy) % self.height;
            self.set(x, y, true);
        }
    }

    /// Befüllt das Gitter pseudozufällig (einfacher Xorshift-PRNG ohne externe Crates).
    pub fn randomize(&mut self, density_percent: u32, seed: u64) {
        let mut state = if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed };
        for cell in &mut self.cells {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            *cell = (state % 100) < density_percent as u64;
        }
    }

    /// Rendert das Gitter als String mit Unicode-Blockzeichen.
    pub fn render(&self) -> String {
        let mut out = String::with_capacity((self.width + 1) * self.height + 64);
        out.push('+');
        for _ in 0..self.width {
            out.push('-');
        }
        out.push_str("+\n");

        for y in 0..self.height {
            out.push('|');
            for x in 0..self.width {
                if self.cells[y * self.width + x] {
                    out.push('█');
                } else {
                    out.push(' ');
                }
            }
            out.push_str("|\n");
        }

        out.push('+');
        for _ in 0..self.width {
            out.push('-');
        }
        out.push('+');
        out
    }
}

/// Bekannte Startmuster zur Demonstration von Emergenz.
fn load_pattern(grid: &mut Grid, name: &str) {
    match name {
        "glider" => {
            // Kleinster Raumschiff-Typ (Periode 4, bewegt sich diagonal)
            let glider = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
            grid.place_pattern(2, 2, &glider);
        }
        "blinker" => {
            // Einfacher Oszillator (Periode 2)
            let blinker = [(0, 1), (1, 1), (2, 1)];
            grid.place_pattern(grid.width / 2 - 1, grid.height / 2 - 1, &blinker);
        }
        "r_pentomino" => {
            // Methusalem: 5 Zellen erzeugen über 1103 Generationen komplexe Strukturen
            let r_pentomino = [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)];
            grid.place_pattern(grid.width / 2 - 1, grid.height / 2 - 1, &r_pentomino);
        }
        "glider_gun" => {
            // Gosper Glider Gun (1970): Erstes bekanntes Muster mit unbegrenztem Wachstum
            let gun = [
                (24, 0),
                (22, 1), (24, 1),
                (12, 2), (13, 2), (20, 2), (21, 2), (34, 2), (35, 2),
                (11, 3), (15, 3), (20, 3), (21, 3), (34, 3), (35, 3),
                (0, 4), (1, 4), (10, 4), (16, 4), (20, 4), (21, 4),
                (0, 5), (1, 5), (10, 5), (14, 5), (16, 5), (17, 5), (22, 5), (24, 5),
                (10, 6), (16, 6), (24, 6),
                (11, 7), (15, 7),
                (12, 8), (13, 8),
            ];
            grid.place_pattern(2, 2, &gun);
        }
        _ => {
            let seed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(42);
            grid.randomize(28, seed);
        }
    }
}

fn print_help() {
    println!(
        "Conways Game of Life (Rust) - BLL Mathematik: Emergenz\n\n\
         VERWENDUNG:\n  game_of_life [OPTIONEN]\n\n\
         OPTIONEN:\n  \
           --pattern <NAME>   Startmuster: glider | blinker | r_pentomino | glider_gun | random (Standard: glider_gun)\n  \
           --width <N>        Gitterbreite in Zellen (Standard: 60)\n  \
           --height <N>       Gitterhöhe in Zellen (Standard: 25)\n  \
           --steps <N>        Anzahl der simulierten Generationen (Standard: 100)\n  \
           --delay <MS>       Pause zwischen Generationen in ms (Standard: 80)\n  \
           --no-clear         Terminal nicht vor jedem Frame leeren (gut für Logs)\n  \
           --help             Diese Hilfe anzeigen"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut pattern = String::from("glider_gun");
    let mut width: usize = 60;
    let mut height: usize = 25;
    let mut steps: usize = 100;
    let mut delay_ms: u64 = 80;
    let mut clear_screen = true;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--pattern" if i + 1 < args.len() => {
                pattern = args[i + 1].clone();
                i += 2;
            }
            "--width" if i + 1 < args.len() => {
                width = args[i + 1].parse().unwrap_or(60);
                i += 2;
            }
            "--height" if i + 1 < args.len() => {
                height = args[i + 1].parse().unwrap_or(25);
                i += 2;
            }
            "--steps" if i + 1 < args.len() => {
                steps = args[i + 1].parse().unwrap_or(100);
                i += 2;
            }
            "--delay" if i + 1 < args.len() => {
                delay_ms = args[i + 1].parse().unwrap_or(80);
                i += 2;
            }
            "--no-clear" => {
                clear_screen = false;
                i += 1;
            }
            _ => {
                eprintln!("Unbekannte Option: {}", args[i]);
                print_help();
                return;
            }
        }
    }

    let mut grid = Grid::new(width, height);
    load_pattern(&mut grid, &pattern);

    for gen in 0..=steps {
        if clear_screen {
            // ANSI-Escape-Sequenz: Bildschirm löschen & Cursor oben links setzen
            print!("\x1B[2J\x1B[1;1H");
        }
        println!(
            "Conways Game of Life | Muster: {} | Generation: {} | Lebende Zellen: {}",
            pattern,
            gen,
            grid.population()
        );
        println!("{}", grid.render());

        if gen < steps {
            grid.step();
            if delay_ms > 0 {
                thread::sleep(Duration::from_millis(delay_ms));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blinker_oscillator() {
        let mut grid = Grid::new(5, 5);
        grid.place_pattern(1, 2, &[(0, 0), (1, 0), (2, 0)]);
        let initial = grid.clone();

        grid.step();
        assert!(grid.get(2, 1));
        assert!(grid.get(2, 2));
        assert!(grid.get(2, 3));
        assert_eq!(grid.population(), 3);

        grid.step();
        assert_eq!(grid, initial);
    }

    #[test]
    fn test_glider_translation() {
        let mut grid = Grid::new(10, 10);
        grid.place_pattern(1, 1, &[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]);

        // Nach 4 Generationen bewegt sich ein Glider um (+1, +1)
        for _ in 0..4 {
            grid.step();
        }
        let mut expected = Grid::new(10, 10);
        expected.place_pattern(2, 2, &[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]);
        assert_eq!(grid, expected);
    }
}