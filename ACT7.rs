struct Usuario {
    id: u32,
    nombre: String,
    sexo: String,
    edad: u32,
    altura: f32,
    peso: f32,
}


fn imprimir_usuario(u: &Usuario) {
    
    println!("ID: {}", u.id);
    println!("Nombre: {}", u.nombre);
    println!("Sexo: {}", u.sexo);
    println!("Edad: {} años", u.edad);
    println!("Altura: {:.2} m", u.altura);
    println!("Peso: {:.1} kg", u.peso);

}

fn main() {

    let usuario1 = Usuario {
        id: 1,
        nombre: String::from("Jenny"),
        sexo: String::from("Femenino"),
        edad: 24,
        altura: 1.68,
        peso: 58.0,
    };

    let usuario2 = Usuario {
        id: 2,
        nombre: String::from("Danixa"),
        sexo: String::from("Femenino"),
        edad: 21,
        altura: 1.72,
        peso: 67.0,
    };

    let usuario3 = Usuario {
        id: 3,
        nombre: String::from("Wendy"),
        sexo: String::from("Femenino"),
        edad: 19,
        altura: 1.64,
        peso: 50.0,
    };


    let num = 3;

    match num {
        1 => imprimir_usuario(&usuario1),
        2 => imprimir_usuario(&usuario2),
        3 => imprimir_usuario(&usuario3),
        _ => println!("Número no válido (solo 1, 2 o 3)"),
    }

    
    match num {
        1 => println!("Número en texto: One"),
        2 => println!("Número en texto: Two"),
        3 => println!("Número en texto: Three"),
        _ => println!("Número fuera de rango"),
    }
}
