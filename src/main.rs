fn main() {
    match division(10, 2) {
        None => println!("Erreur"),
        Some(resultat) => println!("Resultat : {}", resultat)
    }

    match enter_password("") {
        None => println!("Password vide..."),
        Some(pass) => println!("Mot de passe : {}", pass)
    }
}

fn division(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

fn enter_password(pwd: &str) -> Option<&str> {
    if pwd.is_empty() { 
        return None;
    }
    Some(pwd)
}