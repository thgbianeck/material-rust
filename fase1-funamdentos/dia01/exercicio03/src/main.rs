fn main() {
    // ========================================
    // DADOS DO USUÁRIO
    // ========================================
    
    let usuario_nome = "Thiago Bianeck";
    let usuario_login = "bianeck";
    let usuario_nivel = "Sênior";
    let linguagens_dominadas = vec!["Java", "JavaScript", "SQL"];  // Vec = Vector (lista)
    let linguagem_aprendendo = "Rust";
    
    // Estatísticas de estudo
    let dias_total_curso = 60;
    let dias_completos = 1;
    let horas_hoje = 3.5;
    let exercicios_concluidos = 3;
    let exercicios_totais = 180;  // 3 exercícios × 60 dias
    
    // ========================================
    // CÁLCULOS
    // ========================================
    
    // Conversão para float para divisão exata
    let progresso_dias = (dias_completos as f64 / dias_total_curso as f64) * 100.0;
    let progresso_exercicios = (exercicios_concluidos as f64 / exercicios_totais as f64) * 100.0;
    
    // Dias restantes
    let dias_restantes = dias_total_curso - dias_completos;
    
    // Estimativa de horas totais (assumindo 3h/dia)
    let horas_estimadas_totais = dias_total_curso as f64 * 3.0;
    let horas_investidas = dias_completos as f64 * 3.0 + horas_hoje;
    let progresso_horas = (horas_investidas / horas_estimadas_totais) * 100.0;
    
    // ========================================
    // BANNER ASCII ART
    // ========================================
    
    println!("\n");
    println!("██████╗ ██╗   ██╗███████╗████████╗");
    println!("██╔══██╗██║   ██║██╔════╝╚══██╔══╝");
    println!("██████╔╝██║   ██║███████╗   ██║   ");
    println!("██╔══██╗██║   ██║╚════██║   ██║   ");
    println!("██║  ██║╚██████╔╝███████║   ██║   ");
    println!("╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   ");
    println!("    🦀 SISTEMA DE APRENDIZADO 🦀");
    println!("\n═══════════════════════════════════════════════════════\n");
    
    // ========================================
    // INFORMAÇÕES DO USUÁRIO
    // ========================================
    
    println!("┌─────────────────────────────────────────────────────┐");
    println!("│  👤 PERFIL DO DESENVOLVEDOR                         │");
    println!("├─────────────────────────────────────────────────────┤");
    println!("│  Nome:       {:<40}│", usuario_nome);
    println!("│  Login:      {:<40}│", usuario_login);
    println!("│  Nível:      {:<40}│", usuario_nivel);
    
    // Formatação de lista (convertendo Vec para string)
    // join() junta elementos com um separador
    let langs_str = linguagens_dominadas.join(", ");
    println!("│  Domínio:    {:<40}│", langs_str);
    println!("│  Aprendendo: {:<40}│", linguagem_aprendendo);
    println!("└─────────────────────────────────────────────────────┘");
    
    // ========================================
    // ESTATÍSTICAS DE PROGRESSO
    // ========================================
    
    println!("\n┌─────────────────────────────────────────────────────┐");
    println!("│  📊 ESTATÍSTICAS DO CURSO                           │");
    println!("├─────────────────────────────────────────────────────┤");
    
    // Usando formatação com alinhamento
    // {:<30} = alinhado à esquerda com 30 caracteres
    // {:>6} = alinhado à direita com 6 caracteres
    println!("│  {:<35} {:>3}/{:<3} dias     │", 
             "Dias completos:", dias_completos, dias_total_curso);
    println!("│  {:<35} {: >15} dias │", 
             "Dias restantes:", dias_restantes);
    println!("│  {:<35} {:>18.1}h │", 
             "Horas hoje:", horas_hoje);
    println!("│  {:<35} {:>18.1}h │", 
             "Total de horas investidas:", horas_investidas);
    
    println!("│                                                     │");
    
    println!("│  {:<35} {:>3}/{:<3} ex.     │", 
             "Exercícios concluídos:", exercicios_concluidos, exercicios_totais);
    
    println!("└─────────────────────────────────────────────────────┘");
    
    // ========================================
    // BARRAS DE PROGRESSO
    // ========================================
    
    println!("\n┌─────────────────────────────────────────────────────┐");
    println!("│  🎯 PROGRESSO GERAL                                 │");
    println!("├─────────────────────────────────────────────────────┤");
    
    // Função auxiliar para criar barra de progresso
    // Vamos simular isso com strings repetidas
    let criar_barra = |percentual: f64| -> String {
        let largura_total = 40;
        let blocos_cheios = ((percentual / 100.0) * largura_total as f64) as usize;
        let blocos_vazios = largura_total - blocos_cheios;
        
        let mut barra = String::from("│  ");
        barra.push_str(&"█".repeat(blocos_cheios));
        barra.push_str(&"░".repeat(blocos_vazios));
        barra.push_str(&format!(" {:>5.1}% │", percentual));
        barra
    };
    
    println!("│  📅 Dias do curso:                                  │");
    println!("{}", criar_barra(progresso_dias));
    println!("│                                                     │");
    
    println!("│  ⏰ Horas investidas:                               │");
    println!("{}", criar_barra(progresso_horas));
    println!("│                                                     │");
    
    println!("│  ✅ Exercícios resolvidos:                          │");
    println!("{}", criar_barra(progresso_exercicios));
    
    println!("└─────────────────────────────────────────────────────┘");
    
    // ========================================
    // MENSAGEM MOTIVACIONAL
    // ========================================
    
    println!("\n╔═════════════════════════════════════════════════════╗");
    println!("║  💡 MENSAGEM DO DIA                                 ║");
    println!("╠═════════════════════════════════════════════════════╣");
    println!("║                                                     ║");
    
    // Mensagem condicional baseada no progresso
    if progresso_dias < 10.0 {
        println!("║  🚀 Você está começando sua jornada!                ║");
        println!("║  Todo grande programador começou do zero.          ║");
        println!("║  Continue firme, um dia de cada vez! 💪            ║");
    } else if progresso_dias < 50.0 {
        println!("║  🔥 Você está no caminho certo!                     ║");
        println!("║  Cada linha de código te torna mais forte.        ║");
        println!("║  Mantenha o ritmo! 🎯                              ║");
    } else {
        println!("║  🏆 Você está dominando Rust!                       ║");
        println!("║  A reta final está chegando.                      ║");
        println!("║  Continue assim, campeão! 🥇                       ║");
    }
    
    println!("║                                                     ║");
    println!("╚═════════════════════════════════════════════════════╝");
    
    // ========================================
    // RODAPÉ COM TIMESTAMP SIMULADO
    // ========================================
    
    println!("\n─────────────────────────────────────────────────────────");
    println!("  📅 Dia {} de {}  |  ⏱️  {:.1}h hoje  |  🎯 {} ex. concluídos",
             dias_completos, dias_total_curso, horas_hoje, exercicios_concluidos);
    println!("─────────────────────────────────────────────────────────\n");
}