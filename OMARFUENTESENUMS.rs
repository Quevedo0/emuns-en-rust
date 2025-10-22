use std::io;

enum NombreComp {
    Data { nombre: String, apellidos: String },
}

enum Persona {
    Persona {
        id: u8,
        nombrecomp: NombreComp,
        edad: u8,
        peso: f32,
        altura: f32,
        curp: String,
        rfc: String,
    },
    
}

impl Persona {
    fn info(&self) {
        println!("\n--- Información del cliente:");

        match self {
            Persona::Persona {
                id: _,
                nombrecomp,
                edad,
                peso,
                altura,
                curp,
                rfc,
            } => {
                match nombrecomp {
                    NombreComp::Data { nombre, apellidos } => {
                        println!("Nombre completo: {} {}", nombre, apellidos);
                    }
                }
                println!("Edad: {}", edad);
                println!("Peso: {:.2} kg", peso);
                println!("Altura: {:.2} m", altura);
                println!("Curp: {}", curp);
                println!("RFC: {}", rfc);
                println!("IMC: {:.2}", self.calcular_imc());
            }
        }
    }

    fn calcular_imc(&self) -> f32 {
        match self {
            Persona::Persona { peso, altura, .. } => {
                
                *peso / (*altura * *altura)
            }
        }
    }
}

fn main() {
    let persona1 = Persona::Persona {
        id: 1,
        nombrecomp: NombreComp::Data {
            nombre: String::from("omar gerardo"),
            apellidos: String::from("Fuentes Huaira"),
        },
        edad: 30,
        peso: 93.0,
        altura: 1.75,
        curp: String::from("CURPPERS2233232"),
        rfc: String::from("FOEDKEKD3333444"),
    };

    let persona2 = Persona::Persona {
        id: 2,
        nombrecomp: NombreComp::Data {
            nombre: String::from("alex fernando"),
            apellidos: String::from("Huaira Ortega"),
        },
        edad: 24,
        peso: 70.0,
        altura: 1.70,
        curp: String::from("CURPPERS2222222"),
        rfc: String::from("FOEDKEKD3333423334"),
    };

    let persona3 = Persona::Persona {
        id: 3,
        nombrecomp: NombreComp::Data {
            nombre: String::from("anni"),
            apellidos: String::from("Lopez Greenwaibols"),
        },
        edad: 23,
        peso: 50.0,
        altura: 1.55,
        curp: String::from("ANNIES2222222"),
        rfc: String::from("FOEANNIED3333423334"),
    };

    loop {
        println!("\nSelecciona qué persona quieres consultar:");
        println!("1, 2, 3 (otro para salir)");
        println!("Ingresa una opción:");

        let mut op = String::new();
        io::stdin()
            .read_line(&mut op)
            .expect("Error al leer la entrada");

        
        let opcion: u8 = op.trim().parse().unwrap_or(0);

        match opcion {
            1 => {
                persona1.info();
                println!("IMC calculado: {:.2}", persona1.calcular_imc());
            }
            2 => {
                persona2.info();
                println!("IMC calculado: {:.2}", persona2.calcular_imc());
            }
            3 => {
                persona3.info();
                println!("IMC calculado: {:.2}", persona3.calcular_imc());
            }
            _ => {
                println!("Saliendo...");
                break;
            }
        }
    }
}


    
    


