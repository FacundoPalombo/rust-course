fn main() {

    let numero_1 = 420;
    let numero_2 = 71;
    let suma = numero_1 + numero_2;

    loop {
        println!("Escribe la suma de {} y {} ", numero_1, numero_2);

        let mut suma_usuario: String = String::new();
        
        std::io::stdin().read_line(&mut suma_usuario).unwrap();

        let suma_usuario_int: i32 = suma_usuario.trim().parse().unwrap();

        if suma_usuario_int == suma {
            println!("La suma es correcta");
            break;
        } else {
            println!("La suma es incorrecta");
        } 
    }

}
