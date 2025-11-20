use std::io::{self, Write};

#[derive(Debug)] // Para facilitar depuração
struct Usuario { // Struct para armazenar dados do usuário
    nome: String,
    email: String,
    idade: u32,
}

impl Usuario { // Implementação de métodos para Usuario
    fn novo(nome: String, email: String, idade: u32) -> Self { // Construtor
        Usuario {
            nome: capitalizar(&nome),
            email: email.to_lowercase(),
            idade,
        }
    }
    
    fn exibir(&self) { // Exibe dados do usuário formatados
        println!("\n{:-^50}", " DADOS DO USUÁRIO ");
        println!("{:<15} : {}", "Nome", self.nome);
        println!("{:<15} : {}", "Email", self.email);
        println!("{:<15} : {}", "Idade", self.idade);
        println!("{:-^50}", "");
    }
}

fn input(prompt: &str) -> String { // Função para ler entrada do usuário com prompt
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Falha ao ler entrada");
    
    buffer.trim().to_string()
}

fn validar_email(email: &str) -> bool { // Validação simples de email
    if email.is_empty() {
        return false;
    }
    
    let tem_arroba = email.contains('@');
    let tem_ponto = email.contains('.');
    let arroba_antes_ponto = match (email.find('@'), email.find('.')) {
        (Some(arroba), Some(ponto)) => arroba < ponto,
        _ => false,
    };
    
    tem_arroba && tem_ponto && arroba_antes_ponto
}

fn capitalizar(texto: &str) -> String { // Função para capitalizar texto
    let limpo = texto.trim();
    
    if limpo.is_empty() {
        return String::new();
    }
    
    let mut resultado = String::new();
    let mut primeira = true;
    
    for palavra in limpo.split_whitespace() {
        if !primeira {
            resultado.push(' ');
        }
        
        let mut chars = palavra.chars();
        if let Some(primeiro_char) = chars.next() {
            resultado.push_str(&primeiro_char.to_uppercase().to_string());
            resultado.push_str(&chars.as_str().to_lowercase());
        }
        
        primeira = false;
    }
    
    resultado
}

fn ler_idade() -> Option<u32> { // Lê e valida idade do usuário
    loop {
        let input = input("Idade: ");
        
        match input.parse::<u32>() {
            Ok(idade) if idade > 0 && idade < 150 => return Some(idade),
            Ok(_) => println!("❌ Idade deve estar entre 1 e 149!"),
            Err(_) => println!("❌ Digite um número válido!"),
        }
        
        print!("Tentar novamente? (s/n): ");
        io::stdout().flush().unwrap();
        
        let mut resposta = String::new();
        io::stdin().read_line(&mut resposta).unwrap();
        
        if !resposta.trim().eq_ignore_ascii_case("s") {
            return None;
        }
    }
}

fn cadastrar_usuario() -> Option<Usuario> { // Função para cadastrar novo usuário
    println!("\n{:=^50}", " NOVO CADASTRO ");
    
    let nome = input("Nome completo: ");
    if nome.is_empty() {
        println!("❌ Nome não pode ser vazio!");
        return None;
    }
    
    let email = loop {
        let email = input("Email: ");
        
        if validar_email(&email) {
            break email;
        }
        
        println!("❌ Email inválido! Deve conter @ e .");
        print!("Tentar novamente? (s/n): ");
        io::stdout().flush().unwrap();
        
        let mut resposta = String::new();
        io::stdin().read_line(&mut resposta).unwrap();
        
        if !resposta.trim().eq_ignore_ascii_case("s") {
            return None;
        }
    };
    
    let idade = ler_idade()?;
    
    Some(Usuario::novo(nome, email, idade))
}

fn listar_usuarios(usuarios: &[Usuario]) { // Lista todos os usuários cadastrados
    if usuarios.is_empty() {
        println!("\n⚠️  Nenhum usuário cadastrado.");
        return;
    }
    
    println!("\n{:=^70}", " LISTA DE USUÁRIOS ");
    println!("{:<25} {:<30} {:>10}", "NOME", "EMAIL", "IDADE");
    println!("{:-<70}", "");
    
    for (i, usuario) in usuarios.iter().enumerate() {
        println!(
            "{}. {:<23} {:<30} {:>10}",
            i + 1,
            usuario.nome,
            usuario.email,
            usuario.idade
        );
    }
    
    println!("{:=^70}", "");
    println!("\nTotal de usuários: {}", usuarios.len());
}

fn menu() -> Option<char> { // Exibe menu e lê escolha do usuário
    println!("\n{:*^50}", " MENU ");
    println!("1. Cadastrar novo usuário");
    println!("2. Listar todos os usuários");
    println!("3. Sair");
    print!("\nEscolha uma opção: ");
    io::stdout().flush().unwrap();
    
    let mut escolha = String::new();
    io::stdin().read_line(&mut escolha).ok()?;
    
    escolha.trim().chars().next()
}

fn main() { // Função principal do programa de cadastro
    let mut usuarios: Vec<Usuario> = Vec::new();
    
    println!("{:*^50}", " SISTEMA DE CADASTRO ");
    
    loop {
        match menu() {
            Some('1') => {
                if let Some(usuario) = cadastrar_usuario() {
                    usuario.exibir();
                    usuarios.push(usuario);
                    println!("\n✅ Usuário cadastrado com sucesso!");
                } else {
                    println!("\n⚠️  Cadastro cancelado.");
                }
            }
            Some('2') => {
                listar_usuarios(&usuarios);
            }
            Some('3') => {
                println!("\n👋 Encerrando sistema...");
                listar_usuarios(&usuarios);
                break;
            }
            _ => {
                println!("\n❌ Opção inválida!");
            }
        }
    }
}