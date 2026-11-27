# Conways Game of Life in Rust

Eine Simulation von **Conways Game of Life** in Rust (ohne externe Crates) als praktischer Programmierteil einer **Besonderen Lernleistung (BLL)** im Fach Mathematik zum Thema **Emergenz**.

## Mathematischer Hintergrund

Conways Game of Life (1970) ist ein zweidimensionaler zellulärer Automat auf einem Gitter $G = \mathbb{Z}_W \times \mathbb{Z}_H$ mit periodischen Randbedingungen (Torus-Topologie). Jede Zelle $(x, y)$ besitzt den Zustand $s_{x,y}(t) \in \{0, 1\}$ (tot oder lebendig).

Obwohl jede Zelle ausschließlich lokal auf die Anzahl lebender Nachbarn $N_{x,y}(t) \in \{0, \dots, 8\}$ in ihrer **Moore-Nachbarschaft** reagiert (Regel **B3/S23**):

1. **Geburt (Birth):** Eine tote Zelle ($s = 0$) mit genau $N = 3$ lebenden Nachbarn wird in $t+1$ lebendig.
2. **Überleben (Survival):** Eine lebende Zelle ($s = 1$) mit $N \in \{2, 3\}$ lebenden Nachbarn bleibt lebendig.
3. **Tod (Unter-/Überbevölkerung):** In allen anderen Fällen ist die Zelle in $t+1$ tot.

Aus dieser rein lokalen, deterministischen Vorschrift entstehen makroskopisch komplexe Strukturen wie Oszillatoren (*Blinker*), Raumschiffe (*Glider*), langlebige Methusalems (*R-Pentomino*) oder unbegrenzt wachsende Generatoren (*Gosper Glider Gun*) – ein klassisches Beispiel für **Emergenz** und Turing-Vollständigkeit.

## Voraussetzungen

- [Rust](https://www.rust-lang.org/tools/install) (`rustc` oder `cargo`)
- Keine externen Bibliotheken erforderlich (reine Rust-Standardbibliothek `std`)

## Installation & Ausführung

### Direkt mit `rustc`

```bash
rustc -O game_of_life.rs -o game_of_life
./game_of_life --pattern glider_gun --steps 120
```

### Mit `cargo` (optional für ein vollständiges Repository)

Falls du das Projekt als Cargo-Paket strukturieren möchtest:

```bash
cargo init --name game_of_life
cp game_of_life.rs src/main.rs
cargo run --release -- --pattern glider_gun --steps 120
```

### Unit-Tests ausführen

Die korrekte Implementierung der Übergangsregeln (z. B. Periodizität des *Blinkers* und diagonale Translation des *Gliders* nach 4 Schritten) ist durch Unit-Tests abgesichert:

```bash
rustc --test game_of_life.rs -o game_of_life_test && ./game_of_life_test
```

## CLI-Optionen

| Option | Beschreibung | Standardwert |
| :--- | :--- | :--- |
| `--pattern <NAME>` | Startmuster (`glider`, `blinker`, `r_pentomino`, `glider_gun`, `random`) | `glider_gun` |
| `--width <N>` | Breite des Gitters in Zellen | `60` |
| `--height <N>` | Höhe des Gitters in Zellen | `25` |
| `--steps <N>` | Anzahl der simulierten Generationen | `100` |
| `--delay <MS>` | Wartezeit zwischen zwei Generationen in Millisekunden | `80` |
| `--no-clear` | Terminal zwischen Frames nicht leeren | deaktiviert |
| `--help` | Hilfetext anzeigen | – |

## Beispiele

```bash
# Methusalem R-Pentomino beobachten (erzeugt aus 5 Zellen über 1100 Generationen Aktivität)
./game_of_life --pattern r_pentomino --width 80 --height 30 --steps 250 --delay 50

# Zufällige Startkonfiguration
./game_of_life --pattern random --width 70 --height 25 --steps 200
```

## Lizenz

Dieses Projekt steht unter der [MIT-Lizenz](https://opensource.org/licenses/MIT).
