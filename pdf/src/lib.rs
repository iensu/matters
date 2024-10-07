use matters_lib::Problem;
use printpdf::{BuiltinFont, Mm, PdfDocument};

const A4: (f32, f32) = (210.0, 297.0);
const START_POSITION: f32 = 270.0;
const BREAD_TEXT_SIZE: f32 = 10.0;
const INDENT: f32 = 20.0;
const LINE_HEIGHT: f32 = BREAD_TEXT_SIZE;
const MAX_ROWS: usize = 60;

/// Generates a math problem PDF document.
///
/// # Errors
///
/// Returns error if unable to generate PDF document.
pub fn generate_pdf(problems: &[Problem]) -> Result<Vec<u8>, std::io::Error> {
    let (doc, page1, layer1) = PdfDocument::new("Mattepappret", Mm(A4.0), Mm(A4.1), "Layer 1");
    let font = doc.add_builtin_font(BuiltinFont::Courier).map_err(|_err| {
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to set builtin PDF font!")
    })?;
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let mut y = START_POSITION;
    let mut x = INDENT;

    for (index, problem) in problems.iter().enumerate() {
        if index == MAX_ROWS / 2 {
            y = START_POSITION;
            x = INDENT + 100.0;
        }
        if index == 15 || index == 45 {
            y -= LINE_HEIGHT;
        }

        y -= LINE_HEIGHT * 0.8;

        current_layer.use_text(
            format!("{:>2}.", index + 1),
            BREAD_TEXT_SIZE * 0.6,
            Mm(x),
            Mm(y),
            &font,
        );

        current_layer.use_text(
            format!("{problem} = _______"),
            BREAD_TEXT_SIZE,
            Mm(x + 7.0),
            Mm(y),
            &font,
        );
    }

    doc.save_to_bytes().map_err(|_err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to turn PDF document into bytes!",
        )
    })
}
