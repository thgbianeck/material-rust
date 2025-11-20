// ========================================
// VALIDAÇÃO DE CPF
// ========================================

/// Remove formatação do CPF (pontos e hífen)
fn limpar_cpf(cpf: &str) -> String {
    cpf.chars()
        .filter(|c| c.is_digit(10))
        .collect()
}

/// Verifica se CPF tem apenas dígitos repetidos
fn cpf_tem_digitos_repetidos(cpf: &str) -> bool {
    let primeiro = cpf.chars().next().unwrap();
    cpf.chars().all(|c| c == primeiro)
}

/// Calcula um dígito verificador do CPF
fn calcular_digito_cpf(cpf: &[u32], multiplicadores: &[u32]) -> u32 {
    let soma: u32 = cpf.iter()
        .zip(multiplicadores.iter())
        .map(|(digito, mult)| digito * mult)
        .sum();
    
    let resto = soma % 11;
    if resto < 2 { 0 } else { 11 - resto }
}

/// Valida CPF completo
/// Retorna (valido: bool, mensagem: String)
fn validar_cpf(cpf: &str) -> (bool, String) {
    let cpf_limpo = limpar_cpf(cpf);
    
    // Verifica tamanho
    if cpf_limpo.len() != 11 {
        return (false, String::from("CPF deve ter 11 dígitos"));
    }
    
    // Verifica se é sequência repetida (111.111.111-11)
    if cpf_tem_digitos_repetidos(&cpf_limpo) {
        return (false, String::from("CPF não pode ser sequência repetida"));
    }
    
    // Converte para vetor de números
    let digitos: Vec<u32> = cpf_limpo
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    
    // Calcula primeiro dígito verificador
    let multiplicadores1 = vec![10, 9, 8, 7, 6, 5, 4, 3, 2];
    let digito1 = calcular_digito_cpf(&digitos[0..9], &multiplicadores1);
    
    if digito1 != digitos[9] {
        return (false, String::from("Primeiro dígito verificador inválido"));
    }
    
    // Calcula segundo dígito verificador
    let multiplicadores2 = vec![11, 10, 9, 8, 7, 6, 5, 4, 3, 2];
    let digito2 = calcular_digito_cpf(&digitos[0..10], &multiplicadores2);
    
    if digito2 != digitos[10] {
        return (false, String::from("Segundo dígito verificador inválido"));
    }
    
    (true, String::from("✅ CPF válido!"))
}

// ========================================
// VALIDAÇÃO DE EMAIL
// ========================================

fn validar_email(email: &str) -> (bool, String) {
    let email = email.trim();
    
    // Verifica se está vazio
    if email.is_empty() {
        return (false, String::from("Email não pode estar vazio"));
    }
    
    // Verifica se tem exatamente um @
    let partes: Vec<&str> = email.split('@').collect();
    if partes.len() != 2 {
        return (false, String::from("Email deve conter exatamente um @"));
    }
    
    let usuario = partes[0];
    let dominio = partes[1];
    
    // Valida parte do usuário
    if usuario.is_empty() {
        return (false, String::from("Usuário não pode estar vazio"));
    }
    
    // Valida domínio
    if dominio.is_empty() {
        return (false, String::from("Domínio não pode estar vazio"));
    }
    
    if !dominio.contains('.') {
        return (false, String::from("Domínio deve conter pelo menos um ponto"));
    }
    
    // Verifica se domínio não começa ou termina com ponto
    if dominio.starts_with('.') || dominio.ends_with('.') {
        return (false, String::from("Domínio não pode começar ou terminar com ponto"));
    }
    
    // Verifica caracteres válidos no usuário
    if !usuario.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '_' || c == '-') {
        return (false, String::from("Usuário contém caracteres inválidos"));
    }
    
    (true, String::from("✅ Email válido!"))
}

// ========================================
// VALIDAÇÃO DE SENHA
// ========================================

/// Retorna (valido: bool, problemas: Vec<String>)
fn validar_senha(senha: &str) -> (bool, Vec<String>) {
    let mut problemas = Vec::new();
    
    // Tamanho mínimo
    if senha.len() < 8 {
        problemas.push(String::from("❌ Mínimo 8 caracteres"));
    }
    
    // Letra maiúscula
    if !senha.chars().any(|c| c.is_uppercase()) {
        problemas.push(String::from("❌ Pelo menos uma letra MAIÚSCULA"));
    }
    
    // Letra minúscula
    if !senha.chars().any(|c| c.is_lowercase()) {
        problemas.push(String::from("❌ Pelo menos uma letra minúscula"));
    }
    
    // Número
    if !senha.chars().any(|c| c.is_digit(10)) {
        problemas.push(String::from("❌ Pelo menos um número"));
    }
    
    // Caractere especial
    let especiais = "!@#$%^&*()_+-=[]{}|;:,.<>?";
    if !senha.chars().any(|c| especiais.contains(c)) {
        problemas.push(String::from("❌ Pelo menos um caractere especial (!@#$%...)"));
    }
    
    let valido = problemas.is_empty();
    (valido, problemas)
}

/// Calcula força da senha (0-5)
fn calcular_forca_senha(senha: &str) -> u8 {
    let mut forca = 0;
    
    if senha.len() >= 8 { forca += 1; }
    if senha.len() >= 12 { forca += 1; }
    if senha.chars().any(|c| c.is_uppercase()) { forca += 1; }
    if senha.chars().any(|c| c.is_lowercase()) { forca += 1; }
    if senha.chars().any(|c| c.is_digit(10)) { forca += 1; }
    
    let especiais = "!@#$%^&*()_+-=[]{}|;:,.<>?";
    if senha.chars().any(|c| especiais.contains(c)) { forca += 1; }
    
    forca.min(5)
}

fn descricao_forca(forca: u8) -> &'static str {
    match forca {
        0..=1 => "Muito Fraca 😱",
        2 => "Fraca 😟",
        3 => "Razoável 😐",
        4 => "Forte 😊",
        5 => "Muito Forte 💪",
        _ => "Desconhecida"
    }
}

// ========================================
// INTERFACE
// ========================================

use std::io::{self, Write};

fn ler_linha(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler entrada");
    
    input.trim().to_string()
}

fn menu_principal() {
    println!("\n╔════════════════════════════════╗");
    println!("║  ✅ SISTEMA DE VALIDAÇÃO 🦀   ║");
    println!("╠════════════════════════════════╣");
    println!("║ 1. Validar CPF                 ║");
    println!("║ 2. Validar Email               ║");
    println!("║ 3. Validar Senha               ║");
    println!("║ 0. Sair                        ║");
    println!("╚════════════════════════════════╝");
}

fn main() {
    println!("🦀 Bem-vindo ao Sistema de Validação!");
    
    loop {
        menu_principal();
        let opcao = ler_linha("\nEscolha uma opção: ");
        
        match opcao.as_str() {
            "1" => {
                let cpf = ler_linha("\n📄 Digite o CPF (000.000.000-00): ");
                let (valido, mensagem) = validar_cpf(&cpf);
                
                if valido {
                    println!("{}", mensagem);
                } else {
                    println!("❌ CPF inválido: {}", mensagem);
                }
            },
            
            "2" => {
                let email = ler_linha("\n📧 Digite o email: ");
                let (valido, mensagem) = validar_email(&email);
                
                if valido {
                    println!("{}", mensagem);
                } else {
                    println!("❌ Email inválido: {}", mensagem);
                }
            },
            
            "3" => {
                let senha = ler_linha("\n🔒 Digite a senha: ");
                let (valido, problemas) = validar_senha(&senha);
                
                if valido {
                    let forca = calcular_forca_senha(&senha);
                    println!("\n✅ Senha válida!");
                    println!("🔒 Força: {} - {}", forca, descricao_forca(forca));
                } else {
                    println!("\n❌ Senha inválida:");
                    for problema in problemas {
                        println!("  {}", problema);
                    }
                    
                    let forca = calcular_forca_senha(&senha);
                    println!("\n🔒 Força atual: {} - {}", forca, descricao_forca(forca));
                }
            },
            
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
    fn test_validar_cpf_valido() {
        assert!(validar_cpf("111.444.777-35").0);
        assert!(validar_cpf("11144477735").0);
    }
    
    #[test]
    fn test_validar_cpf_invalido() {
        assert!(!validar_cpf("111.111.111-11").0); // Sequência repetida
        assert!(!validar_cpf("123.456.789-00").0); // Dígitos errados
        assert!(!validar_cpf("123").0); // Tamanho errado
    }
    
    #[test]
    fn test_validar_email_valido() {
        assert!(validar_email("teste@exemplo.com").0);
        assert!(validar_email("usuario.nome@dominio.com.br").0);
    }
    
    #[test]
    fn test_validar_email_invalido() {
        assert!(!validar_email("semArroba.com").0);
        assert!(!validar_email("@semUsuario.com").0);
        assert!(!validar_email("semDominio@").0);
        assert!(!validar_email("sem.ponto@dominio").0);
    }
    
    #[test]
    fn test_validar_senha() {
        assert!(validar_senha("SenhaForte123!").0);
        assert!(!validar_senha("fraca").0);
        assert!(!validar_senha("SemNumero!").0);
        assert!(!validar_senha("semnumero123!").0);
    }
    
    #[test]
    fn test_forca_senha() {
        assert_eq!(calcular_forca_senha("123"), 1);
        assert_eq!(calcular_forca_senha("Senha123!"), 5);
        assert_eq!(calcular_forca_senha("SenhaFraca"), 3);
    }
}