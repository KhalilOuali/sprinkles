use rand::seq::SliceRandom;
use std::error::Error;
use std::io::Write;
use termcolor::{Buffer, Color, ColorSpec, WriteColor};

const WARM: [Color; 3] = [Color::Red, Color::Yellow, Color::Magenta];
const COLD: [Color; 3] = [Color::Green, Color::Blue, Color::Cyan];

/// Randomly changes the color of each character in `string` and write it to `buffer`.
/// 
/// # Params
/// * `warm`: Allow warm colors.
/// * `cold`: Allow cold colors.
/// * `shiny`: Use brighter (intense) colors.
/// * `bold`: Make text bold.
/// * `italic`: Make text italic.
/// * `white`: Allow white.
/// 
/// # Errors
/// * All colors are disabled (`warm`=`cold`=`white`=`false`).
/// * Setting a color fails. See [`termcolor::WriteColor::set_color()`].
/// * Writing to the buffer fails. See [`std::io::Write::write()`].
pub fn sprinklize(
    string: &str,
    buffer: &mut Buffer,
    warm: bool,
    cold: bool,
    shiny: bool,
    bold: bool,
    italic: bool,
    white: bool,
) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    let mut colors = Vec::new();
    if warm {
        colors.extend_from_slice(&WARM);
    }
    if cold {
        colors.extend_from_slice(&COLD);
    }
    if white {
        colors.push(Color::White);
    }

    let mut spec = ColorSpec::new();
    spec.set_bold(bold);
    spec.set_italic(italic);
    spec.set_intense(shiny);

    for c in string.chars() {
        buffer.set_color(spec.set_fg(Some(*colors.choose(&mut rng).ok_or("colors is empty")?)))?;
        write!(buffer, "{c}")?;
    }
    Ok(())
}
