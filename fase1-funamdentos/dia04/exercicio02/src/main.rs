use std::io::{self, Write};

// ========================================
// OPERAÇÕES MATEMÁTICAS
// ========================================

fn somar(a: f64, b: f64) -> f64 {
    a + b
}

fn subtrair(a: f64, b: f64) -> f64 {
    a - b
}

fn multiplicar(a: f64, b: f64) -> f64 {
    a * b
}

/// Retorna (sucesso: bool, resultado: f64)
fn dividir(a: f64, b: f64) -> (bool, f64) {
    if b == 0.0 {
        (false, 0.0)
    } else {
        (true, a / b)
    }
}

fn resto(a: f64, b: f64) -> (bool, f64) {
    if b == 0.0 {
        (false, 0.0)
    } else {
        (true, a % b)
    }
}

fn potencia(base: f64, expoente: f64) -> f64 {
    base.powf(expoente)
}

fn raiz_quadrada(n: f64) -> (bool, f64) {
    if n < 0.0 {
        (false, 0.0)
    } else {
        (true, n.sqrt())
    }
}

// ========================================

/// Processa uma operação e retorna (sucesso, resultado, mensagem)
fn processar_operacao(op: char, a: f64, b: f64) -> (bool, f64, String) {
    match op {
        '+' => {
            let resultado = somar(a, b);
            (true, resultado, format!("{} + {} = {}", a, b, resultado))
        },
        '-' => {
            let resultado = subtrair(a, b);
            (true, resultado, format!("{} - {} = {}", a, b, resultado))
        },
        '*' => {
            let resultado = multiplicar(a, b);
            (true, resultado, format!("{} × {} = {}", a, b, resultado))
        },
        '/' => {
            let (ok, resultado) = dividir(a, b);
            if ok {
                (true, resultado, format!("{} ÷ {} = {}", a, b, resultado))
            } else {
                (false, 0.0, String::from("❌ Erro: Divisão por zero!"))
            }
        },
        '%' => {
            let (ok, resultado) = resto(a, b);
            if ok {
                (true, resultado, format!("{} % {} = {}", a, b, resultado))
            } else {
                (false, 0.0, String::from("❌ Erro: Resto por zero!"))
            }
        },
        '^' => {
            let resultado = potencia(a, b);
            (true, resultado, format!("{}^{} = {}", a, b, resultado))
        },
        _ => (false, 0.0, String::from("❌ Operação inválida!"))
    }
}

// ========================================
// UTILITÁRIOS DE I/O
// ========================================

fn ler_linha() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler entrada");
    input.trim().to_string()
}

fn ler_numero(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        
        let input = ler_linha();
        
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("❌ Por favor, digite um número válido!")
        }
    }
}

fn ler_operacao() -> char {
    loop {
        print!("Operação (+, -, *, /, %, ^): ");
        io::stdout().flush().unwrap();
        
        let input = ler_linha();
        
        if input.len() == 1 {
            let op = input.chars().next().unwrap();
            if "+-*/%^".contains(op) {
                return op;
            }
        }
        
        println!("❌ Operação inválida! Use: +, -, *, /, %, ^");
    }
}

// ========================================
// HISTÓRICO
// ========================================

struct Historico {
    operacoes: Vec<String>
}

impl Historico {
    fn novo() -> Self {
        Historico {
            operacoes: Vec::new()
        }
    }
    
    fn adicionar(&mut self, operacao: String) {
        self.operacoes.push(operacao);
    }
    
    fn exibir(&self) {
        if self.operacoes.is_empty() {
            println!("📝 Histórico vazio");
        } else {
            println!("\n📝 HISTÓRICO DE OPERAÇÕES:");
            for (i, op) in self.operacoes.iter().enumerate() {
                println!("  {}. {}", i + 1, op);
            }
        }
    }
    
    fn limpar(&mut self) {
        self.operacoes.clear();
        println!("🗑️  Histórico limpo!");
    }
}

// ========================================
// MENU E INTERFACE
// ========================================

fn exibir_menu() {
    println!("\n╔══════════════════════════════╗");
    println!("║   🔢 CALCULADORA RUST 🦀    ║");
    println!("╠══════════════════════════════╣");
    println!("║ 1. Nova operação             ║");
    println!("║ 2. Raiz quadrada             ║");
    println!("║ 3. Ver histórico             ║");
    println!("║ 4. Limpar histórico          ║");
    println!("║ 0. Sair                      ║");
    println!("╚══════════════════════════════╝");
}

fn executar_operacao(historico: &mut Historico) {
    println!("\n➕ NOVA OPERAÇÃO");
    
    let a = ler_numero("Primeiro número: ");
    let operacao = ler_operacao();
    let b = ler_numero("Segundo número: ");
    
    let (sucesso, _resultado, mensagem) = processar_operacao(operacao, a, b);
    
    println!("\n{}", mensagem);
    
    if sucesso {
        historico.adicionar(mensagem);
    }
}

fn executar_raiz(historico: &mut Historico) {
    println!("\n√ RAIZ QUADRADA");
    
    let n = ler_numero("Número: ");
    let (ok, resultado) = raiz_quadrada(n);
    
    if ok {
        let mensagem = format!("√{} = {}", n, resultado);
        println!("\n{}", mensagem);
        historico.adicionar(mensagem);
    } else {
        println!("\n❌ Erro: Raiz quadrada de número negativo!");
    }
}

fn main() {
    let mut historico = Historico::novo();
    
    println!("🦀 Bem-vindo à Calculadora Rust!");
    
    loop {
        exibir_menu();
        print!("\nEscolha uma opção: ");
        io::stdout().flush().unwrap();
        
        let opcao = ler_linha();
        
        match opcao.as_str() {
            "1" => executar_operacao(&mut historico),
            "2" => executar_raiz(&mut historico),
            "3" => historico.exibir(),
            "4" => historico.limpar(),
            "0" => {
                println!("\n👋 Até logo!");
                break;
            },
            _ => println!("❌ Opção inválida!")
        }
    }
}

// ========================================
// TESTES
// ========================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_somar() {
        assert_eq!(somar(10.0, 5.0), 15.0);
        assert_eq!(somar(-5.0, 3.0), -2.0);
    }
    
    #[test]
    fn test_subtrair() {
        assert_eq!(subtrair(10.0, 5.0), 5.0);
        assert_eq!(subtrair(3.0, 5.0), -2.0);
    }
    
    #[test]
    fn test_multiplicar() {
        assert_eq!(multiplicar(4.0, 5.0), 20.0);
        assert_eq!(multiplicar(-2.0, 3.0), -6.0);
    }
    
    #[test]
    fn test_dividir() {
        assert_eq!(dividir(10.0, 2.0), (true, 5.0));
        assert_eq!(dividir(10.0, 0.0), (false, 0.0));
    }
    
    #[test]
    fn test_potencia() {
        assert_eq!(potencia(2.0, 3.0), 8.0);
        assert_eq!(potencia(5.0, 2.0), 25.0);
    }
    
    #[test]
    fn test_raiz_quadrada() {
        assert_eq!(raiz_quadrada(16.0), (true, 4.0));
        assert_eq!(raiz_quadrada(-4.0).0, false);
    }
}