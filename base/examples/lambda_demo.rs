//! Quick demo of LAMBDA and Spilling features
//! Run with: cargo run --example lambda_demo

use ironcalc_base::Model;

fn main() {
    println!("=== IronCalc LAMBDA & Spilling Demo ===\n");
    
    // Create a new model
    let mut model = Model::new_empty("demo", "en", "UTC").unwrap();
    
    // Demo 1: Simple LAMBDA call
    println!("1. Simple LAMBDA:");
    model.set_user_input(0, 1, 1, "=LAMBDA(x, x*2)(5)".to_string());
    model.evaluate();
    println!("   =LAMBDA(x, x*2)(5) → {}\n", model.get_formatted_cell_value(0, 1, 1).unwrap());
    
    // Demo 2: LAMBDA as value in cell
    println!("2. LAMBDA stored in cell:");
    model.set_user_input(0, 2, 1, "=LAMBDA(a, b, a+b)".to_string());
    model.set_user_input(0, 2, 2, "=A2(10, 20)".to_string());
    model.evaluate();
    println!("   A2: =LAMBDA(a, b, a+b) → {}", model.get_formatted_cell_value(0, 2, 1).unwrap());
    println!("   B2: =A2(10, 20) → {}\n", model.get_formatted_cell_value(0, 2, 2).unwrap());
    
    // Demo 3: Array Spilling
    println!("3. Array Spilling:");
    model.set_user_input(0, 3, 1, "={100, 200, 300}".to_string());
    model.evaluate();
    println!("   A3: ={{100, 200, 300}} → {}", model.get_formatted_cell_value(0, 3, 1).unwrap());
    println!("   B3: (spilled) → {}", model.get_formatted_cell_value(0, 3, 2).unwrap());
    println!("   C3: (spilled) → {}\n", model.get_formatted_cell_value(0, 3, 3).unwrap());
    
    // Demo 4: Spill Collision
    println!("4. Spill Collision:");
    model.set_user_input(0, 4, 2, "BLOCKED".to_string()); // Block B4
    model.set_user_input(0, 4, 1, "={1, 2}".to_string());
    model.evaluate();
    println!("   B4: BLOCKED");
    println!("   A4: ={{1, 2}} → {} (should be #SPILL!)\n", model.get_formatted_cell_value(0, 4, 1).unwrap());
    
    println!("=== Demo Complete ===");
}
