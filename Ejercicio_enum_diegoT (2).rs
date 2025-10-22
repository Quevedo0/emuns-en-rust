//Diego Santos Treviño Camacho

enum Persona{
    Datos {
    ID:i8,
    color_cabello:String,
    estatura:f32,
    peso:i32,
    color_de_ojos:String,
    color_de_piel:String,
    nombre:String,
    apellido:String,
    },
    }

fn comparar(id_busca : i8){

    let Caracteristicas = Persona::Datos{
    ID : 1,
    color_cabello : String::from ("castaño"),
    estatura : 1.87,
    peso : 91,
    color_de_ojos : String::from("cafes"),
    color_de_piel : String::from("moreno"),
    nombre : String::from("Diego"),
    apellido : String::from("Treviño"),
    };

    let Caracteristicas2 = Persona::Datos{
    ID : 2,
    color_cabello : String::from ("negro"),
    estatura : 1.95,
    peso : 95,
    color_de_ojos : String::from("cafes"),
    color_de_piel : String::from("moreno"),
    nombre : String::from("Lissandro"),
    apellido : String::from("Aguilar"),
    };

    let Caracteristicas3 = Persona::Datos{
    ID : 3,
    color_cabello : String::from ("castaño"),
    estatura : 1.80,
    peso : 105,
    color_de_ojos : String::from("cafes"),
    color_de_piel : String::from("moreno"),
    nombre : String::from("Elias"),
    apellido : String::from("Cuevas"),
    };
    
    match id_busca{
        1 => {
         let Persona::Datos{
            nombre,
            apellido,
            color_cabello,
            estatura,
            peso,
            color_de_ojos,
            color_de_piel,
            ..
        } = &Caracteristicas;
             println!("El nombre es: {}",nombre);
             println!("El apellido es: {}",apellido);
             println!("El color de cabello es: {}",color_cabello);
             println!("La estatura en metros es: {}",estatura);
             println!("El peso en kg es: {}",peso);
             println!("El color de ojos es: {}",color_de_ojos);
             println!("El color de piel es: {}",color_de_piel);
        },
        2 => {
         let Persona::Datos{
            nombre,
            apellido,
            color_cabello,
            estatura,
            peso,
            color_de_ojos,
            color_de_piel,
            ..
        } = &Caracteristicas2;
             println!("El nombre es: {}",nombre);
             println!("El apellido es: {}",apellido);
             println!("El color de cabello es: {}",color_cabello);
             println!("La estatura en metros es: {}",estatura);
             println!("El peso en kg es: {}",peso);
             println!("El color de ojos es: {}",color_de_ojos);
             println!("El color de piel es: {}",color_de_piel);
        },
        3 => {
         let Persona::Datos{
            nombre,
            apellido,
            color_cabello,
            estatura,
            peso,
            color_de_ojos,
            color_de_piel,
            ..
        } = &Caracteristicas3;
             println!("El nombre es: {}",nombre);
             println!("El apellido es: {}",apellido);
             println!("El color de cabello es: {}",color_cabello);
             println!("La estatura en metros es: {}",estatura);
             println!("El peso en kg es: {}",peso);
             println!("El color de ojos es: {}",color_de_ojos);
             println!("El color de piel es: {}",color_de_piel);
        },
        _=> println!("ID no encontrado\n"),
    }
}

fn main(){
    let id_busca = 1;
    
    comparar(id_busca);

}