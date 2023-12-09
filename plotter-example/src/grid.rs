use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new("images/grid.png", (700, 700)).into_drawing_area();
    root_area.fill(&WHITE)?;

    // Define the grid size
    let rows = 4;
    let cols = 4;

    let x: Vec<i32> = (0..=15).collect();
    let y: Vec<i32> = (16..=31).collect();

    // Define the values to be displayed in each cell
    let values = vec![
        "A1", "B1", "C1", "D1", "A2", "B2", "C2", "D2", "A3", "B3", "C3", "D3", "A4", "B4", "C4",
        "D4",
    ];
    let combined: Vec<i32> = x.iter().zip(y.iter()).map(|(a, b)| a + b).collect();
    println!("combined({}): {:?}", combined.len(), combined);

    // Iterate over each cell and draw the string
    for (idx, value) in combined.iter().enumerate() {
        println!("idx: {}, value: {}", idx, value);
        let row = idx / cols;
        let col = idx % cols;
        let value = value.to_string();

        let (x_range, y_range) = root_area.get_pixel_range();
        let width = (x_range.end - x_range.start) as u32;
        let height = (y_range.end - y_range.start) as u32;

        let cell = root_area
            .titled(&value, ("sans-serif", 20).into_font().color(&BLACK))?
            .shrink(
                (
                    col as u32 * (width / cols as u32) as u32,
                    row as u32 * height / rows as u32,
                ),
                (width / cols as u32, height / rows as u32),
            );

        cell.fill(&WHITE)?;
        cell.draw(&Text::new(value, (30, 30), ("sans-serif", 20).into_font()))?;
    }

    root_area.present()?;
    Ok(())
}
