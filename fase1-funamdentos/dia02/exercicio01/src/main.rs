// Importa módulo para entrada/saída
use std::io;

fn main() {
    // Banner do programa
    println!("╔════════════════════════════════════╗");
    println!("║   CALCULADORA DE IMC - RUST 🦀    ║");
    println!("╚════════════════════════════════════╝\n");
    
    // Recebe peso do usuário
    let peso = ler_numero("Digite seu peso (kg): ");
    
    // Recebe altura do usuário
    let altura = ler_numero("Digite sua altura (m): ");
    
    // Valida entradas
    if !validar_entradas(peso, altura) {
        println!("\n❌ Valores inválidos! Tente novamente.");
        return;
    }
    
    // Calcula IMC
    let imc = calcular_imc(peso, altura);
    
    // Classifica o resultado
    let classificacao = classificar_imc(imc);
    
    // Exibe resultado
    exibir_resultado(peso, altura, imc, classificacao);
}

/// Lê um número do usuário com mensagem personalizada
/// 
/// # Argumentos
/// * `mensagem` - Texto a ser exibido ao usuário
/// 
/// # Retorna
/// * `f64` - Número digitado pelo usuário (ou 0.0 se inválido)
fn ler_numero(mensagem: &str) -> f64 {
    // Cria buffer para armazenar entrada
    let mut entrada = String::new();
    
    // Exibe mensagem
    print!("{}", mensagem);
    
    // Garante que o print apareça antes do input
    io::Write::flush(&mut io::stdout()).expect("Falha ao flush");
    
    // Lê linha da entrada padrão
    io::stdin()
        .read_line(&mut entrada)
        .expect("Falha ao ler linha");
    
    // Remove espaços em branco e tenta fazer parse
    // trim(): remove \n e espaços
    // parse(): converte String -> f64
    // unwrap_or(0.0): se falhar, retorna 0.0
    entrada.trim().parse().unwrap_or(0.0)
}

/// Valida se peso e altura são valores razoáveis
/// 
/// # Argumentos
/// * `peso` - Peso em kg
/// * `altura` - Altura em metros
/// 
/// # Retorna
/// * `bool` - true se válido, false caso contrário
fn validar_entradas(peso: f64, altura: f64) -> bool {
    // Verifica limites razoáveis
    let peso_valido = peso > 0.0 && peso < 500.0;  // 0-500 kg
    let altura_valida = altura > 0.0 && altura < 3.0;  // 0-3 metros
    
    // Retorna true apenas se AMBOS forem válidos
    peso_valido && altura_valida
}

/// Calcula o IMC (peso / altura²)
/// 
/// # Argumentos
/// * `peso` - Peso em kg
/// * `altura` - Altura em metros
/// 
/// # Retorna
/// * `f64` - Valor do IMC
fn calcular_imc(peso: f64, altura: f64) -> f64 {
    // Fórmula: IMC = peso / altura²
    // powf(2.0) eleva ao quadrado
    peso / altura.powf(2.0)
}

/// Classifica o IMC segundo OMS
/// 
/// ### Argumentos
/// * `imc` - Valor do IMC
/// 
/// #### Retorna
/// * `&str` - Classificação textual
fn classificar_imc(imc: f64) -> &'static str {
    // Match com ranges (pattern matching)
    match imc {
        x if x < 16.0 => "Magreza grave",
        x if x >= 16.0 && x < 17.0 => "Magreza moderada",
        x if x >= 17.0 && x < 18.5 => "Magreza leve",
        x if x >= 18.5 && x < 25.0 => "Peso normal",
        x if x >= 25.0 && x < 30.0 => "Sobrepeso",
        x if x >= 30.0 && x < 35.0 => "Obesidade grau I",
        x if x >= 35.0 && x < 40.0 => "Obesidade grau II",
        _ => "Obesidade grau III (mórbida)",
    }
}

/// Exibe resultado formatado
/// 
/// # Argumentos
/// * `peso` - Peso em kg
/// * `altura` - Altura em metros
/// * `imc` - Valor calculado do IMC
/// * `classificacao` - Classificação do IMC
fn exibir_resultado(peso: f64, altura: f64, imc: f64, classificacao: &str) {
    println!("\n╔════════════════════════════════════╗");
    println!("║           RESULTADO                ║");
    println!("╠════════════════════════════════════╣");
    println!("║ Peso:           {:.2} kg          ║", peso);
    println!("║ Altura:         {:.2} m           ║", altura);
    println!("║ IMC:            {:.2}              ║", imc);
    println!("║ Classificação:  {:<17}║", classificacao);
    println!("╚════════════════════════════════════╝");
    
    // Emoji baseado na classificação
    let emoji = match classificacao {
        "Peso normal" => "✅",
        "Sobrepeso" => "⚠️",
        "Magreza grave" | "Obesidade grau III (mórbida)" => "🚨",
        _ => "⚡",
    };
    
    println!("\n{} {}", emoji, classificacao);
}