use std::mem;

fn main() {
    println!("╔═════════════════════════════════════════════╗");
    println!("║   ANALISADOR DE TIPOS E CASTING - RUST 🦀  ║");
    println!("╚═════════════════════════════════════════════╝\n");
    
    // Análise de tipos inteiros
    analisar_tipos_inteiros();
    
    // Análise de tipos flutuantes
    analisar_tipos_flutuantes();
    
    // Análise de booleanos
    analisar_booleanos();
    
    // Análise de caracteres
    analisar_caracteres();
    
    // Demonstração de casting
    demonstrar_casting();
    
    // Demonstração de overflow
    demonstrar_overflow();
    
    // Demonstração de parse
    demonstrar_parse();
}

/// Analisa todos os tipos inteiros
fn analisar_tipos_inteiros() {
    println!("┌─────────────────────────────────────────────┐");
    println!("│          TIPOS INTEIROS                     │");
    println!("├─────────┬──────────┬──────────────────────┤");
    println!("│ Tipo    │ Tamanho  │ Faixa de Valores     │");
    println!("├─────────┼──────────┼──────────────────────┤");
    
    // i8
    println!("│ i8      │ {} byte   │ {} a {}        │", 
        mem::size_of::<i8>(), i8::MIN, i8::MAX);
    
    // i16
    println!("│ i16     │ {} bytes │ {} a {}     │", 
        mem::size_of::<i16>(), i16::MIN, i16::MAX);
    
    // i32
    println!("│ i32     │ {} bytes │ {} a {} │", 
        mem::size_of::<i32>(), i32::MIN, i32::MAX);
    
    // i64
    println!("│ i64     │ {} bytes │ {} a {}│", 
        mem::size_of::<i64>(), i64::MIN, i64::MAX);
    
    // i128
    println!("│ i128    │ {} bytes │ (muito grande)       │", 
        mem::size_of::<i128>());
    
    println!("├─────────┼──────────┼──────────────────────┤");
    
    // u8
    println!("│ u8      │ {} byte   │ {} a {}              │", 
        mem::size_of::<u8>(), u8::MIN, u8::MAX);
    
    // u16
    println!("│ u16     │ {} bytes │ {} a {}          │", 
        mem::size_of::<u16>(), u16::MIN, u16::MAX);
    
    // u32
    println!("│ u32     │ {} bytes │ {} a {}     │", 
        mem::size_of::<u32>(), u32::MIN, u32::MAX);
    
    // u64
    println!("│ u64     │ {} bytes │ {} a {}│", 
        mem::size_of::<u64>(), u64::MIN, u64::MAX);
    
    // u128
    println!("│ u128    │ {} bytes │ (muito grande)       │", 
        mem::size_of::<u128>());
    
    println!("└─────────┴──────────┴──────────────────────┘\n");
}

/// Analisa tipos flutuantes
fn analisar_tipos_flutuantes() {
    println!("┌──────────────────────────────────────────┐");
    println!("│      TIPOS DE PONTO FLUTUANTE            │");
    println!("├──────┬──────────┬────────────────────────┤");
    println!("│ Tipo │ Tamanho  │ Características        │");
    println!("├──────┼──────────┼────────────────────────┤");
    
    println!("│ f32  │ {} bytes │ ~7 dígitos decimais    │", mem::size_of::<f32>());
    println!("│ f64  │ {} bytes │ ~15 dígitos decimais   │", mem::size_of::<f64>());
    
    println!("└──────┴──────────┴────────────────────────┘");
    
    // Demonstração de precisão
    let f32_num: f32 = 0.1 + 0.2;
    let f64_num: f64 = 0.1 + 0.2;
    
    println!("\nPrecisão:");
    println!("  f32: 0.1 + 0.2 = {:.20}", f32_num);
    println!("  f64: 0.1 + 0.2 = {:.20}\n", f64_num);
}

/// Analisa booleanos
fn analisar_booleanos() {
    println!("┌──────────────────────────────────┐");
    println!("│         TIPO BOOLEANO            │");
    println!("├──────┬──────────┬────────────────┤");
    println!("│ Tipo │ Tamanho  │ Valores        │");
    println!("├──────┼──────────┼────────────────┤");
    println!("│ bool │ {} byte   │ true, false    │", mem::size_of::<bool>());
    println!("└──────┴──────────┴────────────────┘");
    
    // Operações booleanas
    let a = true;
    let b = false;
    
    println!("\nOperações:");
    println!("  a = {}, b = {}", a, b);
    println!("  a && b = {}", a && b);  // AND
    println!("  a || b = {}", a || b);  // OR
    println!("  !a = {}", !a);          // NOT
    println!("  a ^ b = {}\n", a ^ b);  // XOR
}

/// Analisa caracteres
fn analisar_caracteres() {
    println!("┌──────────────────────────────────────────┐");
    println!("│           TIPO CARACTERE                 │");
    println!("├──────┬──────────┬────────────────────────┤");
    println!("│ Tipo │ Tamanho  │ Representa             │");
    println!("├──────┼──────────┼────────────────────────┤");
    println!("│ char │ {} bytes │ Unicode Scalar Value   │", mem::size_of::<char>());
    println!("└──────┴──────────┴────────────────────────┘");
    
    // Exemplos de caracteres
    let exemplos = vec![
        ('A', "Letra ASCII"),
        ('中', "Caractere Chinês"),
        ('😎', "Emoji"),
        ('€', "Símbolo Euro"),
        ('♠', "Símbolo Naipe"),
    ];
    
    println!("\nExemplos:");
    for (c, descricao) in exemplos {
        let codigo = c as u32;
        println!("  '{}' - {} (U+{:04X}) - {} bytes UTF-8", 
            c, descricao, codigo, c.len_utf8());
    }
    println!();
}

/// Demonstra casting entre tipos
fn demonstrar_casting() {
    println!("┌─────────────────────────────────────────────┐");
    println!("│         DEMONSTRAÇÃO DE CASTING             │");
    println!("└─────────────────────────────────────────────┘");
    
    // Int para float
    let inteiro: i32 = 42;
    let flutuante = inteiro as f64;
    println!("\n1️⃣  Inteiro → Flutuante:");
    println!("   i32: {} → f64: {}", inteiro, flutuante);
    
    // Float para int (trunca, não arredonda!)
    let pi: f64 = 3.14159;
    let pi_int = pi as i32;
    println!("\n2️⃣  Flutuante → Inteiro (trunca):");
    println!("   f64: {} → i32: {}", pi, pi_int);
    
    // Casting com perda de informação
    let grande: i64 = 1000;
    let pequeno = grande as i8;  // i8: -128 a 127
    println!("\n3️⃣  Casting com overflow:");
    println!("   i64: {} → i8: {} (overflow!)", grande, pequeno);
    
    // Casting seguro com try_into
    use std::convert::TryInto;
    
    let valor: i32 = 100;
    let resultado: Result<i8, _> = valor.try_into();
    
    println!("\n4️⃣  Casting seguro (try_into):");
    match resultado {
        Ok(v) => println!("   i32: {} → i8: {} ✅", valor, v),
        Err(_) => println!("   Conversão falhou! ❌"),
    }
    
    // Char para int
    let letra: char = 'A';
    let codigo = letra as u32;
    println!("\n5️⃣  Char → Código Unicode:");
    println!("   char: '{}' → u32: {} (U+{:04X})", letra, codigo, codigo);
    
    // Int para char
    let novo_char = char::from_u32(66).unwrap();
    println!("\n6️⃣  Código Unicode → Char:");
    println!("   u32: 66 → char: '{}'", novo_char);
    
    println!();
}

/// Demonstra overflow e underflow
fn demonstrar_overflow() {
    println!("┌─────────────────────────────────────────────┐");
    println!("│       DEMONSTRAÇÃO DE OVERFLOW              │");
    println!("└─────────────────────────────────────────────┘\n");
    
    // Métodos de overflow
    let x: u8 = 255;
    
    println!("Valor inicial: u8 = {}", x);
    println!("\nMétodos de adição (+1):\n");
    
    // 1. wrapping_* : faz overflow circular
    let wrapped = x.wrapping_add(1);
    println!("1️⃣  wrapping_add(1): {} → {} (circula)", x, wrapped);
    
    // 2. checked_* : retorna Option
    let checked = x.checked_add(1);
    println!("2️⃣  checked_add(1): {:?} (None = overflow)", checked);
    
    // 3. saturating_* : satura no máximo/mínimo
    let saturated = x.saturating_add(1);
    println!("3️⃣  saturating_add(1): {} → {} (satura)", x, saturated);
    
    // 4. overflowing_* : retorna (resultado, bool)
    let (value, overflowed) = x.overflowing_add(1);
    println!("4️⃣  overflowing_add(1): ({}, overflow={})", value, overflowed);
    
    // Underflow
    println!("\n\nUnderflow (subtração):\n");
    let y: u8 = 0;
    
    println!("Valor inicial: u8 = {}", y);
    println!("\nMétodos de subtração (-1):\n");
    
    let wrapped = y.wrapping_sub(1);
    println!("1️⃣  wrapping_sub(1): {} → {} (circula)", y, wrapped);
    
    let checked = y.checked_sub(1);
    println!("2️⃣  checked_sub(1): {:?} (None = underflow)", checked);
    
    let saturated = y.saturating_sub(1);
    println!("3️⃣  saturating_sub(1): {} → {} (satura)", y, saturated);
    
    println!();
}

/// Demonstra parse de strings
fn demonstrar_parse() {
    println!("┌─────────────────────────────────────────────┐");
    println!("│         DEMONSTRAÇÃO DE PARSE               │");
    println!("└─────────────────────────────────────────────┘\n");
    
    // Parse bem-sucedido
    let texto1 = "42";
    let numero1: i32 = texto1.parse().unwrap();
    println!("✅ Parse sucesso: \"{}\" → {}", texto1, numero1);
    
    // Parse com turbofish
    let texto2 = "3.14159";
    let numero2 = texto2.parse::<f64>().unwrap();
    println!("✅ Parse turbofish: \"{}\" → {}", texto2, numero2);
    
    // Parse com erro
    let texto3 = "não é número";
    let resultado3 = texto3.parse::<i32>();
    println!("\n❌ Parse com erro: \"{}\"", texto3);
    match resultado3 {
        Ok(n) => println!("   Número: {}", n),
        Err(e) => println!("   Erro: {}", e),
    }
    
    // Parse com valor padrão
    let texto4 = "abc";
    let numero4 = texto4.parse::<i32>().unwrap_or(0);
    println!("\n🔄 Parse com fallback: \"{}\" → {} (padrão)", texto4, numero4);
    
    // Parse de diferentes tipos
    println!("\n📦 Parse de múltiplos tipos:");
    
    let textos = vec![
        ("123", "i32"),
        ("45.67", "f64"),
        ("true", "bool"),
        ("-999", "i32"),
    ];
    
    for (texto, tipo) in textos {
        match tipo {
            "i32" => {
                let n: i32 = texto.parse().unwrap();
                println!("   \"{}\" ({}): {}", texto, tipo, n);
            },
            "f64" => {
                let n: f64 = texto.parse().unwrap();
                println!("   \"{}\" ({}): {}", texto, tipo, n);
            },
            "bool" => {
                let n: bool = texto.parse().unwrap();
                println!("   \"{}\" ({}): {}", texto, tipo, n);
            },
            _ => {},
        }
    }
    
    // Parse com limpeza
    println!("\n🧹 Parse com limpeza:");
    let texto_sujo = "  42  \n\t";
    let limpo = texto_sujo.trim().parse::<i32>().unwrap();
    println!("   \"{}\" (trim) → {}", texto_sujo.escape_default(), limpo);
    
    println!();
}