enum DatosPersona {
    Trabajador {
        id: u8,
        nombres: String,
        apellidos: String,
        edad: i8,
        puesto: String,
        salario: f32,
    },
}

fn mostrar_datos(persona: &DatosPersona, id: u8) {
    match persona {
        DatosPersona::Trabajador {
            id: id_p,
            nombres,
            apellidos,
            edad,
            puesto,
            salario,
        } if *id_p == id => {
            println!("--- DATOS DEL TRABAJADOR ---");
            println!("ID: {}", id);
            println!("Nombre: {} {}", nombres, apellidos);
            println!("Edad: {}", edad);
            println!("Puesto: {}", puesto);
            println!("Salario: ${}", salario);
        }

        _ => {
            println!("No se encontro un trabajador con ese ID.");
        }
    }
}

fn main() {
    let trabajador = DatosPersona::Trabajador {
        id: 1,
        nombres: String::from("Kevin"),
        apellidos: String::from("Salas"),
        edad: 25,
        puesto: String::from("Desarrollador de software"),
        salario: 18500.50,
    };

    mostrar_datos(&trabajador, 1u8);
}
