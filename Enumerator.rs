// Luis Alejandro Aguilar Soberanes

enum Alumno {
    Id(u32),
    Nombre(String),
    Apellido(String),
    Edad(i16),
    Estatura(f32),
    Peso(f32),
    Contacto {
        tipo_contacto: String,
        via_contacto: String,
    },
}

fn coincidencia(alumno: Alumno) {
    match alumno {
        Alumno::Id(50504) => {
            println!("Coincide con Luis Aguilar.");
            println!("Apellido: Aguilar");
            println!("Edad 21");
            println!("Estatura 1.95");
            println!("peso 95kg");
           println!("Contacto por WhatsApp y correo");
        }
        Alumno::Id(210701) => {
            println!("Coincide con Luis Aguilar.");
            println!("Apellido: Aguilar");
            println!("Edad 19");
            println!("Estatura 1.88");
            println!("peso 90kg");
            println!("Contacto por WhatsApp y correo");
    },
        Alumno::Id(21104) =>{ println!("Coincide con Luis Chan.");
             println!("Coincide con Luis Chan.");
            println!("Apellido: Portillo");
            println!("Edad 19");
            println!("Estatura 1.75");
            println!("peso 70kg");
            println!("Contacto por WhatsApp y correo");
    },
        Alumno::Id(otro) => println!("No hay coincidencia para el id: {}", otro),
        _ => println!("No es un ID, es otra información."),
    }
}

fn main() {
    let id1 = Alumno::Id(50504);
    let nombre = Alumno::Nombre(String::from("Luis"));
    let apellido = Alumno::Apellido(String::from("Aguilar"));
    let edad = Alumno::Edad(21);
    let peso = Alumno::Peso(90.0);
    let contacto = Alumno::Contacto {
    tipo_contacto: String::from("Teléfono"),
    via_contacto: String::from("WhatsApp"),
    };


    coincidencia(id1);
}
