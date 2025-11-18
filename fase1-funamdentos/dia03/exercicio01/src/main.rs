use std::io::{self, Write};

fn classificar_nota(nota: i32) -> &'static str {
    match nota {
        90..=100 => "A",
        80..=89  => "B",
        70..=79  => "C",
        60..=69  => "D",
        0..=59   => "F",
        _        => "Inválida"
    }
}

fn main() {
    println!("=== SISTEMA DE CLASSIFICAÇÃO DE NOTAS ===\n");
    
    let mut notas = Vec::new();
    
    // Loop para ler notas
    loop {
        print!("Digite uma nota (0-100) ou -1 para finalizar: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler entrada");
        
        // Parse da entrada
        let nota: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Entrada inválida! Digite um número.\n");
                continue;
            }
        };
        
        // Condição de saída
        if nota == -1 {
            break;
        }
        
        // Validação
        if nota < 0 || nota > 100 {
            println!("❌ Nota deve estar entre 0 e 100!\n");
            continue;
        }
        
        // Adiciona nota válida
        notas.push(nota);
        let conceito = classificar_nota(nota);
        println!("✅ Nota {} registrada: Conceito {}\n", nota, conceito);
    }
    
    // Verifica se há notas para processar
    if notas.is_empty() {
        println!("Nenhuma nota registrada.");
        return;
    }
    
    // Calcula estatísticas
    println!("\n=== ESTATÍSTICAS ===");
    println!("Total de notas: {}", notas.len());
    
    // Soma e média
    let soma: i32 = notas.iter().sum();
    let media = soma as f64 / notas.len() as f64;
    println!("Média: {:.2}", media);
    
    // Maior e menor
    let maior = notas.iter().max().unwrap();
    let menor = notas.iter().min().unwrap();
    println!("Maior nota: {}", maior);
    println!("Menor nota: {}", menor);
    
    // Contagem por conceito
    let mut contagem_a = 0;
    let mut contagem_b = 0;
    let mut contagem_c = 0;
    let mut contagem_d = 0;
    let mut contagem_f = 0;
    
    for &nota in &notas {
        match classificar_nota(nota) {
            "A" => contagem_a += 1,
            "B" => contagem_b += 1,
            "C" => contagem_c += 1,
            "D" => contagem_d += 1,
            "F" => contagem_f += 1,
            _   => {}
        }
    }
    
    println!("\n=== DISTRIBUIÇÃO DE CONCEITOS ===");
    println!("A (90-100): {} alunos", contagem_a);
    println!("B (80-89):  {} alunos", contagem_b);
    println!("C (70-79):  {} alunos", contagem_c);
    println!("D (60-69):  {} alunos", contagem_d);
    println!("F (0-59):   {} alunos", contagem_f);
    
    // Aprovação (>= 60)
    let aprovados = notas.iter().filter(|&&n| n >= 60).count();
    let taxa_aprovacao = (aprovados as f64 / notas.len() as f64) * 100.0;
    println!("\n📊 Taxa de aprovação: {:.1}%", taxa_aprovacao);
}