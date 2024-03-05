use std::collections::HashMap;
use std::io::{self};

struct Product {
    name : String,
    description: String,
    price: f64,
    quantity: i32,
}

impl Product {
    fn edit_name(&mut self, new_name: String) {
        self.name = new_name.trim().to_uppercase();
    }

    fn edit_price(&mut self, new_price: f64) {
        self.price = new_price;
    }

    fn edit_description(&mut self, new_desc: String) {
        self.description = new_desc.trim().to_string();
    }

    fn edit_quantity(&mut self, new_quantity: i32) {
        self.quantity = new_quantity;
    }
}

struct Inventory {
    products: HashMap<String, Product>,
}

impl Inventory {
    fn add_product(&mut self, name: String, description: String, price: f64, quantity: i32) {
        let product = Product { name: name.clone(), description, price, quantity };
        self.products.insert(name.clone(), product);
        println!("Product '{}' added to inventory.", name);
    }

    fn edit_product(&mut self, name: &str) {
        if let Some(product) = self.products.get_mut(name) {

            println!("-------------------");
            println!("'Edit Menu'");
            println!("-------------------");
            println!("** 1: Edit Name");
            println!("** 2: Edit Price");
            println!("** 3: Edit Description");
            println!("** 4: Edit Quantity");
            println!("** 5: Return to Main Menu");
            let item_choice = get_integer_input();

            match item_choice {
                1 => {
                    let mut new_name = String::new();

                    println!("Enter New Name: ");
                    io::stdin().read_line(&mut new_name).expect("Failed to read line");

                    product.edit_name(new_name);
                },
                2 => {
                    println!("Enter New Price of the product: ");
                    let new_price = get_float_input();

                    product.edit_price(new_price);
                },
                3 => {
                    let mut new_desc = String::new();

                    println!("Enter New Description of the product: ");
                    io::stdin().read_line(&mut new_desc).expect("Failed to read line");

                    product.edit_description(new_desc);
                },
                4 => {
                    println!("Enter New Quantity of the product: ");
                    let new_quantity = get_integer_input();

                    product.edit_quantity(new_quantity);
                },
                5 => {return;},
                _ => {},
            }

            println!("Product '{}' updated.", name);
        } else {
            println!("Product '{}' not found.", name);
        }
    }

    fn delete_product(&mut self, name: &str) {
        loop {
            println!("*** 1. Delete one single quantity of this Product");
            println!("*** 2. Delete all stock of this Product");
            println!("*** 3. Return to Main Menu");
            let item_choice = get_integer_input();
            match item_choice {
                1 => {
                    if let Some(product) = self.products.get_mut(name) {
                        product.quantity -= 1;
                        if product.quantity <= 0 {
                            self.products.remove(name);
                            println!("Product '{}' deleted from inventory.", name);
                        }
                    }
                    return;
                },

                2 => {
                    if let Some(_) = self.products.remove(name) {
                        println!("Product '{}' deleted from inventory.", name);
                    } else {
                        println!("Product '{}' not found.", name);
                    }
                    return;
                },

                3 => {
                    return;
                }

                _ => {
                    println!("Incorrect choice, Select a valid choice:");
                }
            }
        }
    }

    fn generate_report(&self) {
        if self.products.is_empty() {
            println!("Inventory is Empty! :<");
            return;
        }
        println!("-------------------");
        println!("'Inventory Report'");
        println!("-------------------");
        for (name, product) in &self.products {
            println!("Name: {}", name);
            println!("Description: {}", product.description);
            println!("Price: ${:.2}", product.price);
            println!("Quantity: {}", product.quantity);
            if product.quantity <= 10 {
                println!("Low Stock!\nUpdate ASAP");
            }
            println!("-------------------");
        }
    }
}

fn main() {

    if !admin_check() {
        println!("Please enter valid credentials:<\nShutting down the System!");
        return;
    }

    let mut inventory = Inventory { products: HashMap::new() };

    let mut menu_choice: i32;

    println!("\tRust Inventory Management\n----------------------------------------");

    loop {
        println!("----------------------------------------
** 1: Add Product
** 2: Edit Product Info
** 3: Delete Product
** 4: Generate Inventory Report
** 5: Close Inventory");
        println!("----------------------------------------");

        println!("Enter your choice:");
        menu_choice = get_integer_input();  
        match menu_choice {
            1 => {
                let mut name = String::new();
                let price: f64;
                let mut description = String::new();
                let mut quantity: i32;

                println!("Enter Product Name:");
                io::stdin().read_line(&mut name).expect("Failed to read line");

                println!("Enter Price:");
                price = get_float_input();

                println!("Enter Description:");
                io::stdin().read_line(&mut description).expect("Failed to read line");

                loop {
                    println!("Enter Quantity:");
                    quantity = get_integer_input();
                    if quantity <= 0 {
                        println!("Quantity can't be zero or less!\nRe-enter:");
                    }
                    else {
                        break;
                    }
                }

                inventory.add_product(name.trim().to_uppercase(), description.trim().to_string(), price, quantity);
                println!("----------------------------------------");
            },

            2 => {
                let mut prod_name = String::new();
                println!("Enter Product Name: ");
                io::stdin().read_line(&mut prod_name).expect("Failed to read line");
                prod_name = prod_name.trim().to_uppercase().to_string();
                inventory.edit_product(&prod_name);
            },

            3 => {
                let mut name = String::new();

                println!("Enter Product Name: ");
                io::stdin().read_line(&mut name).expect("Failed to read line");

                inventory.delete_product(&name.trim().to_uppercase());
            },

            4 => {
                inventory.generate_report();
            },

            5 => {
                println!("Shutting the inventory");
                return;
            }

            _ => println!("Select a valid choice!"),
        }
    }

}

fn get_integer_input() -> i32 {
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid value e.g., integer."),
        }
    }
}

fn get_float_input() -> f64 {
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid value e.g., float."),
        }
    }
}

fn admin_check() -> bool {


    let user_check_val : String = String::from("admin");
    let pass_check_val : String = String::from("admin123");

    println!("\n----------------------------------------\n\tRust Inventory Management\n----------------------------------------");
    println!("\t      'Login Panel'\n");

    let mut user_name = String::new();
    println!("Enter a valid Username: ");
    io::stdin().read_line(&mut user_name)
    .expect("Failed to read line");

    let mut user_pass = String::new();
    println!("Enter a valid Password: ");
    io::stdin().read_line(&mut user_pass)
    .expect("Failed to read line");

    if user_name.trim() == user_check_val && user_pass.trim() == pass_check_val {
        true
    }

    else {
        false
    }
}




