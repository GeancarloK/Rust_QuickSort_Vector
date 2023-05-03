use std::io;

pub fn scan_i32(numero: &mut i32) {
    //input String -> i32
    let mut str_numero: String = String::new();
    io::stdin().read_line(&mut str_numero).expect("erroscan_i32");
    *numero = str_numero.trim().parse::<i32>().unwrap();
}

pub fn scan_f32(numero: &mut f32) {
    //input String -> f32
    let mut str_numero: String = String::new();
    io::stdin().read_line(&mut str_numero).expect("erro scan_f32");
    *numero = str_numero.trim().parse::<f32>().unwrap();
}

pub fn scan_string(string: &mut String) {
    //input String
    io::stdin().read_line(string).expect("erro scan_String");
}

pub fn scan_bool(booleano: &mut bool) {
    //input String
    let mut string: String = String::new();
    io::stdin().read_line(&mut string).expect("erro scan_bool");
    match string.as_str(){
        "true" => *booleano=true,
        "1" => *booleano=true,
        "false" => *booleano=false,
        "0" => *booleano=false,
        _ => todo!()
    }
}
