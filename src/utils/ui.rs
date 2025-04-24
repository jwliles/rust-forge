use std::io::{self, Write};

/// Prompt the user for confirmation, returning true if they answer yes
pub fn confirm(message: &str) -> bool {
    print!("{} [y/N]: ", message);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return false;
    }
    
    let input = input.trim().to_lowercase();
    input == "y" || input == "yes"
}

/// Prompt the user for confirmation with a text match, returning true if they type the exact match
pub fn confirm_with_text(message: &str, required_text: &str) -> bool {
    println!("{}", message);
    print!("Type '{}' to confirm: ", required_text);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return false;
    }
    
    input.trim() == required_text
}

/// Display a warning message in a standard format
pub fn show_warning(message: &str) {
    println!("WARNING: {}", message);
}

/// Format a status message for a dotfile
pub fn format_dotfile_status(status: crate::dotfile::DotFileStatus) -> String {
    match status {
        crate::dotfile::DotFileStatus::Staged => "Staged".to_string(),
        crate::dotfile::DotFileStatus::Linked => "Linked".to_string(),
        crate::dotfile::DotFileStatus::Unlinked => "Unlinked".to_string(),
    }
}