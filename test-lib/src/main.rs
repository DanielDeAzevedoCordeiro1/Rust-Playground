use mathfix::basic_operations::{ Basic, BasicErrors };

fn main(){

    let sum1 = Basic::add_usize(10, usize::MIN);
    match sum1 {
        Ok(value) => println!("Soma: {}", value),
        Err(e) => {
            println!("Erro: {:?}", e);
            std::process::exit(1);
        }
    }

    let sum2 = Basic::add_usize(10, usize::MAX);
    match sum2 {
        Ok(value) => println!("Soma: {}", value),
        Err(e) => {
            println!("Erro: {:?}", e);
            std::process::exit(1);
        }
    }
}