// ========================================
// BIBLIOTECA MATEMÁTICA
// ========================================

/// Calcula o fatorial de um número
/// Exemplo: fatorial(5) = 5 * 4 * 3 * 2 * 1 = 120
fn fatorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 1; // Caso base
    }
    
    n * fatorial(n - 1) // Recursão + retorno implícito
}

/// Fatorial iterativo (mais eficiente)
fn fatorial_iterativo(n: u64) -> u64 {
    let mut resultado = 1;
    
    for i in 2..=n {
        resultado *= i;
    }
    
    resultado // Retorno implícito
}

// ========================================

/// Fibonacci recursivo (LENTO para n > 40)
fn fibonacci_recursivo(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2)
    }
}

/// Fibonacci iterativo (RÁPIDO)
fn fibonacci_iterativo(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    let mut anterior = 0;
    let mut atual = 1;
    
    for _ in 2..=n {
        let proximo = anterior + atual;
        anterior = atual;
        atual = proximo;
    }
    
    atual
}

/// Retorna (valor, tempo_ms) para comparar performance
fn fibonacci_com_tempo(n: u32, usar_recursivo: bool) -> (u64, u128) {
    use std::time::Instant;
    
    let inicio = Instant::now();
    
    let resultado = if usar_recursivo {
        fibonacci_recursivo(n)
    } else {
        fibonacci_iterativo(n)
    };
    
    let duracao = inicio.elapsed().as_millis();
    
    (resultado, duracao) // Tupla com resultado e tempo
}

// ========================================

/// Verifica se um número é primo
fn eh_primo(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false; // Pares (exceto 2) não são primos
    }
    
    // Só precisa testar até raiz quadrada de n
    let limite = (n as f64).sqrt() as u64;
    
    for divisor in (3..=limite).step_by(2) {
        if n % divisor == 0 {
            return false;
        }
    }
    
    true
}

/// Retorna (eh_primo: bool, divisores: Vec<u64>)
fn analisar_numero(n: u64) -> (bool, Vec<u64>) {
    let primo = eh_primo(n);
    
    let mut divisores = Vec::new();
    
    if !primo {
        for i in 2..=n {
            if n % i == 0 {
                divisores.push(i);
            }
        }
    }
    
    (primo, divisores)
}

// ========================================

/// Calcula o MDC (Máximo Divisor Comum) usando algoritmo de Euclides
fn mdc(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        mdc(b, a % b) // Recursão elegante!
    }
}

/// MDC iterativo (alternativa)
fn mdc_iterativo(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Calcula o MMC (Mínimo Múltiplo Comum)
/// Fórmula: MMC(a, b) = (a * b) / MDC(a, b)
fn mmc(a: u64, b: u64) -> u64 {
    (a * b) / mdc(a, b)
}

/// Retorna (mdc, mmc) de uma vez
fn mdc_mmc(a: u64, b: u64) -> (u64, u64) {
    let divisor_comum = mdc(a, b);
    let multiplo_comum = (a * b) / divisor_comum;
    (divisor_comum, multiplo_comum)
}

// ========================================

/// Encontra todos os primos até n (Crivo de Eratóstenes)
fn primos_ate(n: u64) -> Vec<u64> {
    if n < 2 {
        return Vec::new();
    }
    
    let mut eh_primo = vec![true; (n + 1) as usize];
    eh_primo[0] = false;
    eh_primo[1] = false;
    
    let limite = (n as f64).sqrt() as u64;
    
    for i in 2..=limite {
        if eh_primo[i as usize] {
            let mut multiplo = i * i;
            while multiplo <= n {
                eh_primo[multiplo as usize] = false;
                multiplo += i;
            }
        }
    }
    
    // Coleta os primos
    let mut primos = Vec::new();
    for (num, &primo) in eh_primo.iter().enumerate() {
        if primo {
            primos.push(num as u64);
        }
    }
    
    primos
}

// ========================================
// FUNÇÃO MAIN - DEMONSTRAÇÃO
// ========================================

fn main() {
    println!("🧮 BIBLIOTECA MATEMÁTICA EM RUST\n");
    
    // FATORIAIS
    println!("=== FATORIAIS ===");
    for i in 0..=10 {
        println!("{}! = {}", i, fatorial(i));
    }
    
    // FIBONACCI
    println!("\n=== FIBONACCI ===");
    for i in 0..=15 {
        println!("fib({}) = {}", i, fibonacci_iterativo(i));
    }
    
    // Comparação de performance
    println!("\n=== PERFORMANCE FIBONACCI(35) ===");
    let (resultado_rec, tempo_rec) = fibonacci_com_tempo(35, true);
    println!("Recursivo: {} ({}ms)", resultado_rec, tempo_rec);
    
    let (resultado_iter, tempo_iter) = fibonacci_com_tempo(35, false);
    println!("Iterativo: {} ({}ms)", resultado_iter, tempo_iter);
    println!("Iterativo é {}x mais rápido!", tempo_rec / tempo_iter);
    
    // PRIMOS
    println!("\n=== NÚMEROS PRIMOS ===");
    let numeros = [2, 3, 4, 17, 25, 29, 97, 100];
    for &n in &numeros {
        let (primo, divisores) = analisar_numero(n);
        if primo {
            println!("{} é PRIMO", n);
        } else {
            println!("{} NÃO é primo - divisores: {:?}", n, divisores);
        }
    }
    
    // Primos até 50
    println!("\nPrimos até 50: {:?}", primos_ate(50));
    
    // MDC e MMC
    println!("\n=== MDC e MMC ===");
    let pares = [(12, 18), (24, 36), (15, 25), (7, 13)];
    for (a, b) in pares {
        let (divisor, multiplo) = mdc_mmc(a, b);
        println!("MDC({}, {}) = {} | MMC({}, {}) = {}", 
                 a, b, divisor, a, b, multiplo);
    }
}

// ========================================
// TESTES AUTOMATIZADOS
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fatorial() {
        assert_eq!(fatorial(0), 1);
        assert_eq!(fatorial(1), 1);
        assert_eq!(fatorial(5), 120);
        assert_eq!(fatorial(10), 3628800);
    }
    
    #[test]
    fn test_fatorial_iterativo() {
        assert_eq!(fatorial_iterativo(0), 1);
        assert_eq!(fatorial_iterativo(5), 120);
        assert_eq!(fatorial_iterativo(10), 3628800);
    }
    
    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci_iterativo(0), 0);
        assert_eq!(fibonacci_iterativo(1), 1);
        assert_eq!(fibonacci_iterativo(10), 55);
        assert_eq!(fibonacci_iterativo(20), 6765);
    }
    
    #[test]
    fn test_eh_primo() {
        assert_eq!(eh_primo(2), true);
        assert_eq!(eh_primo(3), true);
        assert_eq!(eh_primo(4), false);
        assert_eq!(eh_primo(17), true);
        assert_eq!(eh_primo(97), true);
        assert_eq!(eh_primo(100), false);
    }
    
    #[test]
    fn test_mdc() {
        assert_eq!(mdc(12, 18), 6);
        assert_eq!(mdc(24, 36), 12);
        assert_eq!(mdc(7, 13), 1);
    }
    
    #[test]
    fn test_mmc() {
        assert_eq!(mmc(12, 18), 36);
        assert_eq!(mmc(4, 6), 12);
        assert_eq!(mmc(7, 13), 91);
    }
    
    #[test]
    fn test_primos_ate() {
        let primos = primos_ate(20);
        assert_eq!(primos, vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }
}