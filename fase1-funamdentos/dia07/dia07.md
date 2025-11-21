# 🎯 **DIA 7 - Enums em Rust: Tipos Algébricos Revolucionários**

Bianeck, prepare-se porque **hoje você vai descobrir que enums em Rust são uma BOMBA ATÔMICA** comparados aos humildes enums de Java! 🚀

Se enums em Java são como um cardápio de restaurante (lista fixa de opções), enums em Rust são como **blocos LEGO que carregam caixas dentro** - cada variante pode transportar dados completamente diferentes!

---

## 📑 **Índice de Navegação**

**[Teoria Completa](#teoria-completa)**
├─ [O que são Enums em Rust?](#o-que-são-enums-em-rust)
├─ [Enums Básicos](#enums-básicos)
├─ [Enums com Dados Associados](#enums-com-dados-associados)
├─ [Option<T>: Adeus null!](#optiont-adeus-null)
├─ [Result<T, E>: Erros como Valores](#resultt-e-erros-como-valores)
├─ [Pattern Matching com match](#pattern-matching-com-match)
└─ [if let e while let](#if-let-e-while-let)

**[Comparações Java vs Rust](#comparações-java-vs-rust)**

**[Exercício Prático](#exercício-prático)**

**[Conclusão e Próximos Passos](#conclusão-e-próximos-passos)**

---

## **Teoria Completa**

### **O que são Enums em Rust?**

**Enums em Rust são Tipos Algébricos (Algebraic Data Types)** - isso significa que cada variante pode:

✅ Existir sozinha (como Java)
✅ Carregar dados de tipos diferentes
✅ Ser usada em pattern matching exaustivo
✅ Substituir null e exceptions

**Analogia Divertida:**

Imagine um **sistema de delivery**:

**Java Enum (limitado):**
```java
enum StatusPedido {
    PENDENTE,
    ENVIADO,
    ENTREGUE,
    CANCELADO
}
```

Você só sabe o STATUS, mas não tem DETALHES!

**Rust Enum (poderoso):**
```rust {.line-numbers}
enum StatusPedido {
    Pendente,
    Enviado { rastreio: String, transportadora: String },
    Entregue { data: String, assinatura: String },
    Cancelado { motivo: String, reembolso: f64 }
}
```

Agora cada status **carrega seus próprios dados específicos**! 🎁

---

### **Enums Básicos**

A forma mais simples - similar ao Java, mas já com superpoderes:

```rust {.line-numbers}
// Enum simples - como Java
enum DiaDaSemana {
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
    Domingo,
}

// Usando
fn eh_fim_de_semana(dia: DiaDaSemana) -> bool {
    match dia {
        DiaDaSemana::Sabado | DiaDaSemana::Domingo => true,
        _ => false, // _ é o "default" do match
    }
}

fn main() {
    let hoje = DiaDaSemana::Segunda;
    println!("É fim de semana? {}", eh_fim_de_semana(hoje));
}
```

**Nota:** Já percebeu o `::` para acessar variantes? Rust usa namespacing!

---

### **Enums com Dados Associados**

Aqui começa a **MAGIA REAL**! Cada variante pode ter:

**Dados de tipos diferentes:**

```rust {.line-numbers}
enum Mensagem {
    Sair,                           // Sem dados
    Escrever(String),               // Uma String
    Mover { x: i32, y: i32 },      // Struct anônima
    MudarCor(u8, u8, u8),          // Tupla RGB
}

// Criando instâncias
fn exemplos() {
    let msg1 = Mensagem::Sair;
    let msg2 = Mensagem::Escrever(String::from("Olá Rust!"));
    let msg3 = Mensagem::Mover { x: 10, y: 20 };
    let msg4 = Mensagem::MudarCor(255, 0, 0); // Vermelho
}
```

**Processando com match:**

```rust {.line-numbers}
fn processar_mensagem(msg: Mensagem) {
    match msg {
        Mensagem::Sair => {
            println!("Encerrando aplicação...");
        }
        Mensagem::Escrever(texto) => {
            println!("Texto recebido: {}", texto);
        }
        Mensagem::Mover { x, y } => {
            println!("Movendo para coordenadas ({}, {})", x, y);
        }
        Mensagem::MudarCor(r, g, b) => {
            println!("Mudando cor para RGB({}, {}, {})", r, g, b);
        }
    }
}
```

**Comparação com Java:**

Em Java, você teria que fazer isso com **classes abstratas + herança**:

```java
// Java precisa de toda essa cerimônia!
abstract class Mensagem {}
class Sair extends Mensagem {}
class Escrever extends Mensagem {
    String texto;
    Escrever(String texto) { this.texto = texto; }
}
class Mover extends Mensagem {
    int x, y;
    Mover(int x, int y) { this.x = x; this.y = y; }
}
// ... e instanceof para verificar tipos 🤮
```

---

### **Option<T>: Adeus null!**

**A REVOLUÇÃO:** Rust **NÃO TEM `null`**! 🎉

Em vez disso, usa o enum `Option<T>`:

```rust {.line-numbers}
enum Option<T> {
    Some(T),    // Tem valor
    None,       // Não tem valor
}
```

**Por que isso é GENIAL?**

Em Java, qualquer referência pode ser `null` (erro em tempo de execução):

```java
// Java - NullPointerException esperando pra acontecer!
String nome = buscarNome(id); // pode ser null
int tamanho = nome.length();  // 💥 BOOM se null!
```

Em Rust, você **DEVE** tratar explicitamente a ausência de valor:

```rust {.line-numbers}
// Rust força você a lidar com None
fn buscar_nome(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Bianeck"))
    } else {
        None
    }
}

fn main() {
    let resultado = buscar_nome(2);
    
    // Match exaustivo - OBRIGADO a tratar None
    match resultado {
        Some(nome) => println!("Nome encontrado: {}", nome),
        None => println!("Nome não encontrado!"),
    }
}
```

**Métodos úteis do Option:**

```rust {.line-numbers}
fn exemplos_option() {
    let valor: Option<i32> = Some(42);
    let vazio: Option<i32> = None;
    
    // unwrap - PERIGO! panic se None (use só quando TEM CERTEZA)
    let num = valor.unwrap(); // 42
    
    // unwrap_or - valor padrão se None (SEGURO)
    let num = vazio.unwrap_or(0); // 0
    
    // is_some / is_none - verificação booleana
    if valor.is_some() {
        println!("Tem valor!");
    }
    
    // map - transforma o valor se Some
    let dobro = valor.map(|n| n * 2); // Some(84)
    
    // and_then - encadeia operações que retornam Option
    let resultado = valor
        .and_then(|n| if n > 0 { Some(n * 2) } else { None });
}
```

---

### **Result<T, E>: Erros como Valores**

Rust **NÃO USA EXCEPTIONS**! Erros são valores explícitos com `Result<T, E>`:

```rust {.line-numbers}
enum Result<T, E> {
    Ok(T),      // Sucesso com valor T
    Err(E),     // Erro com valor E
}
```

**Exemplo prático:**

```rust {.line-numbers}
use std::fs::File;
use std::io::{self, Read};

// Função que pode falhar
fn ler_usuario_do_arquivo(caminho: &str) -> Result<String, io::Error> {
    let mut arquivo = File::open(caminho)?; // ? propaga erro
    let mut conteudo = String::new();
    arquivo.read_to_string(&mut conteudo)?;
    Ok(conteudo) // Sucesso!
}

fn main() {
    match ler_usuario_do_arquivo("usuario.txt") {
        Ok(conteudo) => {
            println!("Arquivo lido com sucesso!");
            println!("Conteúdo: {}", conteudo);
        }
        Err(erro) => {
            println!("Erro ao ler arquivo: {}", erro);
        }
    }
}
```

**Operador `?` - Propagação de Erros:**

O `?` é **açúcar sintático** para:

```rust {.line-numbers}
// COM ?
let arquivo = File::open("arquivo.txt")?;

// SEM ? (equivalente)
let arquivo = match File::open("arquivo.txt") {
    Ok(f) => f,
    Err(e) => return Err(e),
};
```

**Comparação Java vs Rust:**

```java
// Java - exceptions escondidas (precisa conhecer documentação)
public String lerArquivo(String caminho) throws IOException {
    // pode explodir em runtime se não tratar
}

// Rust - Result EXPLÍCITO na assinatura
fn ler_arquivo(caminho: &str) -> Result<String, io::Error> {
    // compilador FORÇA você a tratar
}
```

---

### **Pattern Matching com match**

O `match` em Rust é **MUITO MAIS PODEROSO** que `switch` do Java:

**Características:**

✅ **Exaustivo** - precisa cobrir TODOS os casos (compilador verifica!)
✅ **Retorna valores** - é uma expressão
✅ **Destructuring** - extrai dados das variantes
✅ **Guards** - condições extras com `if`

```rust {.line-numbers}
enum TipoPagamento {
    Dinheiro(f64),
    Cartao { numero: String, cvv: u16 },
    Pix { chave: String },
}

fn processar_pagamento(pagamento: TipoPagamento, valor: f64) -> String {
    match pagamento {
        // Destructuring simples
        TipoPagamento::Dinheiro(quantia) if quantia >= valor => {
            let troco = quantia - valor;
            format!("Pago em dinheiro. Troco: R$ {:.2}", troco)
        }
        TipoPagamento::Dinheiro(_) => {
            String::from("Dinheiro insuficiente!")
        }
        
        // Destructuring de struct
        TipoPagamento::Cartao { numero, cvv } => {
            let ultimos_digitos = &numero[numero.len() - 4..];
            format!("Cartão final {}, CVV: ***", ultimos_digitos)
        }
        
        // Binding de valores
        TipoPagamento::Pix { chave } => {
            format!("PIX para chave: {}", chave)
        }
    }
}
```

**Match guards (condições extras):**

```rust {.line-numbers}
fn classificar_nota(nota: Option<u8>) -> &'static str {
    match nota {
        Some(n) if n >= 90 => "Excelente!",
        Some(n) if n >= 70 => "Bom!",
        Some(n) if n >= 50 => "Regular",
        Some(_) => "Insuficiente",
        None => "Nota não informada",
    }
}
```

---

### **if let e while let**

Quando você só quer tratar **UM caso específico**, use `if let`:

**if let - Pattern matching simplificado:**

```rust {.line-numbers}
fn main() {
    let algum_valor: Option<i32> = Some(7);
    
    // COM match (verboso)
    match algum_valor {
        Some(v) => println!("Valor: {}", v),
        None => (),
    }
    
    // COM if let (conciso)
    if let Some(v) = algum_valor {
        println!("Valor: {}", v);
    }
    
    // if let com else
    if let Some(v) = algum_valor {
        println!("Tem valor: {}", v);
    } else {
        println!("Não tem valor!");
    }
}
```

**while let - Loop condicional:**

```rust {.line-numbers}
fn main() {
    let mut pilha = vec![1, 2, 3, 4, 5];
    
    // Remove elementos enquanto houver
    while let Some(topo) = pilha.pop() {
        println!("Removido: {}", topo);
    }
    // Quando pilha vazia, pop() retorna None e loop para
}
```

---

[⬆️ Voltar ao Índice](#índice-de-navegação)

---

## **Comparações Java vs Rust**

### **1. Enums: Constantes vs Tipos Algébricos**

**Java Enum:**

```java
// Java - apenas constantes com métodos
public enum Status {
    PENDENTE(1),
    APROVADO(2),
    REJEITADO(3);
    
    private int codigo;
    
    Status(int codigo) {
        this.codigo = codigo;
    }
    
    public int getCodigo() {
        return codigo;
    }
}

// Todos os enums têm A MESMA estrutura (código)
```

**Rust Enum:**

```rust {.line-numbers}
// Rust - cada variante pode ter estrutura DIFERENTE
enum Status {
    Pendente,
    Aprovado { data: String, aprovador: String },
    Rejeitado { motivo: String, timestamp: u64 },
}

// Cada variante carrega dados ESPECÍFICOS!
```

---

### **2. Null vs Option<T>**

**Java:**

```java
// null pode estar QUALQUER LUGAR (terror!)
public String buscarUsuario(int id) {
    // pode retornar null
    return null;
}

// Precisa lembrar de verificar (ou NullPointerException!)
String usuario = buscarUsuario(1);
if (usuario != null) {
    System.out.println(usuario.toUpperCase());
}
```

**Rust:**

```rust {.line-numbers}
// Option EXPLÍCITO na assinatura
fn buscar_usuario(id: u32) -> Option<String> {
    None // ausência explícita
}

// Compilador FORÇA verificação
match buscar_usuario(1) {
    Some(usuario) => println!("{}", usuario.to_uppercase()),
    None => println!("Usuário não encontrado"),
}
```

---

### **3. Exceptions vs Result<T, E>**

**Java:**

```java
// Exception ESCONDIDA (só descobre na documentação)
public String lerArquivo(String path) throws IOException {
    // pode explodir!
    return Files.readString(Paths.get(path));
}

// Precisa try/catch ou propagar
try {
    String conteudo = lerArquivo("arquivo.txt");
} catch (IOException e) {
    System.err.println("Erro: " + e.getMessage());
}
```

**Rust:**

```rust {.line-numbers}
// Result EXPLÍCITO - erro faz parte da assinatura
fn ler_arquivo(caminho: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(caminho)
}

// Tratamento obrigatório
match ler_arquivo("arquivo.txt") {
    Ok(conteudo) => println!("Lido: {}", conteudo),
    Err(erro) => eprintln!("Erro: {}", erro),
}

// Ou propagação explícita com ?
fn processar() -> Result<(), std::io::Error> {
    let conteudo = ler_arquivo("arquivo.txt")?;
    Ok(())
}
```

---

### **4. Switch vs Match**

**Java Switch:**

```java
// Java - apenas valores primitivos/String
int dia = 1;
String nome = switch (dia) {
    case 1 -> "Segunda";
    case 2 -> "Terça";
    case 3 -> "Quarta";
    default -> "Outro";
};

// NÃO pode fazer destructuring ou pattern matching complexo
```

**Rust Match:**

```rust {.line-numbers}
// Rust - destructuring + guards + exaustivo
enum Dia {
    Util(String),
    FimDeSemana { atividade: String },
}

let dia = Dia::Util(String::from("Segunda"));

let mensagem = match dia {
    Dia::Util(nome) if nome == "Segunda" => {
        "Começou a semana! 😭"
    }
    Dia::Util(nome) => {
        &format!("Dia útil: {}", nome)
    }
    Dia::FimDeSemana { atividade } => {
        &format!("Curtir: {}", atividade)
    }
};
```

---

### **Tabela Resumo: Java vs Rust**

| Aspecto | Java | Rust |
|---------|------|------|
| **Enum** | Constantes fixas | Tipos algébricos com dados |
| **Null** | Qualquer referência pode ser null | `Option<T>` explícito |
| **Erros** | Exceptions (podem ser esquecidas) | `Result<T, E>` (obrigatório tratar) |
| **Pattern Match** | Switch limitado | Match exaustivo + destructuring |
| **Type Safety** | Verificação em runtime | Verificação em compile-time |

---

[⬆️ Voltar ao Índice](#índice-de-navegação)

---

## **Exercício Prático**

### **🎮 Sistema de Operações Bancárias**

Vamos criar um sistema que processa diferentes tipos de transações bancárias usando enums poderosos!

**Requisitos:**

**1. Enum `Transacao` com variantes:**
   - `Deposito` com valor
   - `Saque` com valor
   - `Transferencia` com valor e conta destino
   - `Pagamento` com descrição e valor

**2. Struct `ContaBancaria` com:**
   - Nome do titular
   - Saldo
   - Método `processar_transacao` que retorna `Result<String, String>`

**3. Tratamento de erros:**
   - Saque/Transferência com saldo insuficiente
   - Valores negativos ou zero

**4. Função `exibir_extrato` que processa lista de transações**

---

### **📝 Código Inicial (Complete!)**

```rust {.line-numbers}
// Enum que representa diferentes tipos de transações
enum Transacao {
    // TODO: Implemente as 4 variantes
    // Deposito -> valor: f64
    // Saque -> valor: f64
    // Transferencia -> valor: f64, conta_destino: String
    // Pagamento -> descricao: String, valor: f64
}

// Struct que representa uma conta bancária
struct ContaBancaria {
    titular: String,
    saldo: f64,
}

impl ContaBancaria {
    // Cria nova conta
    fn nova(titular: String, saldo_inicial: f64) -> Self {
        ContaBancaria {
            titular,
            saldo: saldo_inicial,
        }
    }
    
    // Processa uma transação
    // Retorna Ok com mensagem de sucesso ou Err com mensagem de erro
    fn processar_transacao(&mut self, transacao: Transacao) -> Result<String, String> {
        // TODO: Implemente usando match
        // - Validar valores (devem ser > 0)
        // - Deposito: adicionar ao saldo
        // - Saque: verificar saldo suficiente antes de subtrair
        // - Transferencia: verificar saldo suficiente antes de subtrair
        // - Pagamento: verificar saldo suficiente antes de subtrair
        // Retornar mensagens descritivas
        
        todo!() // Remova esta linha e implemente
    }
    
    // Consulta saldo
    fn consultar_saldo(&self) -> f64 {
        self.saldo
    }
}

// Função que processa múltiplas transações e exibe extrato
fn exibir_extrato(conta: &mut ContaBancaria, transacoes: Vec<Transacao>) {
    // TODO: Implemente
    // - Iterar sobre transacoes
    // - Processar cada uma com processar_transacao
    // - Usar match para Ok/Err
    // - Exibir resultado de cada operação
    // - Ao final, mostrar saldo
}

fn main() {
    println!("=== SISTEMA BANCÁRIO ===\n");
    
    let mut conta = ContaBancaria::nova(
        String::from("Bianeck"),
        1000.0
    );
    
    println!("Titular: {}", conta.titular);
    println!("Saldo inicial: R$ {:.2}\n", conta.consultar_saldo());
    
    // Lista de transações para processar
    let transacoes = vec![
        Transacao::Deposito(500.0),
        Transacao::Saque(200.0),
        Transacao::Transferencia {
            valor: 300.0,
            conta_destino: String::from("12345-6"),
        },
        Transacao::Pagamento {
            descricao: String::from("Conta de Luz"),
            valor: 150.0,
        },
        Transacao::Saque(2000.0), // Vai falhar - saldo insuficiente
    ];
    
    exibir_extrato(&mut conta, transacoes);
}
```

---

### **🎯 Desafios Extras (Opcional)**

**1. Adicione mais variantes:**
   - `PIX { chave: String, valor: f64 }`
   - `Investimento { tipo: String, valor: f64 }`

**2. Crie enum para resultado das transações:**
   ```rust {.line-numbers}
   enum ResultadoTransacao {
       Sucesso { mensagem: String, novo_saldo: f64 },
       FalhaSaldoInsuficiente { tentativa: f64, disponivel: f64 },
       FalhaValorInvalido,
   }
   ```

**3. Implemente histórico de transações:**
   ```rust {.line-numbers}
   struct ContaBancaria {
       // ... campos existentes
       historico: Vec<Transacao>,
   }
   ```

---

### **✅ Solução Esperada**

Seu código deve:

✅ Usar enum com dados associados (struct e tupla)
✅ Pattern matching exaustivo com `match`
✅ Retornar `Result<String, String>` adequadamente
✅ Tratar erros de validação (valores <= 0)
✅ Tratar erros de negócio (saldo insuficiente)
✅ Usar `if let` ou `match` para processar Results
✅ Código limpo e bem comentado

---

### **💡 Dicas para Implementação**

**Para `processar_transacao`:**

```rust {.line-numbers}
match transacao {
    Transacao::Deposito(valor) => {
        if valor <= 0.0 {
            return Err(String::from("Valor deve ser positivo"));
        }
        self.saldo += valor;
        Ok(format!("Depósito de R$ {:.2} realizado", valor))
    }
    // ... complete o resto
}
```

**Para `exibir_extrato`:**

```rust {.line-numbers}
for (i, transacao) in transacoes.into_iter().enumerate() {
    println!("Transação {}: ", i + 1);
    
    match conta.processar_transacao(transacao) {
        Ok(mensagem) => println!("✅ {}", mensagem),
        Err(erro) => println!("❌ {}", erro),
    }
    println!();
}
```

---

[⬆️ Voltar ao Índice](#índice-de-navegação)

---

## **Conclusão e Próximos Passos**

### **🎓 O que você aprendeu hoje:**

**Conceitos Fundamentais:**
- Enums em Rust são **tipos algébricos** (ADTs), não apenas constantes
- Cada variante pode carregar dados de tipos diferentes
- `Option<T>` substitui `null` de forma segura
- `Result<T, E>` substitui exceptions com erros explícitos
- Pattern matching é **exaustivo** (compilador garante todos os casos)

**Superpoderes do Rust:**
- `match` com destructuring e guards
- `if let` / `while let` para casos específicos
- Operador `?` para propagação de erros
- Compilador força tratamento de erros em tempo de compilação

**Comparado ao Java:**
- Java: enums são constantes + métodos
- Rust: enums são sum types que carregam dados
- Java: null pode estar em qualquer lugar (runtime terror)
- Rust: Option explícito (compile-time safety)
- Java: exceptions escondidas
- Rust: Result explícito na assinatura

---

### **📚 Próximo Dia: Dia 8 - Vectors (Vec<T>)**

Amanhã você vai dominar:
- **Vec<T>** (similar ao ArrayList de Java)
- Operações: push, pop, insert, remove
- Iteração e ownership
- Slices de vectors
- Métodos poderosos: map, filter, collect

---

### **🚀 Recapitulando a Jornada**

**Dias Anteriores:**
- **Dia 1:** Setup e Hello World
- **Dia 2:** Variáveis e imutabilidade
- **Dia 3:** Controle de fluxo e match
- **Dia 4:** Funções e retornos implícitos
- **Dia 5:** Strings (String vs &str)
- **Dia 6:** Structs e métodos

**Hoje (Dia 7):** Enums revolucionários ✅

**Próximos:**
- **Dia 8:** Vectors
- **Dia 9:** HashMaps
- **Dia 10:** Error Handling avançado

---

### **💪 Desafio Motivacional**

Bianeck, depois de hoje você entende porque desenvolvedores dizem:

> **"Depois de Option e Result, você nunca mais vai querer null e exceptions!"**

Enums em Rust não são apenas sintaxe - são uma **mudança de paradigma** na forma de modelar dados e erros!

**Pratique o exercício e sinta o poder do compilador te guiando!** 🦀

---

[⬆️ Voltar ao Índice](#índice-de-navegação)

---

**Bons estudos, e nos vemos no Dia 8 com Vectors!** 🎯🚀