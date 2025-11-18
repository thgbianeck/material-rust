use std::io::{self, Write};
use std::cmp::Ordering;

fn main() {
    println!("╔═══════════════════════════════════════╗");
    println!("║   JOGO DE ADIVINHAÇÃO EM RUST!      ║");
    println!("╚═══════════════════════════════════════╝\n");
    
    // Simula número aleatório (1-100)
    // Em produção, use: use rand::Rng; let numero = rand::thread_rng().gen_range(1..=100);
    let numero_secreto = 42; // Substitua por gerador aleatório
    
    println!("Pensei em um número entre 1 e 100.");
    println!("Você tem 7 tentativas para acertar!\n");
    
    let max_tentativas = 7;
    let mut tentativa_atual = 0;
    let mut historico = Vec::new();
    let mut acertou = false;
    
    // Loop principal do jogo
    'jogo: loop {
        tentativa_atual += 1;
        
        // Verifica se esgotou tentativas
        if tentativa_atual > max_tentativas {
            println!("\n💔 Suas tentativas acabaram!");
            break 'jogo;
        }
        
        // Exibe cabeçalho da tentativa
        println!("─────────────────────────────────────");
        println!("Tentativa {}/{}", tentativa_atual, max_tentativas);
        print!("Seu palpite: ");
        io::stdout().flush().unwrap();
        
        // Lê entrada do usuário
        let mut palpite_str = String::new();
        io::stdin()
            .read_line(&mut palpite_str)
            .expect("Falha ao ler entrada");
        
        // Parse do palpite
        let palpite: i32 = match palpite_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Isso não é um número válido! Tente novamente.\n");
                tentativa_atual -= 1; // Não conta como tentativa
                continue;
            }
        };
        
        // Validação de range
        if palpite < 1 || palpite > 100 {
            println!("❌ O número deve estar entre 1 e 100!\n");
            tentativa_atual -= 1; // Não conta como tentativa
            continue;
        }
        
        // Verifica se já tentou esse número
        if historico.contains(&palpite) {
            println!("⚠️  Você já tentou {} antes!\n", palpite);
            tentativa_atual -= 1; // Não conta como tentativa
            continue;
        }
        
        // Adiciona ao histórico
        historico.push(palpite);
        
        // Compara com número secreto
        match palpite.cmp(&numero_secreto) {
            Ordering::Less => {
                let diferenca = numero_secreto - palpite;
                
                let dica = match diferenca {
                    1..=5   => "Muito perto! 🔥",
                    6..=15  => "Perto! 🌡️",
                    16..=30 => "Um pouco longe... 🧊",
                    _       => "Muito longe! ❄️"
                };
                
                println!("📈 Muito baixo! {}", dica);
            },
            Ordering::Greater => {
                let diferenca = palpite - numero_secreto;
                
                let dica = match diferenca {
                    1..=5   => "Muito perto! 🔥",
                    6..=15  => "Perto! 🌡️",
                    16..=30 => "Um pouco longe... 🧊",
                    _       => "Muito longe! ❄️"
                };
                
                println!("📉 Muito alto! {}", dica);
            },
            Ordering::Equal => {
                println!("🎉 PARABÉNS! Você acertou!");
                acertou = true;
                break 'jogo;
            }
        }
        
        // Exibe histórico
        print!("Tentativas anteriores: ");
        for (i, &num) in historico.iter().enumerate() {
            print!("{}", num);
            if i < historico.len() - 1 {
                print!(", ");
            }
        }
        println!("\n");
    }
    
    // Resultado final
    println!("\n╔═══════════════════════════════════════╗");
    println!("║           RESULTADO FINAL            ║");
    println!("╚═══════════════════════════════════════╝");
    println!("Número secreto: {}", numero_secreto);
    println!("Tentativas usadas: {}", tentativa_atual);
    
    if acertou {
        let pontuacao = match tentativa_atual {
            1 => "INCRÍVEL! Primeira tentativa! 🏆",
            2..=3 => "EXCELENTE! Muito rápido! 🥇",
            4..=5 => "MUITO BOM! 🥈",
            6..=7 => "BOM! Conseguiu no limite! 🥉",
            _ => "Parabéns!"
        };
        println!("Classificação: {}", pontuacao);
    } else {
        println!("Não foi desta vez... Tente novamente!");
    }
    
    println!("\nHistórico completo: {:?}", historico);
}