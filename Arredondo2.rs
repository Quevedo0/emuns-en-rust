struct Trabajador {
    id: u8,
    nombre: String,
    edad: u8,
    salario: f32,
    contactos: String,
}

enum TipoTrabajador {
    Juan(Trabajador),
    Maria(Trabajador),
    Pedro(Trabajador),
}

fn imprimir_usuario(t: &Trabajador) {
    println!("ID: {}", t.id);
    println!("Nombre: {}", t.nombre);
    println!("Edad: {}", t.edad);
    println!("Salario: {}", t.salario);
    println!("Contactos: {}", t.contactos);
}

fn main() {
    let trabajador_juan = TipoTrabajador::Juan(Trabajador {
        id: 1,
        nombre: String::from("Juan Perez"),
        edad: 30,
        salario: 2500.0,
        contactos: String::from("998837387"),
    });

    let trabajador_maria = TipoTrabajador::Maria(Trabajador {
        id: 2,
        nombre: String::from("Maria Lopez"),
        edad: 28,
        salario: 2700.0,
        contactos: String::from("998837388"),
    });

    let trabajador_pedro = TipoTrabajador::Pedro(Trabajador {
        id: 3,
        nombre: String::from("Pedro Gomez"),
        edad: 35,
        salario: 3000.0,
        contactos: String::from("998837389"),
    });

    let opcion = 2;

    println!("Detalles del trabajador seleccionado:");

    let trabajador_seleccionado = match opcion {
        1 => Some(trabajador_juan),
        2 => Some(trabajador_maria),
        3 => Some(trabajador_pedro),
        _ => None,
    };

    match trabajador_seleccionado {
        Some(TipoTrabajador::Juan(t)) |
        Some(TipoTrabajador::Maria(t)) |
        Some(TipoTrabajador::Pedro(t)) => imprimir_usuario(&t),
        None => println!("Opción no válida"),
    }
}
