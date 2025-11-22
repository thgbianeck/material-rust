fn main() {
    println!("🎓 SISTEMA DE GERENCIAMENTO DE NOTAS\n");
    
    // 1. CRIAÇÃO: Vec com notas iniciais
    let mut notas: Vec<f64> = vec![7.5, 8.0, 6.5, 9.0, 5.5];
    println!("📊 Notas iniciais: {:?}", notas);
    println!("Total de notas: {}\n", notas.len());
    
    // 2. CREATE: Adicionar novas notas
    println!("➕ Adicionando notas...");
    notas.push(8.5);
    notas.push(7.0);
    println!("Notas após push: {:?}\n", notas);
    
    // 3. READ: Acesso seguro vs direto
    println!("🔍 LEITURA DE NOTAS:");
    
    // Acesso direto (pode dar panic)
    let primeira = notas[0];
    println!("Primeira nota (indexação direta): {}", primeira);
    
    // Acesso seguro com get()
    match notas.get(2) {
        Some(nota) => println!("Terceira nota (get): {}", nota),
        None => println!("Índice inválido!"),
    }
    
    // Tentando acessar índice inválido com segurança
    match notas.get(100) {
        Some(nota) => println!("Nota: {}", nota),
        None => println!("❌ Índice 100 não existe (get retornou None)"),
    }
    println!();
    
    // 4. ITERAÇÃO: Diferentes formas
    println!("🔄 ITERAÇÃO:");
    
    // 4.1 iter() - Emprestar imutável (só leitura)
    println!("\n📖 Listando todas as notas (iter):");
    for (i, nota) in notas.iter().enumerate() {
        println!("  Nota {}: {:.1}", i + 1, nota);
    }
    
    // 4.2 iter_mut() - Emprestar mutável (modificar)
    println!("\n🎯 Aplicando bônus de 0.5 em todas as notas (iter_mut):");
    for nota in notas.iter_mut() {
        *nota += 0.5; // Desreferencia e modifica
        if *nota > 10.0 {
            *nota = 10.0; // Limita a 10.0
        }
    }
    println!("Notas após bônus: {:?}", notas);
    
    // 4.3 Iteração com referência no for (atalho para iter)
    println!("\n📊 Contando notas acima de 8.0:");
    let mut count = 0;
    for nota in &notas {
        if *nota >= 8.0 {
            count += 1;
        }
    }
    println!("Total de notas >= 8.0: {}", count);
    println!();
    
    // 5. SLICES: Trabalhando com partes do vector
    println!("✂️ TRABALHANDO COM SLICES:");
    
    // Primeiras 3 notas
    let top_3 = &notas[0..3];
    println!("Top 3 primeiras notas: {:?}", top_3);
    
    // Últimas 2 notas
    let ultimas_2 = &notas[notas.len() - 2..];
    println!("Últimas 2 notas: {:?}", ultimas_2);
    
    // Média das 4 primeiras notas usando slice
    let slice_4 = &notas[..4];
    let media_4: f64 = slice_4.iter().sum::<f64>() / slice_4.len() as f64;
    println!("Média das 4 primeiras notas: {:.2}\n", media_4);
    
    // 6. CÁLCULOS ESTATÍSTICOS (usando iter)
    println!("📈 ESTATÍSTICAS:");
    
    let soma: f64 = notas.iter().sum();
    let media = soma / notas.len() as f64;
    println!("Média geral: {:.2}", media);
    
    let maior = notas.iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    println!("Maior nota: {:.1}", maior);
    
    let menor = notas.iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    println!("Menor nota: {:.1}\n", menor);
    
    // 7. UPDATE: Modificar nota específica
    println!("✏️ UPDATE:");
    let indice_update = 2;
    match notas.get_mut(indice_update) {
        Some(nota) => {
            println!("Nota anterior no índice {}: {:.1}", indice_update, nota);
            *nota = 9.5;
            println!("Nova nota no índice {}: {:.1}", indice_update, nota);
        }
        None => println!("Índice {} inválido!", indice_update),
    }
    println!("Notas após update: {:?}\n", notas);
    
    // 8. DELETE: Remover notas
    println!("🗑️ DELETE:");
    
    // Remover última nota com pop
    if let Some(removida) = notas.pop() {
        println!("Removida última nota: {:.1}", removida);
    }
    
    // Remover nota por índice
    if notas.len() > 1 {
        let removida = notas.remove(1);
        println!("Removida nota no índice 1: {:.1}", removida);
    }
    
    println!("Notas finais: {:?}", notas);
    println!("Total de notas restantes: {}\n", notas.len());
    
    // 9. DEMONSTRAÇÃO: into_iter() consome o vector
    println!("🚨 DEMONSTRAÇÃO DE OWNERSHIP:");
    let temp_notas = vec![10.0, 9.0, 8.0];
    
    println!("Usando into_iter() (consome o vector):");
    for nota in temp_notas.into_iter() {
        println!("  Nota: {:.1}", nota);
    }
    
    // ❌ Descomentar a linha abaixo causaria ERRO DE COMPILAÇÃO
    // println!("Tentando usar temp_notas: {:?}", temp_notas);
    println!("✅ temp_notas foi consumido e não existe mais!");
    
    println!("\n✨ Sistema finalizado!");
}