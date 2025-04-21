use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};

/// Represents a user account in the Space Trader game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccount {
    pub id: String,           // Unique identifier for the account
    pub username: String,     // Unique username for login
    pub password_hash: String, // Bcrypt hashed password
    pub email: Option<String>, // Optional email for password recovery
    pub characters: Vec<String>, // List of character IDs associated with this account
    pub created_at: u64,      // Unix timestamp when account was created
    pub last_login: Option<u64>, // Unix timestamp of last login
}

/// Contains all user accounts and provides methods for account management
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountManager {
    accounts: HashMap<String, UserAccount>, // Map username to account
    account_ids: HashMap<String, String>,   // Map account ID to username
}

impl AccountManager {
    /// Create a new empty account manager
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            account_ids: HashMap::new(),
        }
    }
    
    /// Load account manager from disk, or create new if none exists
    pub fn load() -> Self {
        let accounts_path = Path::new("accounts.json");
        
        if accounts_path.exists() {
            match fs::read_to_string(accounts_path) {
                Ok(content) => {
                    match serde_json::from_str(&content) {
                        Ok(accounts) => return accounts,
                        Err(e) => {
                            eprintln!("Error parsing accounts file: {}", e);
                            // Create a backup of the corrupted file
                            if let Err(e) = fs::copy(accounts_path, "accounts.json.bak") {
                                eprintln!("Failed to backup accounts file: {}", e);
                            }
                        }
                    }
                },
                Err(e) => eprintln!("Error reading accounts file: {}", e),
            }
        }
        
        // If we got here, either no file exists or it was corrupted
        Self::new()
    }
    
    /// Save account manager to disk
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let accounts_path = Path::new("accounts.json");
        let serialized = serde_json::to_string_pretty(self)?;
        
        let mut file = File::create(accounts_path)?;
        file.write_all(serialized.as_bytes())?;
        
        Ok(())
    }
    
    /// Register a new user account
    pub fn register_account(
        &mut self, 
        username: &str, 
        password: &str,
        email: Option<&str>
    ) -> Result<UserAccount, AccountError> {
        // Check if username already exists
        if self.accounts.contains_key(username) {
            return Err(AccountError::UsernameExists);
        }
        
        // Hash the password
        let password_hash = match hash(password, DEFAULT_COST) {
            Ok(h) => h,
            Err(_) => return Err(AccountError::HashingFailed),
        };
        
        // Create a new account
        let account_id = Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        let account = UserAccount {
            id: account_id.clone(),
            username: username.to_string(),
            password_hash,
            email: email.map(|e| e.to_string()),
            characters: Vec::new(),
            created_at: now,
            last_login: None,
        };
        
        // Add to our maps
        self.accounts.insert(username.to_string(), account.clone());
        self.account_ids.insert(account_id, username.to_string());
        
        // Save changes
        if let Err(e) = self.save() {
            eprintln!("Error saving accounts: {}", e);
        }
        
        Ok(account)
    }
    
    /// Authenticate a user with username and password
    pub fn authenticate(&mut self, username: &str, password: &str) -> Result<UserAccount, AccountError> {
        // Check if username exists
        let account = match self.accounts.get(username) {
            Some(acc) => acc.clone(),
            None => return Err(AccountError::InvalidCredentials),
        };
        
        // Verify password
        match verify(password, &account.password_hash) {
            Ok(true) => {
                // Update last login time
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                // Clone the account before modifying it
                let mut updated_account = account.clone();
                updated_account.last_login = Some(now);
                
                // Update in our map
                self.accounts.insert(username.to_string(), updated_account.clone());
                
                // Save changes
                if let Err(e) = self.save() {
                    eprintln!("Error saving accounts after login: {}", e);
                }
                
                Ok(updated_account)
            },
            _ => Err(AccountError::InvalidCredentials),
        }
    }
    
    /// Add a character ID to an account
    pub fn add_character(&mut self, username: &str, character_id: &str) -> Result<(), AccountError> {
        // Get the account
        let account = match self.accounts.get(username) {
            Some(acc) => acc.clone(),
            None => return Err(AccountError::AccountNotFound),
        };
        
        // Check if character is already added
        if account.characters.contains(&character_id.to_string()) {
            return Ok(());
        }
        
        // Create an updated account
        let mut updated_account = account.clone();
        updated_account.characters.push(character_id.to_string());
        
        // Update the account in the map
        self.accounts.insert(username.to_string(), updated_account);
        
        // Save changes
        if let Err(e) = self.save() {
            eprintln!("Error saving accounts after adding character: {}", e);
        }
        
        Ok(())
    }
    
    /// Check if a username exists
    pub fn username_exists(&self, username: &str) -> bool {
        self.accounts.contains_key(username)
    }
    
    /// Get an account by username
    pub fn get_account_by_username(&self, username: &str) -> Option<UserAccount> {
        self.accounts.get(username).cloned()
    }
    
    /// Get an account by ID
    pub fn get_account_by_id(&self, id: &str) -> Option<UserAccount> {
        if let Some(username) = self.account_ids.get(id) {
            self.accounts.get(username).cloned()
        } else {
            None
        }
    }
    
    /// Get all account usernames
    pub fn get_all_usernames(&self) -> Vec<String> {
        self.accounts.keys().cloned().collect()
    }
    
    /// Change password for an account
    pub fn change_password(
        &mut self, 
        username: &str, 
        current_password: &str,
        new_password: &str
    ) -> Result<(), AccountError> {
        // First authenticate with current password
        match self.authenticate(username, current_password) {
            Ok(authenticated_account) => {
                // Hash the new password
                let password_hash = match hash(new_password, DEFAULT_COST) {
                    Ok(h) => h,
                    Err(_) => return Err(AccountError::HashingFailed),
                };
                
                // Create updated account with new password
                let mut updated_account = authenticated_account.clone();
                updated_account.password_hash = password_hash;
                
                // Update in the map
                self.accounts.insert(username.to_string(), updated_account);
                
                // Save changes
                if let Err(e) = self.save() {
                    eprintln!("Error saving accounts after password change: {}", e);
                }
                
                Ok(())
            },
            Err(e) => Err(e),
        }
    }
    
    /// Delete an account (requires password confirmation)
    pub fn delete_account(&mut self, username: &str, password: &str) -> Result<(), AccountError> {
        // First authenticate to confirm deletion
        match self.authenticate(username, password) {
            Ok(account) => {
                // Remove from ID map
                self.account_ids.remove(&account.id);
                
                // Remove from accounts map
                self.accounts.remove(username);
                
                // Save changes
                if let Err(e) = self.save() {
                    eprintln!("Error saving accounts after deletion: {}", e);
                }
                
                Ok(())
            },
            Err(e) => Err(e),
        }
    }
}

/// Errors that can occur during account operations
#[derive(Debug, thiserror::Error)]
pub enum AccountError {
    #[error("Username already exists")]
    UsernameExists,
    
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Account not found")]
    AccountNotFound,
    
    #[error("Failed to hash password")]
    HashingFailed,
}

// Unit tests for account manager
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_register_and_authenticate() {
        let mut manager = AccountManager::new();
        
        // Register a new account
        let account = manager.register_account("testuser", "password123", None).unwrap();
        assert_eq!(account.username, "testuser");
        
        // Try to authenticate
        let auth_result = manager.authenticate("testuser", "password123");
        assert!(auth_result.is_ok());
        
        // Try with wrong password
        let auth_fail = manager.authenticate("testuser", "wrongpassword");
        assert!(auth_fail.is_err());
    }
    
    #[test]
    fn test_username_exists() {
        let mut manager = AccountManager::new();
        
        // Register a new account
        manager.register_account("existinguser", "password123", None).unwrap();
        
        // Check if username exists
        assert!(manager.username_exists("existinguser"));
        assert!(!manager.username_exists("nonexistentuser"));
    }
}