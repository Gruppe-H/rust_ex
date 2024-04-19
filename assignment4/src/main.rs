// Define a struct `Product` to represent a product
#[derive(Debug)] // Implement Debug trait for easier printing
struct Product {
    id: u32,             // Unique identifier for the product
    name: String,        // Name of the product
    price: f64,          // Price of the product
    quantity_in_stock: u32, // Quantity of the product in stock
}

impl Product {
    // Constructor method to create a new product instance
    fn new(id: u32, name: String, price: f64, quantity_in_stock: u32) -> Self {
        Product { id, name, price, quantity_in_stock }
    }
}

// Define a struct `Inventory` to represent an inventory of products
struct Inventory {
    products: Vec<Product>, // Vector to store products in the inventory
}

impl Inventory {
    // Constructor method to create a new inventory instance
    fn new() -> Self {
        Inventory { products: Vec::new() } // Initialize an empty vector of products
    }

    // Method to add a product to the inventory
    fn add_product(&mut self, product: Product) {
        self.products.push(product); // Push the product to the vector of products
    }

    // Method to remove a product from the inventory by ID
    fn remove_product(&mut self, id: u32) -> Option<Product> {
        if let Some(index) = self.products.iter().position(|product| product.id == id) {
            // If product with given ID is found, remove it from the vector and return it
            Some(self.products.remove(index))
        } else {
            // If product with given ID is not found, return None
            None
        }
    }

    // Method to update the quantity of a product in the inventory
    fn update_product_quantity(&mut self, id: u32, new_quantity: u32) -> Option<()> {
        if let Some(product) = self.products.iter_mut().find(|product| product.id == id) {
            // If product with given ID is found, update its quantity
            product.quantity_in_stock = new_quantity;
            Some(()) // Return Some(()) indicating successful update
        } else {
            // If product with given ID is not found, return None
            None
        }
    }
}

fn main() {
    let mut inventory = Inventory::new(); // Create a new inventory instance

    // Add some products to the inventory
    inventory.add_product(Product::new(1, String::from("Product A"), 10.99, 100));
    inventory.add_product(Product::new(2, String::from("Product B"), 20.49, 50));
    inventory.add_product(Product::new(3, String::from("Product C"), 15.75, 75));

    println!("Inventory: {:?}", inventory.products);

    // Remove a product from the inventory by ID
    if let Some(removed_product) = inventory.remove_product(2) {
        println!("Removed product: {:?}", removed_product);
    } else {
        println!("Product with ID 2 not found in inventory");
    }

    // Update the quantity of a product in the inventory
    if let Some(_) = inventory.update_product_quantity(1, 150) {
        println!("Updated quantity of product with ID 1");
    } else {
        println!("Product with ID 1 not found in inventory");
    }

    println!("Inventory after update: {:?}", inventory.products);
}