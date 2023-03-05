mod utils;

use utils::crypt;
use utils::file::get_file_contents;
use utils::file::write_file;
use utils::generate_key;
use utils::parse_key;

use std::env;

fn write_key() {
    let key = generate_key(32);
    let nonce = generate_key(16);

    match write_file("key.bk", &format!("{key}\n{nonce}")) {
        Ok(_) => println!("[+] Key written to ./private_key.bk\n\n[i] You will only be able to decrypt and encrypt files with this key. Please be careful with who you share it with and make sure to not lose it. If you lose it there is no way to recover your files, so be careful when storing these keys."),
        Err(e) => println!("{e}")
    };
}

fn read_key(path: &str) -> Result<(String, String), String> {
    match parse_key(path) {
        Ok(values) => Ok((
            values.get(0).unwrap().to_owned(),
            values.get(1).unwrap().to_owned(),
        )),
        Err(e) => Err(e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Make sure that they supplied the correct amount of arguments or help is requested
    if args.len() < 2 || args.contains(&"-h".to_owned()) {
        utils::help(&args);
        return;
    }

    // Get action
    match args.get(1) {
        Some(action) => match action.as_str() {
            "generate_key" => write_key(),
            "encrypt" => {
                if args.len() < 4 {
                    utils::help(&args);
                    return;
                }

                let (key, nonce) = match read_key(args.get(2).unwrap()) {
                    Ok(v) => v,
                    Err(e) => return println!("{e}"),
                };

                let file_contents = match get_file_contents(args.get(3).unwrap()) {
                    Ok(content) => content,
                    Err(e) => return println!("{e}"),
                };

                match crypt::encrypt(&file_contents, &key, &nonce) {
                    Ok(encrypted_file) => {
                        match write_file(
                            &(args.get(3).unwrap().to_owned() + ".enc"),
                            &encrypted_file,
                        ) {
                            Ok(_) => return println!("[+] File written successfully"),
                            Err(e) => return println!("[-] Error writing file: {e}"),
                        }
                    }
                    Err(e) => return println!("{e}"),
                };
            }
            "decrypt" => todo!(),
            _ => (),
        },
        None => return, // We should not get here because of the previous check
    };
}
