// Benchmark for IronCalc performance with large datasets
// Tests: 500K rows × 18 columns = 9 million cells

use ironcalc_base::Model;
use std::time::Instant;

fn main() {
    println!("=== IronCalc Large Dataset Performance Test ===\n");
    
    // Test parameters - quick test with 10K rows first
    let rows = 10_000;
    let cols = 18;
    let total_cells = rows * cols;
    
    println!("Test parameters:");
    println!("  Rows: {:>12}", format_num(rows));
    println!("  Columns: {:>9}", cols);
    println!("  Total cells: {:>6}", format_num(total_cells));
    println!();

    // Create model
    println!("[1/4] Creating model...");
    let start = Instant::now();
    let mut model = Model::new_empty("benchmark", "en", "UTC").expect("Failed to create model");
    println!("  Model created in {:?}", start.elapsed());

    // Insert data
    println!("[2/4] Inserting {} cells...", format_num(total_cells));
    let start = Instant::now();
    
    let sheet = 0;
    for row in 1..=rows as i32 {
        for col in 1..=cols as i32 {
            let value = format!("{}", (row * col) as f64 / 100.0);
            let _ = model.set_user_input(sheet, row, col, value);
        }
        
        if row % 100_000 == 0 {
            println!("  {} rows inserted...", format_num(row as usize));
        }
    }
    
    let insert_time = start.elapsed();
    let cells_per_sec = total_cells as f64 / insert_time.as_secs_f64();
    println!("  Inserted in {:?} ({:.0} cells/sec)", insert_time, cells_per_sec);

    // Evaluate
    println!("[3/4] Evaluating model...");
    let start = Instant::now();
    model.evaluate();
    println!("  Evaluated in {:?}", start.elapsed());

    // Read back sample cells
    println!("[4/4] Reading sample cells...");
    let start = Instant::now();
    let samples = [
        (1, 1),
        (100_000, 9),
        (250_000, 15),
        (500_000, 18),
    ];
    
    for (row, col) in samples {
        let value = model.get_formatted_cell_value(sheet as u32, row, col).unwrap_or_default();
        println!("  Cell R{}C{}: {}", row, col, value);
    }
    println!("  Read in {:?}", start.elapsed());

    // Memory estimate (rough)
    println!("\n=== Summary ===");
    println!("Total cells: {}", format_num(total_cells));
    println!("Insert time: {:?}", insert_time);
    println!("Throughput: {:.0} cells/sec", cells_per_sec);
}

fn format_num(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, c);
    }
    result
}
