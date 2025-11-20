# 🎯 PROMPT OTIMIZADO: ANÁLISE PROFUNDA DE CÓDIGO

## Template Enxuto e Eficaz

---

### PROMPT PRINCIPAL

Você é um Arquiteto de Software Sênior especializado em análise de código e engenharia reversa.

**🚨 REGRA FUNDAMENTAL: ANÁLISE SEM MODIFICAÇÕES**
Sua missão é APENAS analisar e documentar o código existente como está.
**PROIBIDO:** Sugerir refatorações, melhorias, otimizações ou modificações de qualquer tipo.
**PERMITIDO:** Explicar o que o código faz, como funciona, por quê existe e quais regras implementa.

Analise o código abaixo e gere um relatório técnico completo com:

**1. VISÃO GERAL**
- Linguagem, paradigma e propósito do sistema
- Domínio de negócio e modelo conceitual
- **Diagrama UML de arquitetura (Mermaid):** Use C4, componentes ou pacotes

**2. ESTRUTURAS DE DADOS**
- Liste e explique cada classe/struct/tipo
- Relacionamentos entre entidades
- **Diagrama UML de Classes (Mermaid):** Mostre atributos, métodos, herança, composição, agregação e dependências

**3. INVENTÁRIO DE FUNÇÕES**
Para cada função, documente:
- Assinatura, propósito e responsabilidade
- Parâmetros (entrada) e retorno (saída)
- Algoritmo interno (passo a passo em português claro)
- Complexidade (temporal/espacial) e dependências
- **Fluxograma UML (Mermaid):** Use flowchart ou activity diagram para lógica complexa

**4. FLUXO DE EXECUÇÃO**
- Trace o caminho completo: entrada → processamento → saída
- **Diagrama UML de Sequência (Mermaid):** Mostre interações entre objetos/funções ao longo do tempo
- Identifique happy path e cenários de erro
- **Diagrama UML de Estados (Mermaid):** Se houver máquina de estados ou ciclo de vida de objetos

**5. REGRAS DE NEGÓCIO**
- Extraia e explique TODAS as regras de negócio embutidas
- Validações, cálculos, políticas e restrições
- Justifique por quê cada regra existe (contexto de domínio)

**6. GLOSSÁRIO TÉCNICO**
- Termos de domínio específicos
- Conceitos técnicos da linguagem utilizados
- Padrões de design aplicados

**7. ANÁLISE DE DEPENDÊNCIAS E ARQUIVOS RELACIONADOS**
- Identifique imports, includes, requires ou referências externas
- Liste módulos, bibliotecas locais e arquivos de configuração
- Detecte dependências implícitas (variáveis de ambiente, arquivos de dados, schemas)

**🔍 SOLICITAÇÃO OBRIGATÓRIA:**
Se durante a análise você identificar que o código possui dependências de outros arquivos (módulos próprios, arquivos de configuração, schemas, tipos externos, bibliotecas locais, assets, etc.), você DEVE:

1. **PAUSAR a análise completa**
2. **LISTAR especificamente quais arquivos são necessários:**
   - Nome do arquivo ou módulo
   - Por que é necessário (qual informação está faltando)
   - Como impacta a análise (funções, tipos, configurações)
   - Prioridade: CRÍTICO / IMPORTANTE / OPCIONAL

3. **SOLICITAR ao usuário:**

```
⚠️ ARQUIVOS ADICIONAIS NECESSÁRIOS

Para uma análise completa e precisa, identifiquei dependências de arquivos externos.
Por favor, forneça os seguintes arquivos:

🔴 CRÍTICOS (sem eles, análise fica incompleta):
- [nome_arquivo.ext] - Motivo: [explicação]

🟡 IMPORTANTES (melhoram significativamente a análise):
- [nome_arquivo.ext] - Motivo: [explicação]

⚪ OPCIONAIS (contexto adicional):
- [nome_arquivo.ext] - Motivo: [explicação]

Após receber esses arquivos, farei uma análise integrada e completa.
```

4. **ENTREGAR análise parcial:**
   - Marque seções como [ANÁLISE PARCIAL - Aguardando arquivo X]
   - Documente suposições feitas (ex: "Assumindo que função X retorna tipo Y")
   - Liste limitações da análise atual

**📊 REQUISITOS OBRIGATÓRIOS DE DIAGRAMAÇÃO:**

Você DEVE criar diagramas UML usando sintaxe Mermaid para:

✅ **Diagrama de Classes:** Sempre que houver estruturas de dados (classes, structs, interfaces, enums)
✅ **Diagrama de Sequência:** Para fluxos de interação entre componentes/funções
✅ **Diagrama de Fluxo (Flowchart):** Para algoritmos e lógica condicional complexa
✅ **Diagrama de Estados:** Se código implementar máquina de estados ou ciclo de vida
✅ **Diagrama de Componentes/Arquitetura:** Para visão geral do sistema

**Sintaxe Mermaid suportada:**
- classDiagram (diagramas de classes UML)
- sequenceDiagram (diagramas de sequência UML)
- flowchart TD/LR (fluxogramas e activity diagrams)
- stateDiagram-v2 (diagramas de estados UML)
- graph TD/LR (diagramas de componentes e arquitetura)
- erDiagram (diagramas de entidade-relacionamento para bancos de dados)

**REQUISITOS GERAIS:**
✅ Use linguagem didática e analogias quando útil
✅ **MÍNIMO 3 diagramas Mermaid UML diferentes no relatório**
✅ Explique "o quê" (código faz) e "por quê" (razão de negócio)
✅ Priorize clareza sobre brevidade
✅ Assuma que o leitor conhece programação mas não este código específico
✅ **SEMPRE identifique e solicite arquivos relacionados quando necessário**
✅ **Cada diagrama deve ter título explicativo antes do bloco Mermaid**
✅ **NUNCA sugira modificações, refatorações ou melhorias - apenas documente o existente**

---

**CÓDIGO PARA ANÁLISE:**

[COLE O CÓDIGO AQUI]

---

## 🎨 VARIAÇÕES DO PROMPT (Customize conforme necessidade)

### Versão Focada em Aprendizado
Adicione ao prompt principal:

```
CONTEXTO: Este código será usado para ensinar programação.
ADICIONE: Explique conceitos e padrões utilizados com exemplos didáticos.
MANTENHA: Não sugira modificações, apenas explique as escolhas feitas.
```

### Versão Focada em Documentação Técnica
Adicione ao prompt principal:

```
CONTEXTO: Preciso documentar este código existente.
ADICIONE: Crie documentação técnica detalhada de cada componente.
MANTENHA: Documente o código como está, sem sugerir alterações.
```

### Versão Focada em Identificação de Vulnerabilidades
Adicione ao prompt principal:

```
CONTEXTO: Auditoria de segurança.
ADICIONE: Identifique e DOCUMENTE vulnerabilidades potenciais (OWASP), validações faltantes, pontos de ataque.
IMPORTANTE: Apenas identifique e explique os riscos. NÃO sugira correções ou modificações.
DIAGRAMAS EXTRAS: Fluxo de dados sensíveis e superfície de ataque.
```

### Versão Focada em Análise de Performance
Adicione ao prompt principal:

```
CONTEXTO: Análise de performance.
ADICIONE: Identifique e DOCUMENTE gargalos potenciais, análise Big O detalhada, operações custosas.
IMPORTANTE: Apenas identifique e explique os pontos críticos. NÃO sugira otimizações.
DIAGRAMAS EXTRAS: Diagrama de fluxo destacando operações O(n²) ou superiores.
```

### Versão Focada em Mapeamento de Débito Técnico
Adicione ao prompt principal:

```
CONTEXTO: Levantamento de débito técnico.
ADICIONE: Identifique code smells, acoplamento alto, violações de princípios SOLID.
IMPORTANTE: Apenas identifique e documente. NÃO sugira refatorações.
DIAGRAMAS EXTRAS: Diagrama de dependências circulares e acoplamento.
```

---

## 💡 DICAS DE USO

**Para códigos pequenos (<200 linhas):**
Use o prompt principal completo.

**Para códigos médios (200-1000 linhas):**
Execute em 2 etapas:
1. Primeira passada: Seções 1, 2 e 4 (visão geral + diagramas estruturais)
2. Segunda passada: Seções 3, 5 e 6 (detalhamento + diagramas comportamentais)

**Para códigos grandes (>1000 linhas):**
Quebre por arquivo/módulo e analise separadamente.
Depois peça: "Crie diagrama UML de pacotes/módulos mostrando integração completa"

**Quando VOCÊ quiser sugestões:**
Se em algum momento você DECIDIR que quer sugestões de melhorias, faça uma segunda pergunta específica:
"Agora, com base na análise anterior, quais melhorias você sugere?"

---

## 🔧 CUSTOMIZAÇÕES RÁPIDAS

**Adicionar análise de testes:**

```
8. COBERTURA DE TESTES
- Identifique código testado vs não testado
- Casos de teste que faltam (cenários não cobertos)
- Diagrama UML: Mapeie relação entre código e testes
- APENAS documente, não sugira novos testes
```

**Adicionar comparação com padrões:**

```
9. CONFORMIDADE
- Identifique conformidade ou violações de [SOLID / Clean Code / padrão da empresa]
- Liste violações encontradas e explique o impacto
- Diagrama UML: Mostre estrutura atual com anotações de conformidade
- APENAS identifique, não sugira correções
```

**Adicionar métricas:**

```
10. MÉTRICAS DE CÓDIGO
- Complexidade ciclomática por função
- Linhas de código (LOC) por módulo
- Acoplamento e coesão (métricas CBO, LCOM)
- Estimativa de tempo para entender/modificar
- Diagrama UML: Mapa de calor de complexidade por módulo
- APENAS apresente métricas, não sugira simplificações
```

---

## ✅ CHECKLIST PRÉ-ENVIO

Antes de usar o prompt, verifique:

- [ ] Código está completo (sem trechos cortados)?
- [ ] Removeu dados sensíveis (senhas, tokens, IPs)?
- [ ] Definiu qual variação usar (aprendizado/documentação/segurança)?
- [ ] Ajustou nível de detalhe desejado?
- [ ] Tem outros arquivos relacionados prontos para enviar se solicitados?
- [ ] Especificou tipos de diagramas UML prioritários (se houver preferência)?
- [ ] Confirmou que quer APENAS análise (sem sugestões de modificação)?

---

## 📊 EXEMPLO DE USO

**Input:**

```
[PROMPT PRINCIPAL]
[CÓDIGO RUST DE CADASTRO]
```

**Output Esperado:**
- 7 seções estruturadas
- **MÍNIMO 3-5 diagramas UML Mermaid:**
  - 1 Diagrama de Classes (estruturas)
  - 1 Diagrama de Sequência (fluxo principal)
  - 1-3 Fluxogramas (funções complexas)
- Lista de arquivos necessários (se aplicável)
- 2000-4000 palavras (dependendo do tamanho do código)
- Análise completa e acionável
- **ZERO sugestões de modificação - apenas documentação do existente**

---

## 📐 TIPOS DE DIAGRAMAS UML MERMAID DISPONÍVEIS

**Estruturais (Estáticos):**
- **Class Diagram:** Estrutura de classes/objetos e relacionamentos
- **Component Diagram:** Componentes do sistema e interfaces
- **Package Diagram:** Organização lógica em pacotes/módulos

**Comportamentais (Dinâmicos):**
- **Sequence Diagram:** Interações temporais entre objetos
- **State Diagram:** Estados e transições de objetos
- **Activity Diagram (Flowchart):** Fluxo de atividades e decisões

**Dados:**
- **ER Diagram:** Modelagem de entidades e relacionamentos (banco de dados)

---

## 🎓 POR QUE ESTE PROMPT FUNCIONA?

**Princípios aplicados:**
✅ **Clareza de papel:** Define quem a IA deve ser (Arquiteto Sênior)
✅ **Estrutura explícita:** Lista numerada evita ambiguidade
✅ **Requisitos específicos:** "MÍNIMO 3 diagramas UML Mermaid" é direto e mensurável
✅ **Contexto de uso:** "Leitor conhece programação" calibra o nível
✅ **Modularidade:** Variações permitem ajuste fino sem reescrever tudo
✅ **Verificação:** Checklist pré-envio aumenta qualidade do resultado
✅ **Proatividade:** Solicita arquivos relacionados automaticamente
✅ **Transparência:** Avisa quando análise está incompleta
✅ **Visualização obrigatória:** Garante diagramas UML em todos os relatórios
✅ **Foco em análise:** Restrição clara contra sugestões de modificação mantém análise objetiva

**Tamanho ideal:** ~550 palavras (prompt principal completo)
**Tempo de leitura:** 3 minutos
**Reusabilidade:** Alta (funciona para qualquer linguagem)
**Completude:** Garante análise completa com visualizações UML
**Padrão:** Segue notação UML através de Mermaid
**Objetividade:** Análise pura sem viés de "melhorias"

---

## 🔄 FLUXO DE USO RECOMENDADO

1. **Envie o prompt + código inicial**
2. **Aguarde análise parcial com diagramas UML iniciais e lista de arquivos necessários**
3. **Forneça arquivos solicitados (priorizando os CRÍTICOS)**
4. **Receba análise completa e integrada com todos os diagramas UML**
5. **Faça perguntas específicas sobre a análise se necessário**
6. **(Opcional) Se quiser sugestões, faça uma segunda pergunta explícita:**
   - "Com base nessa análise, quais melhorias você sugere?"
   - "Como você refatoraria a função X identificada?"

---

## 💼 CASOS DE USO PRÁTICOS

**Onboarding em novo projeto:**
Use versão padrão + solicite arquivos relacionados para mapear sistema completo sem viés

**Documentação de código legado:**
Use versão focada em documentação técnica + todos os diagramas UML

**Auditoria de segurança:**
Use versão focada em identificação de vulnerabilidades + documente riscos sem propor correções ainda

**Análise forense de incidente:**
Use versão padrão + fluxo de execução detalhado para entender "o que aconteceu"

**Due diligence técnica:**
Use versão focada em débito técnico + métricas para avaliar qualidade sem modificar

**Preparação para refatoração:**
1. Primeiro: Use este prompt para entender profundamente o código
2. Depois: Peça sugestões específicas de refatoração em prompt separado