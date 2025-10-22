enum Persona {
    Persona {
        usuario: String,
        edad: u8,
        peso: u8,
        estatura: i16,
        sexo: String,
        nombre: String,
        vivo: bool,
    },
}

fn print_persona(p: &Persona) {
    match p {
        Persona::Persona { usuario, edad, peso, estatura, sexo, nombre, vivo } => {
            println!("usuario: {}", usuario);
            println!("edad: {}", edad);
            println!("peso: {}", peso);
            println!("estatura: {}", estatura);
            println!("sexo: {}", sexo);
            println!("nombre: {}", nombre);
            println!("vivo: {}", vivo);
        }
    }
}

fn main() {
    let usuario1 = Persona::Persona {
        usuario: String::from("dani123"),
        edad: 25,
        peso: 70,
        estatura: 175,
        sexo: String::from("masculino"),
        nombre: String::from("Daniel Enrique"),
        vivo: true,
    };

    print_persona(&usuario1);
}
