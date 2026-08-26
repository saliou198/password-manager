//! Interactive menu, like the Python version's `main()` loop.
//! Skeleton: no login, no add/remove, no disk persistence yet.

mod crypto;

use std::io::{self, Write};

/// Read a line from stdin and trim it.
fn input_str(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().ok(); // make sure the prompt is shown before reading
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read input");
    return line.trim().to_string();
}

/// Read an integer from stdin, looping until the input parses.
fn input_int(prompt: &str) -> i32 {
    loop {
        let line = input_str(prompt);
        match line.parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("Invalid number, try again."),
        }
    }
}

fn main() {
    loop {
        println!("\n--- Password Manager ---");
        println!("1. Generate a password");
        println!("2. Derive key (debug)");
        println!("3. Encrypt (debug)");
        println!("4. Decrypt (debug)");
        println!("5. Exit");

        let choice = input_int("Choice: ");

        match choice {
            // GENERATE command
            1 => println!("Generated password: {}", crypto::generate_password()),
            // Derive key (debug)
            2 => {
                let password = input_str("Master password: ");
                let salt = [0u8; 16];
                match crypto::derive_key(&password, &salt) {
                    Ok(key) => println!("Key: {}", hex(&key)),
                    Err(e) => eprintln!("Error: {e}"),
                }
            }
            // Encrypt (debug)
            3 => {
                let password = input_str("Master password: ");
                let data = input_str("Data to encrypt: ");
                let salt = [0u8; 16];
                let Ok(key) = crypto::derive_key(&password, &salt) else {
                    eprintln!("Error: derive failed");
                    continue;
                };
                let vault: crypto::VaultData = [(
                    "demo".to_string(),
                    crypto::Credential {
                        username: "user".to_string(),
                        password: data,
                    },
                )]
                .into_iter()
                .collect();
                match crypto::encrypt_vault(&vault, &key) {
                    Ok((nonce, ct)) => {
                        println!("nonce={}", b64(&nonce));
                        println!("ciphertext={}", b64(&ct));
                    }
                    Err(e) => eprintln!("Error: {e}"),
                }
            }
            // Decrypt (debug)
            4 => {
                let password = input_str("Master password: ");
                let nonce_b64 = input_str("Nonce (base64): ");
                let ct_b64 = input_str("Ciphertext (base64): ");
                let salt = [0u8; 16];
                let Ok(key) = crypto::derive_key(&password, &salt) else {
                    eprintln!("Error: derive failed");
                    continue;
                };
                let Ok(nonce) = unb64(&nonce_b64) else {
                    eprintln!("Error: bad nonce");
                    continue;
                };
                let Ok(ct) = unb64(&ct_b64) else {
                    eprintln!("Error: bad ciphertext");
                    continue;
                };
                match crypto::decrypt_vault(&nonce, &ct, &key) {
                    Ok(vault) => println!("{vault:#?}"),
                    Err(e) => eprintln!("Error: {e}"),
                }
            }
            5 => {
                println!("Bye.");
                break;
            }
            _ => println!("Invalid choice."),
        }
    }
}

fn hex(bytes: &[u8]) -> String {
    return bytes.iter().map(|b| format!("{b:02x}")).collect();
}

fn b64(bytes: &[u8]) -> String {
    use base64::Engine;
    return base64::engine::general_purpose::STANDARD.encode(bytes);
}

fn unb64(s: &str) -> Result<Vec<u8>, ()> {
    use base64::Engine;
    return base64::engine::general_purpose::STANDARD.decode(s).map_err(|_| ());
}
