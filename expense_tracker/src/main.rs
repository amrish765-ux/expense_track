use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{self, Write, Read};

#[derive(Serialize, Deserialize, Debug)]
struct Expense {
    description: String,
    amount: f64,
}

fn main() {
    let mut expenses: Vec<Expense> = vec![];
    let filename = "expenses.json";

    // Load existing expenses if the file exists
    if let Ok(expenses_from_file) = fs::read_to_string(filename) {
        expenses = serde_json::from_str(&expenses_from_file).unwrap_or_else(|_| {
            println!("Could not read existing expenses, starting fresh.");
            vec![]
        });
    }

    loop {
        println!("\nExpense Tracker");
        println!("1. Add Expense");
        println!("2. List Expenses");
        println!("3. Save Expenses");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim() {
            "1" => add_expense(&mut expenses),
            "2" => list_expenses(&expenses),
            "3" => save_expenses(&expenses, filename),
            "4" => break,
            _ => println!("Invalid choice! Please choose again."),
        }
    }
}

fn add_expense(expenses: &mut Vec<Expense>) {
    let mut description = String::new();
    let mut amount_str = String::new();

    println!("Enter expense description: ");
    io::stdin().read_line(&mut description).expect("Failed to read line");

    println!("Enter expense amount: ");
    io::stdin().read_line(&mut amount_str).expect("Failed to read line");

    let amount: f64 = amount_str.trim().parse().expect("Please enter a valid number");

    let expense = Expense {
        description: description.trim().to_string(),
        amount,
    };

    expenses.push(expense);
    println!("Expense added!");
}

fn list_expenses(expenses: &Vec<Expense>) {
    if expenses.is_empty() {
        println!("No expenses recorded.");
    } else {
        for (i, expense) in expenses.iter().enumerate() {
            println!("{}: {} - ${:.2}", i + 1, expense.description, expense.amount);
        }
    }
}

fn save_expenses(expenses: &Vec<Expense>, filename: &str) {
    let file = File::create(filename).expect("Could not create file");
    serde_json::to_writer(file, expenses).expect("Could not write to file");
    println!("Expenses saved!");
}
