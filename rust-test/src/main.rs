use mathfix::Basic;

fn main() {
    let mut res = 0;
    Basic::add_u64(100, 20, &mut res);
    println!("Resultado adicao: {}", res);

    let mut res = 0;
    Basic::subtract_u64(100, 20, &mut res);
    println!("Resultado subtracao: {}", res);

    let mut res = 0;
    Basic::multiply_u64(100, 20, &mut res);
    println!("Resultado multiplicacao: {}", res);

    let mut res = 0;
    Basic::divide_u64(100, 20, &mut res);
    println!("Resultado divisao: {}", res);
}
