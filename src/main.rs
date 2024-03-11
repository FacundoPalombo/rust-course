fn main() {
    println!("Por  favor introduce tu edad");
    let mut edad: String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();

    // Convertir esa edad a un numero
    let edad_int: u8 = edad.trim().parse().unwrap();

    if edad_int >= 18 {
        println!("Bien, ¿llevas puestas zapatillas de lona? Y/n");
        let mut zapatillas_de_lona: String = String::new();
        std::io::stdin().read_line(&mut zapatillas_de_lona).unwrap();
        
        let opt: String = zapatillas_de_lona.trim().to_uppercase().to_string();


        if opt == "Y" {
            println!("Al boliche con zapatillas de lona no pasas pibe...");
        } else {
            println!("Pasa papá");
        }
    } else {
        println!("Al boliche no pasas nene");
    }
}
