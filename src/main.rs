use std::process::Command;
use std::fs;
use std::io::Write;

const PASS_DIR : &str = "";
const DEFAULT_EMAIL : &str = "";
const EXPORT_CSV : &str = "bitwarden.csv";
const EXCEPTIONS : [&str; 1] = [".git"];

fn get_all_sites(dir : &str, password_links : &mut Vec<String>) { // -> Vec<String> {
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() && !EXCEPTIONS.contains(&path.to_str().unwrap()) {
            get_all_sites(path.to_str().unwrap(), password_links);
        } else if path.is_file() && path.to_str().unwrap().ends_with(".gpg") {
            // let base_name = path.file_name().unwrap().to_str().unwrap();
            let path_str = path.to_str().unwrap();
            let pass_name = &path_str[PASS_DIR.len() + 1..path_str.len() - 4];
            password_links.push(String::from(pass_name));

        }
    }
}
#[derive(Debug)]
struct VaultwardenPass {
    email: String,
    password: String,
    url: String,
}

fn get_base_name_from_pass_path(pass_path : &String) -> String {
    let parts = pass_path.split("/");
    format!("https://{}", String::from(parts.last().unwrap())).to_string()
}

fn pass2vaultwarden_format(pass_path : &String) -> Option<VaultwardenPass> {
    let pass_store_content = String::from_utf8(Command::new("pass").args(["show", pass_path]).output().unwrap().stdout).unwrap();

    let it : Vec<&str> = pass_store_content.split("\n").collect();
    // Defining my specific formats

    if it.len() == 4 && it[1].starts_with("login: ") {
        println!("Format identified: pass firefox extension");
        Some(VaultwardenPass {
            email: String::from(&it[1]["login: ".len()..]), password: String::from(it[0]), url: get_base_name_from_pass_path(pass_path),
        })
    } else if it.len() == 3 && it[1].starts_with("url: ") {
        println!("Format identified: pass firefox extension without email");
        // stdin get email
        let mut email = String::new();
        print!("Enter email: ");
        std::io::stdin().read_line(&mut email).unwrap();
        let mut email = email.trim().to_string();
        if email.is_empty() { 
            email = String::from(DEFAULT_EMAIL);
        }
        Some(VaultwardenPass {
            email, password: String::from(it[0]), url: get_base_name_from_pass_path(pass_path),
        })
    } else if it.len() == 3 {
        println!("Format identified: password with email next");
        Some(VaultwardenPass {
            email: String::from(it[1]), password: String::from(it[0]), url: get_base_name_from_pass_path(pass_path),
        })
    } else if it.len() == 2 {
        println!("Format identified: manually inserted password");
        let mut email = String::new();
        print!("Enter email: ");
        std::io::stdin().read_line(&mut email).unwrap();
        let mut email = email.trim().to_string();
        if email.is_empty() {
            email = String::from(DEFAULT_EMAIL);
        }
        Some(VaultwardenPass {
            email, password: String::from(it[0]), url: get_base_name_from_pass_path(pass_path),
        })
    } else {
        for i in it {
            println!("{}", i);
        }
        None
    }

}

fn write_csv_headers() {
    let _ = fs::write(EXPORT_CSV, "name,login_username,login_password,login_uri\n");
}
fn write_to_csv(p : &VaultwardenPass) {
    let mut file = fs::OpenOptions::new().write(true).append(true).open(EXPORT_CSV).unwrap();
    writeln!(file, "{},{},{},{}", p.url, p.email, p.password, p.url);
}

fn main() {
    let mut password_links : Vec<String> = vec![];
    get_all_sites(PASS_DIR, &mut password_links);
    write_csv_headers();

    for link in password_links {
        println!("Pass links: {}", link);
        let vwp = pass2vaultwarden_format(&link).unwrap();
        println!("{:?}", vwp);
        write_to_csv(&vwp);
    }
}
