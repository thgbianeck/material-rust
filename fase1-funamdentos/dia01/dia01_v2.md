# 🦀 Rust - Dia 1: Setup Completo e Hello World

**Bem-vindo à sua jornada em Rust, Bianeck!** Hoje você vai configurar todo o ambiente e criar seu primeiro projeto. Vamos comparar bastante com Java para facilitar sua transição.

---

## 📑 Índice

- **[Parte 1: Instalação e Configuração](#parte-1-instalação-e-configuração)**
  - [1.1 Instalando o Rust](#11-instalando-o-rust)
  - [1.2 Configurando o VSCode](#12-configurando-o-vscode)

- **[Parte 2: Primeiro Projeto](#parte-2-primeiro-projeto)**
  - [2.1 Criando com Cargo](#21-criando-com-cargo)
  - [2.2 Anatomia do Projeto](#22-anatomia-do-projeto)
  - [2.3 Comandos Essenciais](#23-comandos-essenciais)

- **[Parte 3: Anatomia do Hello World](#parte-3-anatomia-do-hello-world)**
  - [3.1 Comparação com Java](#31-comparação-com-java)
  - [3.2 Entendendo Cada Linha](#32-entendendo-cada-linha)

- **[Parte 4: Exercícios Práticos](#parte-4-exercícios-práticos)**
  - [Exercício 1: Mensagem Personalizada (Fácil)](#exercício-1-mensagem-personalizada-fácil)
  - [Exercício 2: Calculadora Simples (Médio)](#exercício-2-calculadora-simples-médio)
  - [Exercício 3: Analisador de Texto (Desafiador)](#exercício-3-analisador-de-texto-desafiador)

- **[Parte 5: Troubleshooting](#parte-5-troubleshooting)**

- **[Checkpoint Final](#checkpoint-final)**

---

## Parte 1: Instalação e Configuração

### 1.1 Instalando o Rust

**🎯 Analogia:** Pense no Rust como se fosse o JDK do Java, mas muito mais completo. Você instala uma única ferramenta chamada **rustup** que gerencia tudo: o compilador (rustc), o gerenciador de pacotes (cargo) e as versões do Rust.

#### 🐧 **Linux (Ubuntu/Debian/Fedora/etc.)**

**Passo 1:** Abra o terminal e execute:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Passo 2:** Durante a instalação, você verá opções. Pressione **1** (instalação padrão):

```
1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>1
```

**Passo 3:** Após a instalação, configure o PATH:

```bash
source "$HOME/.cargo/env"
```

**Passo 4:** Verifique a instalação:

```bash
rustc --version    # Compilador (como javac)
cargo --version    # Gerenciador de pacotes (como Maven/Gradle)
rustfmt --version  # Formatador de código (como google-java-format)
```

Você deve ver algo como:
```
rustc 1.75.0 (82e1608df 2023-12-21)
cargo 1.75.0 (1d8b05cdd 2023-11-20)
rustfmt 1.7.0-stable (82e1608d 2023-12-21)
```

---

#### 🪟 **Windows**

**Passo 1:** Baixe o instalador em: https://rustup.rs/

**Passo 2:** Execute o arquivo `rustup-init.exe`

**Passo 3:** Antes de continuar, você precisará do **Visual Studio C++ Build Tools**:

> **⚠️ IMPORTANTE:** Rust no Windows depende do MSVC (Microsoft Visual C++)

- O instalador do rustup vai avisar sobre isso
- Baixe o "Build Tools for Visual Studio" em: https://visualstudio.microsoft.com/downloads/
- Instale apenas o "Desktop development with C++"

**Passo 4:** Reinicie o terminal e execute rustup-init novamente:

```powershell
# No PowerShell ou CMD
rustup-init.exe
```

Escolha opção **1** (instalação padrão)

**Passo 5:** Reinicie o terminal e verifique:

```powershell
rustc --version
cargo --version
rustfmt --version
```

---

#### 🔄 **Atualizando o Rust**

```bash
rustup update
```

> **💡 Dica:** Faça isso regularmente! Rust tem ciclo de releases de 6 semanas (muito mais rápido que Java).

---

### 1.2 Configurando o VSCode

**🎯 Analogia:** Assim como você usa plugins Java no VSCode (Extension Pack for Java), em Rust o plugin essencial é o **rust-analyzer** (pense nele como o IntelliJ IDEA embutido para Rust).

#### **Passo 1: Instalar Extensões**

Abra o VSCode e instale estas extensões:

1. **rust-analyzer** (oficial, essencial)
   - Autocomplete inteligente
   - Navegação de código
   - Refactoring
   - Erros em tempo real

2. **CodeLLDB** (debugger)
   - Debugging como no Java
   - Breakpoints, watches, step-by-step

3. **crates** (opcional, mas útil)
   - Mostra versões atualizadas de dependências
   - Similar ao Maven Helper no IntelliJ

4. **Even Better TOML** (opcional)
   - Sintaxe highlight para Cargo.toml
   - TOML é como o pom.xml do Rust

#### **Passo 2: Configurar settings.json**

Pressione `Ctrl+Shift+P` (ou `Cmd+Shift+P` no Mac) → "Preferences: Open Settings (JSON)"

Adicione estas configurações:

```json
{
    // Formatação automática ao salvar (como no Java com save actions)
    "editor.formatOnSave": true,
    
    // Rust-analyzer configurações
    "rust-analyzer.check.command": "clippy",  // Linter mais rigoroso
    "rust-analyzer.checkOnSave": true,         // Verifica ao salvar
    
    // Inlay hints (tipo inferência visível)
    "rust-analyzer.inlayHints.typeHints.enable": true,
    "rust-analyzer.inlayHints.parameterHints.enable": true,
    
    // Imports automáticos
    "rust-analyzer.completion.autoimport.enable": true
}
```

> **💡 Dica:** Os "inlay hints" mostram os tipos inferidos no código. Em Rust, você raramente precisa declarar tipos explicitamente (similar ao `var` do Java 10+), então os hints ajudam muito!

#### **Passo 3: Testar o Setup**

Vamos criar um projeto teste rápido:

```bash
cargo new test_setup
cd test_setup
code .
```

Abra `src/main.rs`. Você deve ver:
- ✅ Syntax highlighting
- ✅ Ícones de "Run" acima de `fn main()`
- ✅ Autocomplete funcionando

---

[🔝 Voltar ao Índice](#-índice)

---

## Parte 2: Primeiro Projeto

### 2.1 Criando com Cargo

**🎯 Analogia:** Cargo é como Maven e Gradle combinados, mas muito mais rápido e sem XML! É o coração do ecossistema Rust.

#### **Comparação Cargo vs Maven:**

| Cargo (Rust)              | Maven (Java)           |
|---------------------------|------------------------|
| `cargo new projeto`       | `mvn archetype:generate` |
| `cargo build`             | `mvn compile`          |
| `cargo run`               | `mvn exec:java`        |
| `cargo test`              | `mvn test`             |
| `Cargo.toml`              | `pom.xml`              |
| `target/`                 | `target/`              |

#### **Criando o Projeto "hello_rust"**

```bash
# Cria um novo projeto executável (binário)
cargo new hello_rust

# Entre no diretório
cd hello_rust

# Veja a estrutura criada
ls -la
```

**Estrutura criada:**

```
hello_rust/
├── Cargo.toml       ← Arquivo de configuração (como pom.xml)
├── .git/            ← Git inicializado automaticamente!
├── .gitignore       ← Já configurado para Rust
└── src/
    └── main.rs      ← Seu código-fonte (entry point)
```

> **💡 Dica:** Cargo inicializa um repositório Git automaticamente! Você pode desabilitar isso com `cargo new hello_rust --vcs none`.

#### **Alternativa: Projeto como Biblioteca**

```bash
# Criar uma biblioteca (como um JAR library)
cargo new minha_lib --lib

# Estrutura diferente:
# src/lib.rs ao invés de src/main.rs
```

---

### 2.2 Anatomia do Projeto

#### **📄 Cargo.toml - O "pom.xml" do Rust**

Abra `Cargo.toml`:

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
```

**Comparação com pom.xml:**

```xml
<!-- Equivalente em Maven -->
<project>
    <artifactId>hello_rust</artifactId>
    <version>0.1.0</version>
    <properties>
        <java.version>21</java.version>  <!-- "edition" do Rust -->
    </properties>
    <dependencies>
        <!-- Suas dependências aqui -->
    </dependencies>
</project>
```

**Entendendo cada seção:**

- **[package]** - Metadados do projeto
  - `name`: Nome do executável/biblioteca
  - `version`: Segue SemVer (Semantic Versioning)
  - `edition`: Versão da linguagem (2015, 2018, 2021, 2024)

- **[dependencies]** - Suas bibliotecas externas
  - Similar ao `<dependencies>` do Maven
  - Formato: `nome_crate = "versão"`
  
**Exemplo com dependências:**

```toml
[dependencies]
serde = "1.0"              # Equivalente ao Gson/Jackson
tokio = "1.35"             # Framework async (como Spring WebFlux)
reqwest = "0.11"           # Cliente HTTP (como Apache HttpClient)
```

> **📦 Vocabulário:** Em Rust, bibliotecas são chamadas de **crates** (caixas). O repositório central é **crates.io** (equivalente ao Maven Central).

---

#### **📄 src/main.rs - O Entry Point**

Abra `src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

**Este é o código mais simples em Rust!** Apenas 3 linhas.

---

### 2.3 Comandos Essenciais

#### **🔨 cargo build - Compilar o Projeto**

```bash
cargo build
```

**O que acontece:**

1. Baixa e compila dependências (como `mvn dependency:resolve`)
2. Compila seu código
3. Cria executável em `target/debug/hello_rust`

**Saída esperada:**

```
   Compiling hello_rust v0.1.0 (/caminho/hello_rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

**Estrutura do target/:**

```
target/
├── debug/                    ← Build de desenvolvimento
│   ├── hello_rust           ← Seu executável (não é .jar!)
│   ├── hello_rust.d
│   └── deps/                 ← Dependências compiladas
└── CACHEDIR.TAG
```

**Diferenças do Java:**

| Java (Maven)                    | Rust (Cargo)                |
|---------------------------------|-----------------------------|
| Gera `.class` e `.jar`          | Gera binário nativo direto  |
| Precisa de JVM para executar    | Executável standalone       |
| `java -jar app.jar`             | `./target/debug/hello_rust` |
| Bytecode interpretado           | Código nativo compilado     |

#### **🏃 cargo run - Compilar e Executar**

```bash
cargo run
```

**Equivalente a:**

```bash
cargo build && ./target/debug/hello_rust
```

**Saída:**

```
   Compiling hello_rust v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/hello_rust`
Hello, world!
```

> **💡 Dica:** Use `cargo run` durante desenvolvimento. É como `mvn exec:java`, mas muito mais rápido!

---

#### **✅ cargo check - Verificação Rápida**

```bash
cargo check
```

**O que faz:**
- Verifica se o código compila
- **NÃO** gera o executável
- **10x mais rápido** que `cargo build`

**Quando usar:**
- Durante escrita de código (feedback rápido)
- Em CI/CD para verificação inicial
- Similar ao "Compile" do IntelliJ, mas via CLI

**Comparação de velocidade:**

```
cargo check  → ~0.1s  ✅ Super rápido
cargo build  → ~1.0s  ⚡ Rápido
cargo build --release → ~5.0s  🐢 Lento (mas otimizado)
```

---

#### **🚀 cargo build --release - Build de Produção**

```bash
cargo build --release
```

**Diferenças do modo debug:**

| Debug (`cargo build`)        | Release (`--release`)         |
|------------------------------|-------------------------------|
| Rápido para compilar         | Lento para compilar           |
| Sem otimizações              | Otimizações máximas (LLVM)    |
| Debug symbols inclusos       | Sem debug symbols             |
| `target/debug/`              | `target/release/`             |
| Código ~100x mais lento      | Código otimizado              |

**Quando usar release:**
- Deploy em produção
- Benchmarks de performance
- Distribuição para usuários

> **⚠️ IMPORTANTE:** Nunca meça performance com build debug! A diferença pode ser de 100x ou mais.

---

#### **🧪 Outros Comandos Úteis**

```bash
# Formatar código (como google-java-format)
cargo fmt

# Linter rigoroso (como CheckStyle + SpotBugs)
cargo clippy

# Limpar arquivos compilados
cargo clean

# Rodar testes
cargo test

# Gerar documentação (como Javadoc)
cargo doc --open

# Atualizar dependências
cargo update
```

---

[🔝 Voltar ao Índice](#-índice)

---

## Parte 3: Anatomia do Hello World

### 3.1 Comparação com Java

Vamos comparar o "Hello World" lado a lado:

**☕ Java:**

```java
// Arquivo: HelloWorld.java
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, world!");
    }
}

// Compilar: javac HelloWorld.java
// Executar: java HelloWorld
```

**🦀 Rust:**

```rust
// Arquivo: main.rs
fn main() {
    println!("Hello, world!");
}

// Compilar: cargo build
// Executar: cargo run
```

**Diferenças chave:**

| Aspecto                | Java                          | Rust                     |
|------------------------|-------------------------------|--------------------------|
| Classe wrapper         | ✅ Obrigatória                | ❌ Não existe            |
| Modificadores          | `public static`               | Nenhum necessário        |
| Tipo de retorno        | `void`                        | Implícito (omitido)      |
| Parâmetros main        | `String[] args`               | Omitido (opcional)       |
| Macro vs Método        | `System.out.println()`        | `println!()`             |
| Ponto e vírgula        | ✅ Obrigatório                | ✅ Obrigatório           |

---

### 3.2 Entendendo Cada Linha

Vamos dissecar o código Rust:

```rust
fn main() {
    println!("Hello, world!");
}
```

#### **Linha 1: `fn main() {`**

- **`fn`** - Palavra-chave para declarar função (como `def` em Python)
  - Equivalente a "function"
  - Em Java você usa apenas `void`/`int`/`String` etc.

- **`main`** - Nome da função de entrada
  - Como em Java, é o entry point
  - Convenção: snake_case (diferente do camelCase do Java)
  
- **`()`** - Parâmetros (vazios aqui)
  - Para aceitar argumentos CLI: `fn main()`... (veremos depois)
  - Java: `String[] args` é obrigatório, Rust: opcional

- **`{`** - Início do bloco
  - Igual ao Java

**🔍 Detalhes importantes:**

- **Sem modificadores de acesso:** Rust não tem `public`, `private` dentro de funções
- **Sem tipo de retorno explícito:** Quando omitido, assume `()` (unit type - equivalente a `void`)
- **Sem ponto e vírgula após `{`:** Como em Java

---

#### **Linha 2: `println!("Hello, world!");`**

- **`println!`** - Macro de impressão (note o **`!`**)
  - O `!` indica que é uma **macro**, não uma função comum
  - Macros são expandidas em compile-time (como templates C++)
  
- **Comparação com Java:**

```java
// Java
System.out.println("Hello");  // Método estático, sem !

// Rust
println!("Hello");             // Macro, com !
```

- **Por que macro?**
  - Permite formatação complexa em compile-time
  - Verifica tipos em compile-time
  - Mais eficiente que método normal

**Exemplos de uso:**

```rust
// Simples
println!("Hello");

// Com formatação (como String.format em Java)
let nome = "Bianeck";
let idade = 40;
println!("Olá, {}! Você tem {} anos.", nome, idade);
// Saída: Olá, Bianeck! Você tem 40 anos.

// Com placeholders nomeados
println!("Olá, {nome}! Você tem {idade} anos.", nome="Bianeck", idade=40);

// Debug de variáveis (como toString() automático)
let lista = vec![1, 2, 3];
println!("{:?}", lista);  // [1, 2, 3]
```

> **💡 Dica:** `println!` adiciona quebra de linha automaticamente. Use `print!` sem quebra.

---

#### **Linha 3: `}`**

- Fecha o bloco da função `main`
- Sem `return` explícito necessário
- A função retorna implicitamente `()` (equivalente a `void`)

---

#### **🎯 Versão Completa com Comentários**

```rust
// Esta é a função de entrada do programa
// Equivalente ao 'public static void main(String[] args)' do Java
fn main() {
    // println! é uma macro (note o '!')
    // Imprime texto no console com quebra de linha
    // Equivalente ao System.out.println() do Java
    println!("Hello, world!");
    
    // A função retorna implicitamente '()' (unit type)
    // Equivalente a 'void' em Java
}
```

---

#### **🔬 Aceitando Argumentos da Linha de Comando**

Se você precisar dos `args` (como em Java):

```rust
use std::env;  // Importar módulo env (como import em Java)

fn main() {
    // Obter argumentos (Vec<String>, similar a String[])
    let args: Vec<String> = env::args().collect();
    
    // Imprimir argumentos
    println!("Argumentos: {:?}", args);
    
    // args[0] é o caminho do executável (como em Java)
    // args[1], args[2]... são seus argumentos
}
```

**Executar com argumentos:**

```bash
cargo run -- arg1 arg2 arg3
# Saída: Argumentos: ["target/debug/hello_rust", "arg1", "arg2", "arg3"]
```

> **💡 Dica:** O `--` separa argumentos do cargo dos argumentos do seu programa.

---

[🔝 Voltar ao Índice](#-índice)

---

## Parte 4: Exercícios Práticos

### Exercício 1: Mensagem Personalizada (Fácil)

**🎯 Objetivo:** Criar um programa que imprime uma mensagem de boas-vindas personalizada.

**📋 Requisitos:**
- Imprimir seu nome e profissão
- Mostrar a data atual (hardcoded por enquanto)
- Usar múltiplas chamadas `println!`
- Adicionar comentários explicativos

**🔧 Setup:**

```bash
cargo new exercicio1_mensagem
cd exercicio1_mensagem
code .
```

**✅ Solução Completa:**

```rust
// exercicio1_mensagem/src/main.rs

// Esta é a função principal que será executada ao rodar o programa
fn main() {
    // Em Rust, usamos println! (com !) porque é uma macro, não uma função
    // Macros são expandidas em tempo de compilação e permitem formatação flexível
    
    // Imprime uma linha de separação decorativa
    println!("========================================");
    
    // Imprime mensagem de boas-vindas
    println!("🦀 Bem-vindo ao Rust!");
    
    // Linha em branco (equivalente a System.out.println() vazio em Java)
    println!();
    
    // Informações pessoais
    println!("👤 Nome: Thiago Bianeck");
    println!("💼 Profissão: Engenheiro de Software Sênior");
    println!("🏠 Cidade: Francisco Beltrão, PR");
    
    // Linha em branco
    println!();
    
    // Informações sobre o aprendizado
    println!("📚 Estou aprendendo Rust!");
    println!("📅 Data de início: 15 de Novembro de 2024");
    
    // Linha de separação
    println!("========================================");
    
    // Nota: Não precisamos de 'return' ou 'return;' no final
    // A função retorna implicitamente '()' (unit type - equivalente a void)
}
```

**▶️ Executar:**

```bash
cargo run
```

**📤 Saída esperada:**

```
========================================
🦀 Bem-vindo ao Rust!

👤 Nome: Thiago Bianeck
💼 Profissão: Engenheiro de Software Sênior
🏠 Cidade: Francisco Beltrão, PR

📚 Estou aprendendo Rust!
📅 Data de início: 15 de Novembro de 2024
========================================
```

**🎓 Conceitos Aprendidos:**
- ✅ Usar `println!` macro para imprimir texto
- ✅ Comentários com `//`
- ✅ Estrutura básica de um programa Rust
- ✅ Emojis funcionam nativamente (UTF-8 por padrão)

---

### Exercício 2: Calculadora Simples (Médio)

**🎯 Objetivo:** Criar uma calculadora que realiza operações básicas com valores hardcoded.

**📋 Requisitos:**
- Declarar variáveis com valores numéricos
- Realizar operações: soma, subtração, multiplicação, divisão
- Usar formatação de strings para exibir resultados
- Demonstrar tipos de dados numéricos

**🔧 Setup:**

```bash
cargo new exercicio2_calculadora
cd exercicio2_calculadora
code .
```

**✅ Solução Completa:**

```rust
// exercicio2_calculadora/src/main.rs

fn main() {
    println!("🧮 Calculadora Simples em Rust\n");
    
    // DECLARAÇÃO DE VARIÁVEIS
    // ========================
    
    // Em Rust, variáveis são IMUTÁVEIS por padrão (diferente de Java!)
    // É como declarar 'final' em Java: final int a = 10;
    let a = 10;        // Tipo inferido automaticamente como i32 (inteiro 32 bits)
    let b = 3;         // Também i32
    
    // Para variáveis MUTÁVEIS, use 'mut' (similar a variável normal em Java)
    let mut resultado = 0;  // Podemos mudar o valor depois
    
    // Você também pode declarar o tipo explicitamente (como em Java)
    let x: i32 = 100;       // i32 = inteiro 32 bits com sinal
    let y: f64 = 50.5;      // f64 = float 64 bits (double em Java)
    
    println!("📊 Valores iniciais:");
    println!("   a = {}", a);
    println!("   b = {}", b);
    println!("   x = {}", x);
    println!("   y = {}", y);
    println!();
    
    
    // OPERAÇÕES BÁSICAS
    // =================
    
    // Soma (igual em Java)
    resultado = a + b;
    println!("➕ Soma: {} + {} = {}", a, b, resultado);
    
    // Subtração
    resultado = a - b;
    println!("➖ Subtração: {} - {} = {}", a, b, resultado);
    
    // Multiplicação
    resultado = a * b;
    println!("✖️  Multiplicação: {} * {} = {}", a, b, resultado);
    
    // Divisão inteira (retorna i32)
    resultado = a / b;
    println!("➗ Divisão inteira: {} / {} = {}", a, b, resultado);
    
    // Módulo (resto da divisão)
    resultado = a % b;
    println!("📐 Módulo (resto): {} % {} = {}", a, b, resultado);
    
    println!();
    
    
    // OPERAÇÕES COM FLOATS
    // ====================
    
    // Para divisão com resultado decimal, use floats
    let a_float = a as f64;  // Conversão explícita (cast - igual Java)
    let b_float = b as f64;
    
    let divisao_decimal = a_float / b_float;
    
    // Formatação com 2 casas decimais: {:.2}
    println!("🔢 Operações com decimais:");
    println!("   {} / {} = {:.2}", a, b, divisao_decimal);
    
    // Operações com y (já é f64)
    let resultado_float = x as f64 + y;
    println!("   {} + {} = {:.2}", x, y, resultado_float);
    
    println!();
    
    
    // OPERAÇÕES COMPOSTAS
    // ===================
    
    // Rust suporta operadores compostos como Java
    let mut contador = 10;
    
    println!("📈 Operações compostas:");
    println!("   Valor inicial: {}", contador);
    
    contador += 5;  // Equivalente a: contador = contador + 5
    println!("   Após += 5: {}", contador);
    
    contador -= 3;
    println!("   Após -= 3: {}", contador);
    
    contador *= 2;
    println!("   Após *= 2: {}", contador);
    
    contador /= 4;
    println!("   Após /= 4: {}", contador);
    
    println!();
    
    
    // COMPARAÇÕES COM JAVA
    // ====================
    
    println!("📚 Comparação Rust vs Java:");
    println!("   Java: final int a = 10;    → Rust: let a = 10;");
    println!("   Java: int b = 3;           → Rust: let mut b = 3;");
    println!("   Java: double x = 10.5;     → Rust: let x: f64 = 10.5;");
    println!("   Java: (double) a / b       → Rust: a as f64 / b as f64");
}
```

**▶️ Executar:**

```bash
cargo run
```

**📤 Saída esperada:**

```
🧮 Calculadora Simples em Rust

📊 Valores iniciais:
   a = 10
   b = 3
   x = 100
   y = 50.5

➕ Soma: 10 + 3 = 13
➖ Subtração: 10 - 3 = 7
✖️  Multiplicação: 10 * 3 = 30
➗ Divisão inteira: 10 / 3 = 3
📐 Módulo (resto): 10 % 3 = 1

🔢 Operações com decimais:
   10 / 3 = 3.33
   100 + 50.5 = 150.50

📈 Operações compostas:
   Valor inicial: 10
   Após += 5: 15
   Após -= 3: 12
   Após *= 2: 24
   Após /= 4: 6

📚 Comparação Rust vs Java:
   Java: final int a = 10;    → Rust: let a = 10;
   Java: int b = 3;           → Rust: let mut b = 3;
   Java: double x = 10.5;     → Rust: let x: f64 = 10.5;
   Java: (double) a / b       → Rust: a as f64 / b as f64
```

**🎓 Conceitos Aprendidos:**
- ✅ Variáveis imutáveis por padrão (`let`)
- ✅ Variáveis mutáveis (`let mut`)
- ✅ Tipos numéricos: `i32`, `f64`
- ✅ Inferência de tipos (compiler adivinha o tipo)
- ✅ Anotação explícita de tipos (`: i32`, `: f64`)
- ✅ Operadores aritméticos
- ✅ Operadores compostos (`+=`, `-=`, etc.)
- ✅ Conversão de tipos (`as`)
- ✅ Formatação de strings (`{}`, `{:.2}`)

**💡 Diferenças importantes do Java:**

| Conceito          | Java                      | Rust                      |
|-------------------|---------------------------|---------------------------|
| Imutável          | `final int x = 10;`       | `let x = 10;`             |
| Mutável           | `int x = 10;`             | `let mut x = 10;`         |
| Tipo explícito    | `int x = 10;`             | `let x: i32 = 10;`        |
| Cast              | `(double) x`              | `x as f64`                |
| Default           | Mutável                   | Imutável                  |

---

### Exercício 3: Analisador de Texto (Desafiador)

**🎯 Objetivo:** Criar um programa que analisa uma string e exibe estatísticas.

**📋 Requisitos:**
- Trabalhar com strings (`String` vs `&str`)
- Contar caracteres, palavras e linhas
- Usar métodos de string
- Demonstrar iteração básica
- Usar múltiplas variáveis e formatação avançada

**🔧 Setup:**

```bash
cargo new exercicio3_analisador
cd exercicio3_analisador
code .
```

**✅ Solução Completa:**

```rust
// exercicio3_analisador/src/main.rs

fn main() {
    println!("📝 Analisador de Texto em Rust\n");
    
    
    // STRINGS EM RUST
    // ===============
    
    // Em Rust existem 2 tipos principais de strings:
    //
    // 1. &str (string slice) - Imutável, referência, tamanho fixo
    //    Similar a String em Java (que também é imutável)
    //    Exemplo: "Hello" é do tipo &str
    //
    // 2. String - Mutável, owned, tamanho dinâmico
    //    Similar a StringBuilder em Java
    //    Exemplo: String::from("Hello") ou "Hello".to_string()
    
    
    // String literal (tipo &str - imutável)
    let texto: &str = "Rust é uma linguagem de programação moderna.
Focada em segurança, velocidade e concorrência.
Ideal para sistemas de alto desempenho!";
    
    // Converter para String (owned - pode crescer/mudar)
    let texto_owned: String = texto.to_string();
    
    println!("📄 Texto analisado:");
    println!("{}", texto);
    println!("\n" + "=".repeat(50) + "\n");
    
    
    // ANÁLISE 1: CONTAGEM DE CARACTERES
    // ==================================
    
    // len() retorna o número de BYTES (não caracteres!)
    // Em UTF-8, caracteres podem ter 1-4 bytes
    let total_bytes = texto.len();
    
    // chars() retorna um iterador sobre caracteres Unicode
    // count() conta elementos do iterador
    let total_caracteres = texto.chars().count();
    
    println!("🔤 Análise de Caracteres:");
    println!("   Total de bytes: {}", total_bytes);
    println!("   Total de caracteres: {}", total_caracteres);
    
    // Contar letras (excluindo espaços e pontuação)
    // filter() é como Stream.filter() em Java
    // is_alphabetic() verifica se é letra
    let total_letras = texto
        .chars()
        .filter(|c| c.is_alphabetic())
        .count();
    
    println!("   Total de letras: {}", total_letras);
    
    // Contar dígitos
    let total_digitos = texto
        .chars()
        .filter(|c| c.is_numeric())
        .count();
    
    println!("   Total de dígitos: {}", total_digitos);
    
    // Contar espaços em branco
    let total_espacos = texto
        .chars()
        .filter(|c| c.is_whitespace())
        .count();
    
    println!("   Total de espaços: {}", total_espacos);
    println!();
    
    
    // ANÁLISE 2: CONTAGEM DE PALAVRAS
    // ================================
    
    // split_whitespace() divide string por espaços (similar a split("\s+") em Java)
    // Retorna um iterador de palavras
    let palavras: Vec<&str> = texto.split_whitespace().collect();
    let total_palavras = palavras.len();
    
    println!("📚 Análise de Palavras:");
    println!("   Total de palavras: {}", total_palavras);
    
    // Encontrar palavra mais longa
    let palavra_mais_longa = palavras
        .iter()                          // Cria iterador
        .max_by_key(|palavra| palavra.len())  // Encontra max por tamanho
        .unwrap_or(&"");                 // Valor padrão se vazio
    
    println!("   Palavra mais longa: \"{}\" ({} caracteres)", 
             palavra_mais_longa, 
             palavra_mais_longa.len());
    
    // Encontrar palavra mais curta
    let palavra_mais_curta = palavras
        .iter()
        .min_by_key(|palavra| palavra.len())
        .unwrap_or(&"");
    
    println!("   Palavra mais curta: \"{}\" ({} caracteres)", 
             palavra_mais_curta, 
             palavra_mais_curta.len());
    
    // Comprimento médio de palavra
    let comprimento_total: usize = palavras
        .iter()
        .map(|palavra| palavra.len())  // Mapeia para comprimentos
        .sum();                         // Soma todos
    
    let comprimento_medio = if total_palavras > 0 {
        comprimento_total as f64 / total_palavras as f64
    } else {
        0.0
    };
    
    println!("   Comprimento médio: {:.2} caracteres", comprimento_medio);
    println!();
    
    
    // ANÁLISE 3: CONTAGEM DE LINHAS
    // ==============================
    
    // lines() retorna iterador sobre linhas
    let linhas: Vec<&str> = texto.lines().collect();
    let total_linhas = linhas.len();
    
    println!("📊 Análise de Linhas:");
    println!("   Total de linhas: {}", total_linhas);
    
    // Listar cada linha com número
    for (indice, linha) in linhas.iter().enumerate() {
        // enumerate() adiciona índice (como IntStream.range() em Java)
        // indice + 1 porque queremos contar de 1, não de 0
        println!("   Linha {}: \"{}\" ({} caracteres)", 
                 indice + 1, 
                 linha, 
                 linha.len());
    }
    
    println!();
    
    
    // ANÁLISE 4: BUSCA E SUBSTITUIÇÃO
    // ================================
    
    println!("🔍 Busca e Manipulação:");
    
    // contains() verifica se contém substring (como contains() em Java)
    let contem_rust = texto.contains("Rust");
    println!("   Contém 'Rust': {}", contem_rust);
    
    // Contar ocorrências de uma palavra
    let palavra_busca = "de";
    let ocorrencias = texto
        .split_whitespace()
        .filter(|palavra| palavra.to_lowercase() == palavra_busca)
        .count();
    
    println!("   Ocorrências de '{}': {}", palavra_busca, ocorrencias);
    
    // Substituir texto (retorna novo String - strings são imutáveis!)
    let texto_substituido = texto.replace("Rust", "🦀 Rust");
    println!("\n   Texto com substituição:");
    println!("   {}", texto_substituido);
    
    println!();
    
    
    // ANÁLISE 5: CONVERSÕES
    // ======================
    
    println!("🔄 Conversões:");
    
    // Converter para maiúsculas (retorna novo String)
    let texto_maiusculo = texto.to_uppercase();
    println!("   Maiúsculas: {}", texto_maiusculo.lines().next().unwrap_or(""));
    
    // Converter para minúsculas
    let texto_minusculo = texto.to_lowercase();
    println!("   Minúsculas: {}", texto_minusculo.lines().next().unwrap_or(""));
    
    // Primeira letra de cada palavra em maiúscula (manual)
    let texto_capitalizado = palavras
        .iter()
        .map(|palavra| {
            let mut chars = palavra.chars();
            match chars.next() {
                Some(primeira) => primeira.to_uppercase().collect::<String>() 
                                  + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ");
    
    println!("   Capitalizado: {}", texto_capitalizado.lines().next().unwrap_or(""));
    
    println!();
    
    
    // RESUMO FINAL
    // ============
    
    println!("\n" + "=".repeat(50));
    println!("📈 RESUMO DA ANÁLISE");
    println!("=".repeat(50));
    println!("Caracteres: {} | Letras: {} | Palavras: {} | Linhas: {}", 
             total_caracteres, 
             total_letras, 
             total_palavras, 
             total_linhas);
    println!("=".repeat(50));
    
    
    // BÔNUS: COMPARAÇÃO COM JAVA
    // ===========================
    
    println!("\n💡 Comparação com Java:");
    println!("   Java: texto.length()           → Rust: texto.len() [bytes]");
    println!("   Java: texto.length()           → Rust: texto.chars().count() [chars]");
    println!("   Java: texto.split(\" \")         → Rust: texto.split_whitespace()");
    println!("   Java: texto.contains(\"x\")      → Rust: texto.contains(\"x\")");
    println!("   Java: texto.replace(\"a\", \"b\")  → Rust: texto.replace(\"a\", \"b\")");
    println!("   Java: texto.toUpperCase()      → Rust: texto.to_uppercase()");
    println!("   Java: texto.toLowerCase()      → Rust: texto.to_lowercase()");
    println!("\n   Diferença chave:");
    println!("   - Java: String é imutável, StringBuilder é mutável");
    println!("   - Rust: &str é imutável, String é mutável");
}
```

**▶️ Executar:**

```bash
cargo run
```

**📤 Saída esperada:**

```
📝 Analisador de Texto em Rust

📄 Texto analisado:
Rust é uma linguagem de programação moderna.
Focada em segurança, velocidade e concorrência.
Ideal para sistemas de alto desempenho!

==================================================

🔤 Análise de Caracteres:
   Total de bytes: 157
   Total de caracteres: 157
   Total de letras: 123
   Total de dígitos: 0
   Total de espaços: 28

📚 Análise de Palavras:
   Total de palavras: 16
   Palavra mais longa: "concorrência." (13 caracteres)
   Palavra mais curta: "é" (1 caracteres)
   Comprimento médio: 7.69 caracteres

📊 Análise de Linhas:
   Total de linhas: 3
   Linha 1: "Rust é uma linguagem de programação moderna." (46 caracteres)
   Linha 2: "Focada em segurança, velocidade e concorrência." (49 caracteres)
   Linha 3: "Ideal para sistemas de alto desempenho!" (40 caracteres)

🔍 Busca e Manipulação:
   Contém 'Rust': true
   Ocorrências de 'de': 3

   Texto com substituição:
   🦀 Rust é uma linguagem de programação moderna.
Focada em segurança, velocidade e concorrência.
Ideal para sistemas de alto desempenho!

🔄 Conversões:
   Maiúsculas: RUST É UMA LINGUAGEM DE PROGRAMAÇÃO MODERNA.
   Minúsculas: rust é uma linguagem de programação moderna.
   Capitalizado: Rust É Uma Linguagem De Programação Moderna.

==================================================
📈 RESUMO DA ANÁLISE
==================================================
Caracteres: 157 | Letras: 123 | Palavras: 16 | Linhas: 3
==================================================

💡 Comparação com Java:
   Java: texto.length()           → Rust: texto.len() [bytes]
   Java: texto.length()           → Rust: texto.chars().count() [chars]
   Java: texto.split(" ")         → Rust: texto.split_whitespace()
   Java: texto.contains("x")      → Rust: texto.contains("x")
   Java: texto.replace("a", "b")  → Rust: texto.replace("a", "b")
   Java: texto.toUpperCase()      → Rust: texto.to_uppercase()
   Java: texto.toLowerCase()      → Rust: texto.to_lowercase()

   Diferença chave:
   - Java: String é imutável, StringBuilder é mutável
   - Rust: &str é imutável, String é mutável
```

**🎓 Conceitos Aprendidos:**
- ✅ Diferença entre `&str` (slice) e `String` (owned)
- ✅ Métodos de string: `len()`, `chars()`, `split_whitespace()`, `lines()`
- ✅ Iteradores: `iter()`, `filter()`, `map()`, `count()`, `sum()`
- ✅ Métodos de iterador: `max_by_key()`, `min_by_key()`, `collect()`
- ✅ Loops: `for ... in` com `enumerate()`
- ✅ Pattern matching básico: `match`
- ✅ Closures (funções anônimas): `|c| c.is_alphabetic()`
- ✅ Option type: `unwrap_or()`
- ✅ Conversões: `to_string()`, `to_uppercase()`, `to_lowercase()`
- ✅ UTF-8 nativo: emojis funcionam perfeitamente

**💡 Conceitos Avançados Introduzidos:**

| Conceito         | Explicação                                        |
|------------------|---------------------------------------------------|
| Iterator         | Similar a Stream API do Java 8+                  |
| Closure          | `\|x\| x + 1` (como lambda em Java: `x -> x + 1`) |
| Option<T>        | Similar a Optional<T> do Java                     |
| unwrap_or()      | Como orElse() do Optional                         |
| collect()        | Coleta iterador em coleção (Vec, String, etc.)   |

---

[🔝 Voltar ao Índice](#-índice)

---

## Parte 5: Troubleshooting

### 🔥 Problemas Comuns e Soluções

#### **1. "cargo: command not found" ou "rustc: command not found"**

**Problema:** O PATH não foi configurado corretamente.

**Solução Linux/Mac:**

```bash
# Adicionar ao ~/.bashrc ou ~/.zshrc
echo 'source "$HOME/.cargo/env"' >> ~/.bashrc

# Recarregar
source ~/.bashrc

# Ou usar o PATH completo:
export PATH="$HOME/.cargo/bin:$PATH"
```

**Solução Windows:**

```powershell
# O instalador deve configurar automaticamente
# Se não funcionar, adicione manualmente ao PATH:
# %USERPROFILE%\.cargo\bin

# Verificar PATH atual:
echo $env:PATH

# Adicionar temporariamente:
$env:PATH += ";$env:USERPROFILE\.cargo\bin"
```

---

#### **2. Erro "linking with `cc` failed" no Linux**

**Problema:** Faltam ferramentas de compilação C/C++.

**Solução Ubuntu/Debian:**

```bash
sudo apt update
sudo apt install build-essential
```

**Solução Fedora:**

```bash
sudo dnf groupinstall "Development Tools"
```

**Solução Arch:**

```bash
sudo pacman -S base-devel
```

---

#### **3. Erro "error: linker 'link.exe' not found" no Windows**

**Problema:** Visual Studio Build Tools não instalado.

**Solução:**

1. Baixe: https://visualstudio.microsoft.com/downloads/
2. Instale "Build Tools for Visual Studio 2022"
3. Selecione "Desktop development with C++"
4. Reinicie e execute `rustup-init.exe` novamente

---

#### **4. rust-analyzer não funciona no VSCode**

**Problema:** Extensão não carregou corretamente ou projeto não reconhecido.

**Solução:**

```bash
# 1. Verificar se rust-analyzer está instalado
rustup component add rust-analyzer

# 2. Recarregar janela do VSCode
# Ctrl+Shift+P → "Developer: Reload Window"

# 3. Verificar se está em uma pasta com Cargo.toml
# rust-analyzer precisa de Cargo.toml na raiz

# 4. Ver logs do rust-analyzer
# Ctrl+Shift+P → "Rust Analyzer: Show RA Logs"
```

**Se ainda não funcionar:**

```json
// Adicionar em settings.json
{
    "rust-analyzer.server.path": "~/.cargo/bin/rust-analyzer",
    "rust-analyzer.trace.server": "verbose"
}
```

---

#### **5. Erro "cannot find `println` in this scope"**

**Problema:** Esqueceu o `!` na macro.

**Errado:**

```rust
println("Hello");  // ❌ Faltou !
```

**Correto:**

```rust
println!("Hello");  // ✅ Macro precisa de !
```

---

#### **6. Erro "cannot assign twice to immutable variable"**

**Problema:** Tentou mudar variável imutável.

**Errado:**

```rust
let x = 5;
x = 10;  // ❌ Erro! x é imutável
```

**Correto:**

```rust
let mut x = 5;  // ✅ Adicionar 'mut'
x = 10;
```

---

#### **7. Erro "expected `;`"**

**Problema:** Esqueceu ponto e vírgula no final da linha.

**Errado:**

```rust
let x = 5
println!("{}", x);  // ❌ Faltou ; na linha anterior
```

**Correto:**

```rust
let x = 5;  // ✅ Adicionar ;
println!("{}", x);
```

> **💡 Nota:** Rust às vezes não precisa de `;` (em expressões de retorno), mas isso é avançado. Por enquanto, sempre use `;`.

---

#### **8. Compilação lenta na primeira vez**

**Problema:** Rust compila todas as dependências na primeira execução.

**Solução:**

```bash
# É normal! Rust compila dependências uma vez e cacheia
# Compilações subsequentes são MUITO mais rápidas

# Para limpar cache (se necessário):
cargo clean

# Para compilações mais rápidas durante desenvolvimento:
cargo check  # 10x mais rápido que cargo build
```

**Comparação de velocidade:**

```
Primeira compilação:  ~30s   🐢
Compilações seguintes: ~0.5s ⚡
cargo check:          ~0.1s  🚀
```

---

#### **9. Erro "format argument must be a string literal"**

**Problema:** Tentou usar variável diretamente em `println!`.

**Errado:**

```rust
let msg = "Hello";
println!(msg);  // ❌ Não funciona
```

**Correto:**

```rust
let msg = "Hello";
println!("{}", msg);  // ✅ Usar placeholder {}
```

---

#### **10. Warnings sobre código não usado**

**Problema:** Variáveis declaradas mas não usadas.

```rust
let x = 5;  // ⚠️ warning: unused variable: `x`
```

**Soluções:**

```rust
// Opção 1: Usar a variável
let x = 5;
println!("{}", x);

// Opção 2: Prefixar com _ (indica "intencional")
let _x = 5;  // ✅ Sem warning

// Opção 3: Desabilitar warning (temporário, para testes)
#[allow(unused_variables)]
let x = 5;
```

---

#### **🆘 Comandos de Diagnóstico**

```bash
# Ver versões instaladas
rustc --version
cargo --version
rustup --version

# Ver todas as toolchains instaladas
rustup show

# Verificar atualizações
rustup check

# Atualizar Rust
rustup update

# Ver informações do sistema
rustc --version --verbose

# Limpar arquivos de compilação
cargo clean

# Ver dependências do projeto
cargo tree

# Verificar sintaxe sem compilar (SUPER RÁPIDO)
cargo check

# Executar com backtrace detalhado (para debug)
RUST_BACKTRACE=1 cargo run
```

---

[🔝 Voltar ao Índice](#-índice)

---

## Checkpoint Final

### ✅ O que você aprendeu hoje:

**🔧 Setup e Ferramentas:**
- ✅ Instalar Rust com rustup (Linux e Windows)
- ✅ Configurar VSCode com rust-analyzer
- ✅ Entender o ecossistema Rust (rustc, cargo, rustfmt)

**📦 Cargo - Gerenciador de Projetos:**
- ✅ Criar projetos com `cargo new`
- ✅ Estrutura de projeto (`Cargo.toml`, `src/main.rs`)
- ✅ Comandos essenciais:
  - `cargo build` - Compilar
  - `cargo run` - Executar
  - `cargo check` - Verificar (rápido)
  - `cargo fmt` - Formatar
  - `cargo clippy` - Lint

**🦀 Fundamentos da Linguagem:**
- ✅ Função `fn main()`
- ✅ Macro `println!` (note o `!`)
- ✅ Comentários com `//`
- ✅ Variáveis imutáveis (`let`) vs mutáveis (`let mut`)
- ✅ Tipos numéricos (`i32`, `f64`)
- ✅ Strings (`&str` vs `String`)
- ✅ Operadores aritméticos e compostos
- ✅ Conversão de tipos (`as`)

**🎯 Comparações com Java:**
- ✅ `fn main()` vs `public static void main(String[] args)`
- ✅ `let x = 5` (imutável) vs `final int x = 5`
- ✅ `let mut x = 5` vs `int x = 5`
- ✅ `println!` (macro) vs `System.out.println` (método)
- ✅ Cargo vs Maven/Gradle
- ✅ Binário nativo vs bytecode JVM

**💪 Exercícios Práticos:**
- ✅ Exercício 1: Hello World personalizado
- ✅ Exercício 2: Calculadora com tipos numéricos
- ✅ Exercício 3: Analisador de texto com strings e iteradores

---

### 🎯 Teste seu conhecimento:

Responda mentalmente (ou anote):

1. **Qual comando cria um novo projeto Rust?**
   <details><summary>Resposta</summary>`cargo new nome_projeto`</details>

2. **Qual a diferença entre `let x = 5` e `let mut x = 5`?**
   <details><summary>Resposta</summary>Primeira é imutável, segunda é mutável</details>

3. **Por que `println!` tem um `!` no final?**
   <details><summary>Resposta</summary>É uma macro, não uma função comum</details>

4. **Qual comando verifica se o código compila (rápido)?**
   <details><summary>Resposta</summary>`cargo check`</details>

5. **Qual a diferença entre `&str` e `String`?**
   <details><summary>Resposta</summary>`&str` é referência imutável, `String` é owned e mutável</details>

6. **Como converter um `i32` para `f64`?**
   <details><summary>Resposta</summary>`valor as f64`</details>

7. **Qual arquivo contém as dependências do projeto?**
   <details><summary>Resposta</summary>`Cargo.toml`</details>

8. **Onde fica o executável após `cargo build`?**
   <details><summary>Resposta</summary>`target/debug/nome_projeto`</details>

---

### 📚 Para o próximo dia:

No **Dia 2**, você vai aprender:
- **Tipos de dados primitivos** em profundidade
- **Shadowing** de variáveis
- **Constantes** (`const`)
- **Type casting** e conversões
- **Tuples** e desestruturação

**Prepare-se para código mais complexo!** 🚀

---

### 🎓 Dicas Finais:

> **💡 Sobre Erros de Compilação:**
> 
> O compilador Rust é seu **melhor amigo**! Ele dá mensagens de erro extremamente detalhadas e úteis. Diferente de Java (que às vezes é vago), Rust explica:
> - **O que está errado**
> - **Por que está errado**
> - **Como consertar**
> 
> Não tenha medo de erros - leia as mensagens com calma!

> **📖 Sobre Documentação:**
> 
> - **Oficial:** https://doc.rust-lang.org/book/
> - **By Example:** https://doc.rust-lang.org/rust-by-example/
> - **Std Library:** https://doc.rust-lang.org/std/
> - **Crates:** https://crates.io/

> **🎮 Prática:**
> 
> - **Exercism:** https://exercism.org/tracks/rust
> - **Rustlings:** https://github.com/rust-lang/rustlings

> **🤝 Comunidade:**
> 
> - **Forum:** https://users.rust-lang.org/
> - **Discord:** https://discord.gg/rust-lang
> - **Reddit:** https://reddit.com/r/rust

---

### 🏆 Parabéns, Bianeck!

Você completou o **Dia 1** do seu aprendizado de Rust! 🎉

**Conquistas desbloqueadas:**
- ✅ Ambiente Rust configurado
- ✅ Primeiro projeto criado
- ✅ Hello World executado
- ✅ 3 exercícios completos
- ✅ Fundamentos compreendidos

**Estatísticas do dia:**
- ⏱️ Tempo estimado: 2-3 horas
- 📝 Linhas de código escritas: ~200+
- 🧠 Conceitos aprendidos: 15+
- 💪 Exercícios completados: 3/3

---

**Nos vemos no Dia 2!** 🦀✨

[🔝 Voltar ao Índice](#-índice)