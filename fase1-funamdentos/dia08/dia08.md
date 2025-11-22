# 🦀 DIA 8: Vec<T> - O ArrayList Turbinado do Rust

Olá, **Bianeck**! Hoje vamos mergulhar nos **Vectors**, a estrutura de dados dinâmica mais usada em Rust. Se você já domina `ArrayList` em Java, vai se sentir em casa... mas com superpoderes de segurança! 🚀

---

## 📑 Índice

**[1. O que é Vec<T>?](#1-o-que-é-vect)**  
**[2. Criação de Vectors](#2-criação-de-vectors)**  
**[3. Manipulação: Push, Pop e Cia](#3-manipulação-push-pop-e-cia)**  
**[4. Acesso Seguro vs Arriscado](#4-acesso-seguro-vs-arriscado)**  
**[5. Iteração: O Coração do Ownership](#5-iteração-o-coração-do-ownership)**  
**[6. Slices: Janelas para o Vector](#6-slices-janelas-para-o-vector)**  
**[7. Capacidade vs Comprimento](#7-capacidade-vs-comprimento)**  
**[8. Comparação Java vs Rust](#8-comparação-java-vs-rust)**  
**[9. EXERCÍCIO PRÁTICO: Sistema de Gerenciamento de Notas](#9-exercício-prático-sistema-de-gerenciamento-de-notas)**

---

## 1. O que é Vec<T>?

**Vec<T>** é a estrutura de dados **dinâmica** do Rust que armazena elementos do mesmo tipo **T** em sequência contígua na **heap**.

**Analogia Divertida:** Imagine um trem 🚂 com vagões elásticos. Você pode adicionar vagões (push), remover o último (pop), ou até inserir um vagão no meio. Mas diferente de um trem real, se você tentar acessar o vagão 1000 quando só existem 5, o Rust te avisa antes de você cair nos trilhos! 🛤️

**Em Java:**
```java
ArrayList<String> nomes = new ArrayList<>();
nomes.add("Clara");
nomes.add("Lunna");
```

**Em Rust:**
```rust {.line-numbers}
let mut nomes: Vec<String> = Vec::new();
nomes.push(String::from("Clara"));
nomes.push(String::from("Lunna"));
```

**[⬆ Voltar ao Índice](#-índice)**

---

## 2. Criação de Vectors

Existem **3 formas principais** de criar um `Vec<T>`:

### 2.1 Vec::new() - Criação Vazia

```rust {.line-numbers}
// Vector vazio (precisa de anotação de tipo)
let mut numeros: Vec<i32> = Vec::new();
numeros.push(10);
numeros.push(20);
```

### 2.2 vec![] - Macro com Valores Iniciais

```rust {.line-numbers}
// O compilador infere o tipo automaticamente
let numeros = vec![10, 20, 30, 40];

// Repetição: vec![valor; quantidade]
let zeros = vec![0; 5]; // [0, 0, 0, 0, 0]
```

### 2.3 Vec::with_capacity() - Pré-alocação

```rust {.line-numbers}
// Reserva espaço para 100 elementos (evita realocações)
let mut numeros: Vec<i32> = Vec::with_capacity(100);
println!("Capacidade: {}", numeros.capacity()); // 100
println!("Comprimento: {}", numeros.len());     // 0
```

**Comparação Java vs Rust:**

| Java | Rust |
|------|------|
| `new ArrayList<>()` | `Vec::new()` ou `vec![]` |
| `new ArrayList<>(capacidade)` | `Vec::with_capacity(capacidade)` |
| `Arrays.asList(1, 2, 3)` | `vec![1, 2, 3]` |

**[⬆ Voltar ao Índice](#-índice)**

---

## 3. Manipulação: Push, Pop e Cia

### 3.1 Adicionar Elementos

```rust {.line-numbers}
let mut tarefas = Vec::new();

// push: adiciona no final (como add() em Java)
tarefas.push(String::from("Estudar Rust"));
tarefas.push(String::from("Fazer exercícios"));

// insert: insere em posição específica
tarefas.insert(0, String::from("Tomar café")); // Insere no início
```

### 3.2 Remover Elementos

```rust {.line-numbers}
let mut numeros = vec![10, 20, 30, 40, 50];

// pop: remove e retorna o último (Option<T>)
let ultimo = numeros.pop(); // Some(50)
println!("{:?}", ultimo);   // Some(50)

// remove: remove por índice e retorna o valor
let segundo = numeros.remove(1); // Remove o 20
println!("{}", segundo);         // 20

// clear: remove todos os elementos
numeros.clear();
println!("Tamanho: {}", numeros.len()); // 0
```

### 3.3 Por que pop() retorna Option?

**Em Java:**
```java
ArrayList<Integer> lista = new ArrayList<>();
Integer valor = lista.remove(lista.size() - 1); // IndexOutOfBoundsException!
```

**Em Rust:**
```rust {.line-numbers}
let mut lista: Vec<i32> = Vec::new();
let valor = lista.pop(); // Some(valor) ou None (sem panic!)

match valor {
    Some(v) => println!("Removido: {}", v),
    None => println!("Lista já estava vazia!"),
}
```

**💡 Rust te força a pensar no caso "vazio"!**

**[⬆ Voltar ao Índice](#-índice)**

---

## 4. Acesso Seguro vs Arriscado

### 4.1 Indexação Direta: [] - Pode Dar Panic!

```rust {.line-numbers}
let cores = vec!["Vermelho", "Verde", "Azul"];

let primeira = cores[0];  // "Vermelho" ✅
// let quinta = cores[4]; // ❌ PANIC! index out of bounds
```

**Analogia:** É como pular de olhos fechados para o vagão 10 do trem... se ele não existir, você cai! 💥

### 4.2 get() - Acesso Seguro com Option

```rust {.line-numbers}
let cores = vec!["Vermelho", "Verde", "Azul"];

// get() retorna Option<&T>
match cores.get(0) {
    Some(cor) => println!("Primeira cor: {}", cor),
    None => println!("Índice inválido!"),
}

// Usando if let (mais idiomático para um único case)
if let Some(cor) = cores.get(10) {
    println!("Cor: {}", cor);
} else {
    println!("Não existe cor no índice 10"); // Esse ramo executa
}
```

**Comparação Java vs Rust:**

| Java | Rust | Resultado |
|------|------|-----------|
| `lista.get(indice)` | `vec[indice]` | Panic se inválido |
| `lista.get(indice)` com validação manual | `vec.get(indice)` | `Option<&T>` (seguro) |

**[⬆ Voltar ao Índice](#-índice)**

---

## 5. Iteração: O Coração do Ownership

Aqui está a **GRANDE DIFERENÇA** entre Java e Rust! 🎯

### 5.1 Emprestar Imutável: iter()

```rust {.line-numbers}
let numeros = vec![1, 2, 3, 4, 5];

// iter() empresta cada elemento como &T
for num in numeros.iter() {
    println!("{}", num); // num é &i32
}

// numeros ainda é válido! ✅
println!("Soma: {}", numeros.iter().sum::<i32>());
```

**Em Java (equivalente):**
```java
for (Integer num : numeros) {
    System.out.println(num); // num é Integer (cópia ou referência)
}
// numeros continua acessível
```

### 5.2 Emprestar Mutável: iter_mut()

```rust {.line-numbers}
let mut numeros = vec![1, 2, 3, 4, 5];

// iter_mut() empresta cada elemento como &mut T
for num in numeros.iter_mut() {
    *num *= 2; // Desreferencia e modifica
}

println!("{:?}", numeros); // [2, 4, 6, 8, 10]
```

**Em Java (equivalente):**
```java
for (int i = 0; i < numeros.size(); i++) {
    numeros.set(i, numeros.get(i) * 2);
}
```

### 5.3 Consumir: into_iter()

```rust {.line-numbers}
let numeros = vec![1, 2, 3, 4, 5];

// into_iter() MOVE cada elemento (consome o vector)
for num in numeros.into_iter() {
    println!("{}", num); // num é i32 (owned)
}

// ❌ numeros não existe mais! Foi consumido!
// println!("{:?}", numeros); // ERRO DE COMPILAÇÃO
```

**Analogia Divertida:**

- **iter()**: Você tira **fotos** dos vagões do trem 📸 (só olha)
- **iter_mut()**: Você **pinta** os vagões 🎨 (modifica, mas o trem continua seu)
- **into_iter()**: Você **desmonta** o trem e leva os vagões embora 🚚 (consome)

### 5.4 For Loop Direto (Atalho para into_iter)

```rust {.line-numbers}
let numeros = vec![1, 2, 3];

// Isso é o mesmo que numeros.into_iter()
for num in numeros {
    println!("{}", num);
}

// ❌ numeros foi movido!
```

**Para evitar consumir, use referência:**

```rust {.line-numbers}
let numeros = vec![1, 2, 3];

// &numeros chama automaticamente iter()
for num in &numeros {
    println!("{}", num); // num é &i32
}

// ✅ numeros ainda existe!
println!("{:?}", numeros);
```

**Tabela Resumo:**

| Método | Tipo do Elemento | Vector Depois | Uso |
|--------|------------------|---------------|-----|
| `iter()` | `&T` | Válido ✅ | Só ler |
| `iter_mut()` | `&mut T` | Válido ✅ | Modificar |
| `into_iter()` | `T` | Consumido ❌ | Transferir ownership |
| `&vec` no for | `&T` | Válido ✅ | Atalho para `iter()` |
| `vec` no for | `T` | Consumido ❌ | Atalho para `into_iter()` |

**[⬆ Voltar ao Índice](#-índice)**

---

## 6. Slices: Janelas para o Vector

**Slices** são "visões" de partes do vector, sem copiar dados.

```rust {.line-numbers}
let numeros = vec![10, 20, 30, 40, 50];

// Slice do índice 1 até 3 (exclusivo)
let slice = &numeros[1..4]; // [20, 30, 40]
println!("{:?}", slice);

// Slice do início até índice 2 (exclusivo)
let inicio = &numeros[..3]; // [10, 20, 30]

// Slice do índice 2 até o fim
let fim = &numeros[2..]; // [30, 40, 50]

// Slice completo
let tudo = &numeros[..]; // [10, 20, 30, 40, 50]
```

**Tipo:** `&[T]` (slice de T, não é `Vec<T>`)

**Em Java:**
```java
List<Integer> slice = numeros.subList(1, 4); // Cria uma view
```

**Diferença:** Em Rust, slices são **mais eficientes** (apenas ponteiro + tamanho) e **mais seguras** (borrow checker garante validade).

**[⬆ Voltar ao Índice](#-índice)**

---

## 7. Capacidade vs Comprimento

```rust {.line-numbers}
let mut vec = Vec::with_capacity(10);
vec.push(1);
vec.push(2);

println!("Comprimento: {}", vec.len());      // 2 (elementos atuais)
println!("Capacidade: {}", vec.capacity());  // 10 (espaço alocado)
```

**Analogia:** Um ônibus 🚌 com 50 assentos (capacidade) mas apenas 10 passageiros (comprimento).

**Quando a capacidade importa?**

```rust {.line-numbers}
let mut vec = Vec::new();

for i in 0..1000 {
    vec.push(i);
    // Rust pode realocar várias vezes (caro!)
}

// Melhor: pré-alocar
let mut vec = Vec::with_capacity(1000);
for i in 0..1000 {
    vec.push(i); // Sem realocações!
}
```

**Em Java:**
```java
ArrayList<Integer> lista = new ArrayList<>(1000); // Capacidade inicial
```

**[⬆ Voltar ao Índice](#-índice)**

---

## 8. Comparação Java vs Rust

| Operação | Java ArrayList | Rust Vec |
|----------|----------------|----------|
| Criar vazio | `new ArrayList<>()` | `Vec::new()` ou `vec![]` |
| Criar com valores | `Arrays.asList(1, 2)` | `vec![1, 2]` |
| Adicionar no final | `.add(valor)` | `.push(valor)` |
| Remover último | `.remove(size() - 1)` | `.pop()` retorna `Option<T>` |
| Remover por índice | `.remove(i)` | `.remove(i)` |
| Acesso direto | `[i]` ou `.get(i)` | `[i]` (panic) ou `.get(i)` (safe) |
| Iterar (só ler) | `for (T x : list)` | `for x in &vec` |
| Iterar (modificar) | `for (int i...)` + `.set()` | `for x in &mut vec` |
| Tamanho | `.size()` | `.len()` |
| Capacidade | `lista.trimToSize()` conceito | `.capacity()` |

**Diferença Crítica de Ownership:**

```rust {.line-numbers}
let vec = vec![1, 2, 3];

// ❌ ERRO: vec foi movido!
for x in vec {
    println!("{}", x);
}
// println!("{:?}", vec); // ERRO!

// ✅ CORRETO: emprestar
for x in &vec {
    println!("{}", x);
}
println!("{:?}", vec); // OK!
```

**Em Java, isso nunca acontece** (GC gerencia tudo).

**[⬆ Voltar ao Índice](#-índice)**

---

## 9. EXERCÍCIO PRÁTICO: Sistema de Gerenciamento de Notas

Vamos criar um sistema simples que gerencia notas de alunos, aplicando tudo que aprendemos!

**Requisitos:**

- CRUD completo (Create, Read, Update, Delete)
- Iteração de diferentes formas
- Uso de slices
- Tratamento de índices inválidos
- Cálculos estatísticos

**Código Completo:**

```rust {.line-numbers}
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
```

**Saída Esperada:**

```
🎓 SISTEMA DE GERENCIAMENTO DE NOTAS

📊 Notas iniciais: [7.5, 8.0, 6.5, 9.0, 5.5]
Total de notas: 5

➕ Adicionando notas...
Notas após push: [7.5, 8.0, 6.5, 9.0, 5.5, 8.5, 7.0]

🔍 LEITURA DE NOTAS:
Primeira nota (indexação direta): 7.5
Terceira nota (get): 6.5
❌ Índice 100 não existe (get retornou None)

🔄 ITERAÇÃO:

📖 Listando todas as notas (iter):
  Nota 1: 7.5
  Nota 2: 8.0
  Nota 3: 6.5
  Nota 4: 9.0
  Nota 5: 5.5
  Nota 6: 8.5
  Nota 7: 7.0

🎯 Aplicando bônus de 0.5 em todas as notas (iter_mut):
Notas após bônus: [8.0, 8.5, 7.0, 9.5, 6.0, 9.0, 7.5]

📊 Contando notas acima de 8.0:
Total de notas >= 8.0: 4

✂️ TRABALHANDO COM SLICES:
Top 3 primeiras notas: [8.0, 8.5, 7.0]
Últimas 2 notas: [9.0, 7.5]
Média das 4 primeiras notas: 8.25

📈 ESTATÍSTICAS:
Média geral: 7.93
Maior nota: 9.5
Menor nota: 6.0

✏️ UPDATE:
Nota anterior no índice 2: 7.0
Nova nota no índice 2: 9.5
Notas após update: [8.0, 8.5, 9.5, 9.5, 6.0, 9.0, 7.5]

🗑️ DELETE:
Removida última nota: 7.5
Removida nota no índice 1: 8.5
Notas finais: [8.0, 9.5, 9.5, 6.0, 9.0]
Total de notas restantes: 5

🚨 DEMONSTRAÇÃO DE OWNERSHIP:
Usando into_iter() (consome o vector):
  Nota: 10.0
  Nota: 9.0
  Nota: 8.0
✅ temp_notas foi consumido e não existe mais!

✨ Sistema finalizado!
```

**[⬆ Voltar ao Índice](#-índice)**

---

## 🎯 Pontos-Chave do Exercício

### ✅ O que praticamos:

**Criação:**
- `vec![]` com valores iniciais
- `let mut` para permitir modificações

**Manipulação:**
- `push()` para adicionar elementos
- `pop()` retorna `Option<T>` (seguro!)
- `remove(indice)` remove por posição

**Acesso:**
- `vec[i]` (direto, pode dar panic)
- `vec.get(i)` retorna `Option<&T>` (seguro)
- `vec.get_mut(i)` retorna `Option<&mut T>` para modificar

**Iteração (3 formas):**
- `iter()`: empresta imutável `&T` (só leitura)
- `iter_mut()`: empresta mutável `&mut T` (modificar)
- `into_iter()`: consome o vector `T` (transfere ownership)

**Slices:**
- `&vec[0..3]` acessa partes do vector
- Não copia dados, apenas referencia

**Métodos úteis:**
- `.len()`: tamanho atual
- `.iter().sum()`: soma elementos
- `.enumerate()`: iteração com índice

---

## 🆚 Resumo Final: Java vs Rust

| Aspecto | Java ArrayList | Rust Vec |
|---------|----------------|----------|
| **Mutabilidade** | Sempre mutável | `let mut` explícito |
| **Acesso inválido** | Exception em runtime | `get()` retorna `Option` ou panic com `[]` |
| **Iteração** | Não afeta ownership | `iter()` vs `into_iter()` (crítico!) |
| **Modificar ao iterar** | `.set(i, valor)` | `iter_mut()` com `*elemento` |
| **Segurança** | Runtime checks | Compile-time checks |

---

## 🚀 Próximos Passos

Amanhã (Dia 9) vamos explorar **HashMaps**, o equivalente ao `HashMap` do Java, mas com toda a elegância do sistema de ownership do Rust!

**Prepare-se para:**
- `HashMap<K, V>` e suas operações
- Entry API (muito mais elegante que Java!)
- Iteração sobre chaves e valores
- Tratamento de chaves ausentes com `Option`

---

**🎉 Parabéns, Bianeck!** Você dominou os Vectors! Agora você entende como Rust garante segurança de memória mesmo com estruturas dinâmicas, e por que o `ArrayList` do Java parece simples em comparação. O poder está no controle! 💪

**[⬆ Voltar ao Índice](#-índice)**