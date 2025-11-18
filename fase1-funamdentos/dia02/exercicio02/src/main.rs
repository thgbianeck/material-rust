use exercicio02::{EscalaTemperatura, Temperatura};
use std::io;

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║  CONVERSOR DE TEMPERATURA - RUST 🦀   ║");
    println!("╚════════════════════════════════════════╝\n");

    loop {
        // Mostra menu
        exibir_menu();

        // Lê opção
        let opcao = ler_opcao();

        if opcao == 0 {
            println!("\n👋 Até logo!");
            break;
        }

        // Valida opção
        if opcao < 1 || opcao > 6 {
            println!("\n❌ Opção inválida! Tente novamente.\n");
            continue;
        }

        // Lê temperatura
        print!("Digite a temperatura: ");
        io::Write::flush(&mut io::stdout()).unwrap();
        let valor = ler_numero();

        // Determina escalas de origem e destino
        let (escala_origem, escala_destino) = match opcao {
            1 => (EscalaTemperatura::Celsius, EscalaTemperatura::Fahrenheit),
            2 => (EscalaTemperatura::Celsius, EscalaTemperatura::Kelvin),
            3 => (EscalaTemperatura::Fahrenheit, EscalaTemperatura::Celsius),
            4 => (EscalaTemperatura::Fahrenheit, EscalaTemperatura::Kelvin),
            5 => (EscalaTemperatura::Kelvin, EscalaTemperatura::Celsius),
            6 => (EscalaTemperatura::Kelvin, EscalaTemperatura::Fahrenheit),
            _ => unreachable!(), // Nunca vai chegar aqui
        };

        // Cria temperatura e converte
        let temp_origem = Temperatura::new(valor, escala_origem);
        let temp_destino = temp_origem.converter_para(escala_destino);

        // Exibe resultado
        println!("\n┌─────────────────────────────┐");
        println!("│ RESULTADO DA CONVERSÃO      │");
        println!("├─────────────────────────────┤");
        println!("│ Origem:  {}  │", temp_origem);
        println!("│ Destino: {}  │", temp_destino);
        println!("└─────────────────────────────┘\n");

        // Mostra tabela de comparação
        exibir_tabela_comparacao(&temp_origem);

        println!();
    }
}

/// Exibe menu de opções
fn exibir_menu() {
    println!("CONVERSÕES DISPONÍVEIS:");
    println!("  1. Celsius → Fahrenheit");
    println!("  2. Celsius → Kelvin");
    println!("  3. Fahrenheit → Celsius");
    println!("  4. Fahrenheit → Kelvin");
    println!("  5. Kelvin → Celsius");
    println!("  6. Kelvin → Fahrenheit");
    println!("  0. Sair\n");
}

/// Lê opção do menu
fn ler_opcao() -> u8 {
    print!("Escolha uma opção: ");
    io::Write::flush(&mut io::stdout()).unwrap();

    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Falha ao ler entrada");

    entrada.trim().parse().unwrap_or(99)
}

/// Lê número do usuário
fn ler_numero() -> f64 {
    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Falha ao ler entrada");

    entrada.trim().parse().unwrap_or(0.0)
}

/// Exibe tabela de comparação com todas as escalas
fn exibir_tabela_comparacao(temp: &Temperatura) {
    println!("TABELA DE COMPARAÇÃO:");
    println!("┌────────────────┬──────────────┐");
    println!("│ Escala         │ Valor        │");
    println!("├────────────────┼──────────────┤");
    println!("│ Celsius        │ {:>8.2}°C  │", temp.para_celsius());
    println!("│ Fahrenheit     │ {:>8.2}°F  │", temp.para_fahrenheit());
    println!("│ Kelvin         │ {:>8.2}K   │", temp.para_kelvin());
    println!("└────────────────┴──────────────┘");
}
