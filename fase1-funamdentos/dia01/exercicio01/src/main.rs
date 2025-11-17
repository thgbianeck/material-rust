fn main() {
    // Declaração de variáveis com 'let'
    // Em Rust, variáveis são IMUTÁVEIS por padrão (veremos isso melhor no Dia 2)
    let nome = "Thiago Bianeck";
    let idade = 40;
    let profissao = "Engenheiro de Software Sênior";
    
    // println! com placeholders {}
    // Rust infere automaticamente o tipo e formata corretamente
    println!("===================================");
    println!("       APRESENTAÇÃO PESSOAL        ");
    println!("===================================");
    println!("Nome: {}", nome);
    println!("Idade: {} anos", idade);
    println!("Profissão: {}", profissao);
    println!("===================================");
    
    // Podemos usar múltiplas variáveis em uma linha
    println!("Olá! Sou {}, tenho {} anos e trabalho como {}.", 
             nome, idade, profissao);
    
    // String literal multilinha com indentação
    println!("\n💪 Mensagem motivacional:");
    println!("\"Aprender Rust vai expandir sua mente e");
    println!(" tornar você um desenvolvedor melhor!\"");
    
    // Usando escape de caracteres
    println!("\n\t→ Vamos com tudo! 🚀");
}
