use std::io::{self, Write};

fn fibonacci_loop(n: usize) -> Vec<u64> {
    if n == 0 {
        return vec![];
    }
    
    if n == 1 {
        return vec![0];
    }
    
    let mut sequencia = vec![0, 1];
    
    // Loop tradicional - começa do terceiro termo
    let mut contador = 2;
    loop {
        if contador >= n {
            break;
        }
        
        let proximo = sequencia[contador - 1] + sequencia[contador - 2];
        sequencia.push(proximo);
        contador += 1;
    }
    
    sequencia
}

fn main() {
    println!("=== GERADOR FIBONACCI (LOOP) ===\n");
    
    print!("Quantos termos deseja gerar? ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let n: usize = match input.trim().parse() {
        Ok(num) if num > 0 && num <= 50 => num,
        _ => {
            println!("Digite um número entre 1 e 50!");
            return;
        }
    };
    
    let resultado = fibonacci_loop(n);
    
    println!("\nSequência de Fibonacci ({} termos):", n);
    
    for (i, valor) in resultado.iter().enumerate() {
        print!("{}", valor);
        if i < resultado.len() - 1 {
            print!(", ");
        }
    }
    
    println!("\n\nÚltimo termo: {}", resultado[resultado.len() - 1]);
    
    // Soma de todos os termos
    let soma: u64 = resultado.iter().sum();
    println!("Soma de todos os termos: {}", soma);
}