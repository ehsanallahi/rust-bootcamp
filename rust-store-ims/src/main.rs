use dialoguer::{Input, Select};
use std::io;



struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}


struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    fn edit_product(&mut self, index: usize, new_product: Product) -> Result<(), &'static str> {
        if index >= self.products.len() {
            return Err("Index out of bounds");
        }
        self.products[index] = new_product;
        Ok(())
    }

    fn delete_product(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.products.len() {
            return Err("Index out of bounds");
        }
        self.products.remove(index);
        Ok(())
    }

    fn generate_report(&self) {
        println!("Inventory Report:");
        println!("{:<20} | {:<20} | {:<10} | {:<10}", "Name", "Description", "Price", "Quantity");
        for product in &self.products {
            println!("{:<20} | {:<20} | {:<10.2} | {:<10}", product.name, product.description, product.price, product.quantity);
        }
    }
}

fn main() {
    let mut inventory = Inventory { products: vec![] };

    loop {
        let choice = Select::new()
            .items(&["Add Product", "Edit Product", "Delete Product", "Generate Report", "Exit"])
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => {
                let name: String = Input::new().with_prompt("Enter product name:").interact_text().unwrap();
                let description: String = Input::new().with_prompt("Enter product description:").interact_text().unwrap();
                let price: f64 = Input::new().with_prompt("Enter product price:").interact().unwrap();
                let quantity: u32 = Input::new().with_prompt("Enter product quantity:").interact().unwrap();

                let product = Product {
                    name,
                    description,
                    price,
                    quantity,
                };

                inventory.add_product(product);
                println!("Product added successfully!");
            }
            1 => {
                if inventory.products.is_empty() {
                    println!("Inventory is empty. No products to edit.");
                    continue;
                }

                let index: usize = Select::new()
                    .items(&inventory.products.iter().map(|p| p.name.clone()).collect::<Vec<String>>())
                    .default(0)
                    .interact()
                    .unwrap();

                let name: String = Input::new().with_prompt("Enter new product name:").interact_text().unwrap();
                let description: String = Input::new().with_prompt("Enter new product description:").interact_text().unwrap();
                let price: f64 = Input::new().with_prompt("Enter new product price:").interact().unwrap();
                let quantity: u32 = Input::new().with_prompt("Enter new product quantity:").interact().unwrap();

                let new_product = Product {
                    name,
                    description,
                    price,
                    quantity,
                };

                inventory.edit_product(index, new_product).unwrap_or_else(|e| println!("Error: {}", e));
                println!("Product edited successfully!");
            }
            2 => {
                if inventory.products.is_empty() {
                    println!("Inventory is empty. No products to delete.");
                    continue;
                }

                let index: usize = Select::new()
                    .items(&inventory.products.iter().map(|p| p.name.clone()).collect::<Vec<String>>())
                    .default(0)
                    .interact()
                    .unwrap();

                inventory.delete_product(index).unwrap_or_else(|e| println!("Error: {}", e));
                println!("Product deleted successfully!");
            }
            3 => {
                if inventory.products.is_empty() {
                    println!("Inventory is empty. No report to generate.");
                } else {
                    inventory.generate_report();
                }
            }
            4 => {
                println!("Exiting the program.");
                break;
            }
            _ => unreachable!(),
        }
    }
}
