use std::env;
use conway_aes::{encrypt, decrypt, ConwayParams};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut mode = "encrypt";
    let mut public = String::new();
    let mut private = String::new();
    let mut method = String::new();
    let mut data = String::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--mode" => { i += 1; mode = &args[i]; }
            "--public" => { i += 1; public = args[i].clone(); }
            "--private" => { i += 1; private = args[i].clone(); }
            "--method" => { i += 1; method = args[i].clone(); }
            "--data" => { i += 1; data = args[i].clone(); }
            _ => {}
        }
        i += 1;
    }

    let params = ConwayParams {
        public: public.into_bytes(),
        private: private.into_bytes(),
        method: method.into_bytes(),
    };

    let result = if mode == "decrypt" {
        decrypt(data.as_bytes(), &params)
    } else {
        encrypt(data.as_bytes(), &params)
    };

    match result {
        Ok(out) => {
            let hex: String = out.iter().map(|b| format!("{:02x}", b)).collect();
            println!("{}", hex);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
