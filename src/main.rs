// Import HashMap from standard library - similar to Python's dict
use std::collections::HashMap;

// #[derive(Debug, Clone)] - Auto-generates traits:
// Debug: allows printing with {:?} (like Python's __repr__)
// Clone: allows copying the struct with .clone() method
#[derive(Debug, Clone)]
pub struct Account {
    // pub makes fields publicly accessible (like Python without underscore)
    // u32 = unsigned 32-bit integer (always positive, 0 to 4,294,967,295)
    pub id: u32,
    // String = owned, growable string (like Python str but with ownership)
    pub name: String,
    // f64 = 64-bit floating point number (like Python float)
    pub balance: f64,
}

// impl block contains methods for Account struct
// Similar to Python class methods but separate from struct definition
impl Account {
    // Associated function (constructor) - called with Account::new()
    // pub makes it publicly accessible
    // Self refers to Account type (cleaner than writing Account)
    pub fn new(id: u32, name: String, balance: f64) -> Self {
        // Struct initialization - field names match parameter names
        Account { id, name, balance }
    }
}

// Bank struct - only derives Debug (can't clone because HashMap is complex)
#[derive(Debug)]
pub struct Bank {
    // Private field (no pub) - encapsulation like Python's _accounts
    // HashMap<K, V> = key-value store like Python dict
    // u32 = account ID (key), Account = account data (value)
    accounts: HashMap<u32, Account>,
    // Tracks next available ID for auto-assignment
    next_id: u32,
}

// Implementation block for Bank methods
impl Bank {
    // Constructor function - creates new empty bank
    // Self refers to Bank type
    pub fn new() -> Self {
        Bank {
            // HashMap::new() creates empty hash map
            accounts: HashMap::new(),
            // Start IDs from 1 (0 used as "not set" indicator)
            next_id: 1,
        }
    }

    // Add account to bank
    // &mut self = mutable borrow of Bank (can modify Bank's data)
    // mut account: Account = takes ownership of account AND makes it mutable
    // Result<u32, String> = return type - either Ok(account_id) or Err(error_message)
    pub fn add_account(&mut self, mut account: Account) -> Result<u32, String> {
        // Auto-assign ID if not set (0 means not set)
        if account.id == 0 {
            // Assign next available ID
            account.id = self.next_id;
            // Increment for next account
            self.next_id += 1;
        }

        // Check if account already exists using contains_key method
        // &account.id = borrow the ID (don't move it)
        if self.accounts.contains_key(&account.id) {
            // Early return with error - format! creates String like Python f-string
            return Err(format!("Account with ID {} already exists", account.id));
        }

        // Store account ID before moving account into HashMap
        let account_id = account.id;
        // insert() moves account into HashMap - account can't be used after this
        self.accounts.insert(account_id, account);
        // Return success with account ID
        Ok(account_id)
    }

    // Remove account from bank
    // &mut self = need mutable access to modify HashMap
    // account_id: u32 = copy the ID (u32 implements Copy trait)
    // Returns the removed account or error message
    pub fn remove_account(&mut self, account_id: u32) -> Result<Account, String> {
        // Method chaining: remove() returns Option<Account>
        // remove() takes ownership of the account (moves it out)
        // ok_or_else() converts Option to Result
        // || = closure (like Python lambda) that creates error message
        self.accounts
            .remove(&account_id)
            .ok_or_else(|| format!("Account with ID {} not found", account_id))
    }

    // Get account reference (read-only)
    // &self = immutable borrow (can't modify Bank)
    // Returns Option<&Account> = either Some(reference) or None
    // & = reference, not ownership - caller can read but not move account
    pub fn get_account(&self, account_id: u32) -> Option<&Account> {
        // get() returns Option<&V> - reference to value if exists
        self.accounts.get(&account_id)
    }

    // Get mutable account reference
    // &mut self = mutable borrow needed to get mutable reference to account
    // Returns Option<&mut Account> = mutable reference if account exists
    pub fn get_account_mut(&mut self, account_id: u32) -> Option<&mut Account> {
        // get_mut() returns Option<&mut V> - mutable reference to value
        self.accounts.get_mut(&account_id)
    }

    // List all accounts
    // &self = immutable borrow (read-only access)
    // Returns Vec<&Account> = vector of references to accounts
    // Vec = dynamic array like Python list
    pub fn list_accounts(&self) -> Vec<&Account> {
        // values() returns iterator over HashMap values
        // collect() consumes iterator and creates Vec
        // Returns references, not owned accounts (accounts stay in HashMap)
        self.accounts.values().collect()
    }

    // Update account balance
    // &mut self = mutable borrow to modify account data
    // Returns Result<(), String> = either Ok(()) for success or Err(message)
    // () = unit type (like Python's None for "no meaningful return value")
    pub fn update_balance(&mut self, account_id: u32, new_balance: f64) -> Result<(), String> {
        // Pattern matching on Option<&mut Account>
        match self.get_account_mut(account_id) {
            // Some(account) = found account, extract mutable reference
            Some(account) => {
                // Modify account through mutable reference
                account.balance = new_balance;
                // Return success with no data
                Ok(())
            }
            // None = account not found
            None => Err(format!("Account with ID {} not found", account_id)),
        }
    }
}

fn main() {
    // Create mutable bank instance - mut required because we'll modify it
    let mut bank = Bank::new();

    // Create accounts using constructor
    // Account::new() is associated function (like static method in Python)
    // to_string() converts &str to owned String
    let account1 = Account::new(0, "Alice".to_string(), 1000.0);
    let account2 = Account::new(0, "Bob".to_string(), 2000.0);

    // Add accounts - this MOVES accounts into bank (ownership transfer)
    // After this, account1 and account2 can't be used anymore
    // Pattern matching on Result<u32, String>
    match bank.add_account(account1) {
        // Ok(id) = success case, extract the returned ID
        Ok(id) => println!("Added account with ID: {}", id),
        // Err(e) = error case, extract error message
        Err(e) => println!("Error: {}", e),
    }

    // Same for second account
    match bank.add_account(account2) {
        Ok(id) => println!("Added account with ID: {}", id),
        Err(e) => println!("Error: {}", e),
    }

    // List all accounts
    println!("\nAll accounts:");
    // bank.list_accounts() returns Vec<&Account>
    // for loop borrows each reference - no ownership transfer
    for account in bank.list_accounts() {
        // {:?} uses Debug trait to print struct contents
        println!("{:?}", account);
    }

    // Get specific account using if let pattern
    // if let = pattern matching shorthand for single case
    // Some(account) = extract reference if account exists
    if let Some(account) = bank.get_account(1) {
        println!("\nFound account: {:?}", account);
    }

    // Update balance - different error handling pattern
    // if let Err(e) = only handles error case
    if let Err(e) = bank.update_balance(1, 1500.0) {
        println!("Error updating balance: {}", e);
    } else {
        // else handles Ok(()) case
        println!("Updated account 1 balance");
    }

    // Remove account - returns the removed account on success
    match bank.remove_account(2) {
        // Ok(removed_account) = success, get ownership of removed account
        Ok(removed_account) => println!("Removed account: {:?}", removed_account),
        Err(e) => println!("Error: {}", e),
    }

    // Show remaining accounts
    println!("\nRemaining accounts:");
    for account in bank.list_accounts() {
        println!("{:?}", account);
    }
}