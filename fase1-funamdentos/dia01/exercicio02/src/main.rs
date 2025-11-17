fn main() {
    // ========================================
    // PARTE 1: Operações com Inteiros
    // ========================================
    
    // Em Rust, precisamos especificar o tipo se não for óbvio
    // i32 = inteiro de 32 bits (padrão do Rust)
    // Comparação: int em Java também é 32 bits
    let numero1: i32 = 42;
    let numero2: i32 = 10;
    
    println!("╔══════════════════════════════════╗");
    println!("║  CALCULADORA RUST - INTEIROS     ║");
    println!("╚══════════════════════════════════╝");
    println!("Números: {} e {}\n", numero1, numero2);
    
    // Operações básicas
    let soma = numero1 + numero2;
    let subtracao = numero1 - numero2;
    let multiplicacao = numero1 * numero2;
    let divisao = numero1 / numero2;  // Divisão inteira (trunca)
    let resto = numero1 % numero2;     // Módulo (resto da divisão)
    
    println!("➕ Soma:          {} + {} = {}", numero1, numero2, soma);
    println!("➖ Subtração:     {} - {} = {}", numero1, numero2, subtracao);
    println!("✖️  Multiplicação: {} × {} = {}", numero1, numero2, multiplicacao);
    println!("➗ Divisão:       {} ÷ {} = {}", numero1, numero2, divisao);
    println!("📐 Resto:         {} % {} = {}", numero1, numero2, resto);
    
    // ========================================
    // PARTE 2: Operações com Floats
    // ========================================
    
    println!("\n╔══════════════════════════════════╗");
    println!("║  CALCULADORA RUST - DECIMAIS     ║");
    println!("╚══════════════════════════════════╝");
    
    // f64 = float de 64 bits (padrão do Rust, como double em Java)
    let preco_produto: f64 = 127.50;
    let desconto_percentual: f64 = 15.0;
    
    // Cálculo de desconto
    let valor_desconto = preco_produto * (desconto_percentual / 100.0);
    let preco_final = preco_produto - valor_desconto;
    
    // Formatação de floats: {:.2} = 2 casas decimais
    println!("💰 Preço original: R$ {:.2}", preco_produto);
    println!("🏷️  Desconto:       {}%", desconto_percentual);
    println!("💸 Valor desconto: R$ {:.2}", valor_desconto);
    println!("✅ Preço final:    R$ {:.2}", preco_final);
    
    // ========================================
    // PARTE 3: Type Casting (Conversão)
    // ========================================
    
    println!("\n╔══════════════════════════════════╗");
    println!("║  CONVERSÃO DE TIPOS              ║");
    println!("╚══════════════════════════════════╝");
    
    let inteiro = 42;
    let float = 10.5;
    
    // Em Rust, conversão explícita é obrigatória (sem coerção automática)
    // Use 'as' para casting (similar ao cast em Java)
    let inteiro_como_float = inteiro as f64;
    let float_como_inteiro = float as i32;  // Trunca (não arredonda)
    
    println!("Inteiro {} como float: {:.1}", inteiro, inteiro_como_float);
    println!("Float {} como inteiro: {} (truncado)", float, float_como_inteiro);
    
    // Operação mista (precisa converter)
    let resultado_misto = inteiro_como_float + float;
    println!("Operação mista: {} + {} = {:.1}", inteiro, float, resultado_misto);
    
    // ========================================
    // PARTE 4: Operações Matemáticas Avançadas
    // ========================================
    
    println!("\n╔══════════════════════════════════╗");
    println!("║  FUNÇÕES MATEMÁTICAS             ║");
    println!("╚══════════════════════════════════╝");
    
    let numero: f64 = 16.0;
    
    // Funções matemáticas são métodos do tipo float
    let raiz_quadrada = numero.sqrt();      // Square root
    let potencia = numero.powf(2.0);        // Power (potência)
    let arredondado = 3.7_f64.round();      // Arredondamento
    let piso = 3.7_f64.floor();             // Piso (floor)
    let teto = 3.2_f64.ceil();              // Teto (ceil)
    
    println!("Raiz quadrada de {}: {}", numero, raiz_quadrada);
    println!("Potência {}²: {}", numero, potencia);
    println!("Arredondar 3.7: {}", arredondado);
    println!("Piso de 3.7: {}", piso);
    println!("Teto de 3.2: {}", teto);
    
    // ========================================
    // COMPARAÇÃO: Rust vs Java
    // ========================================
    
    println!("\n╔══════════════════════════════════╗");
    println!("║  🦀 RUST vs ☕ JAVA              ║");
    println!("╚══════════════════════════════════╝");
    println!("RUST:  let x: i32 = 42;    (tipo explícito)");
    println!("JAVA:  int x = 42;         (similar)");
    println!();
    println!("RUST:  let y = 42;         (inferência automática)");
    println!("JAVA:  var y = 42;         (Java 10+)");
    println!();
    println!("RUST:  42_i32, 3.14_f64    (sufixos de tipo)");
    println!("JAVA:  42, 3.14D           (literal D para double)");
}