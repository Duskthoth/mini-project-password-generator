# Password Generator

Gerador de senhas em linha de comando, escrito em Rust, com foco em geração criptograficamente segura e regras explícitas de composição.

## Visão Geral

O projeto gera uma ou mais senhas a partir dos grupos de caracteres selecionados por parâmetro.

- Usa `OsRng` para obter entropia do sistema operacional.
- Usa `ChaCha8Rng` como gerador pseudoaleatório para montar as senhas.
- Exige que cada grupo informado apareça pelo menos uma vez na senha.
- Falha com erro quando o tamanho solicitado não comporta todos os requisitos ativos.
- Permite excluir caracteres específicos com `--exclude`.

## Documentação

As informações técnicas do projeto foram movidas para a pasta `docs`.

- `docs/implementacao-tecnica.md`: stack, arquitetura, fluxo interno, estrutura do projeto, dependências e estratégia de testes.

## Instalação

### Executar localmente

```bash
git clone https://github.com/Duskthoth/mini-project-password-generator.git
cd mini-project-password-generator
cargo run -- --lower --digits --length 16
```

### Gerar binário de release

```bash
cargo build --release
./target/release/password-generator --lower --digits --length 16
```

No Windows, o executável gerado fica em `target/release/password-generator.exe`.

## Uso

Pelo menos um dos grupos `--upper`, `--lower`, `--digits` ou `--symbols` precisa ser informado. Se nenhum for passado, o programa retorna erro.

### Opções disponíveis

| Opção | Atalho | Descrição | Padrão |
|--------|--------|-----------|--------|
| `--length <N>` | `-l` | Tamanho da senha | `12` |
| `--upper` | - | Inclui letras maiúsculas | `false` |
| `--lower` | - | Inclui letras minúsculas | `false` |
| `--digits` | - | Inclui dígitos | `false` |
| `--symbols` | - | Inclui símbolos ASCII | `false` |
| `--exclude <CHAR>` | `-e` | Exclui um caractere específico; pode ser repetido | vazio |
| `--count <N>` | `-c` | Quantidade de senhas geradas | `1` |

### Exemplos

```bash
# Gera uma senha com minúsculas e dígitos
cargo run -- --lower --digits

# Gera 5 senhas de 20 caracteres com maiúsculas, minúsculas e dígitos
cargo run -- --upper --lower --digits --length 20 --count 5

# Gera uma senha com todos os grupos e garante ao menos 1 caractere de cada
cargo run -- --upper --lower --digits --symbols --length 4

# Exclui caracteres específicos
cargo run -- --lower --digits --exclude a --exclude 0 --length 12
```

## Regras de Erro

O programa encerra com erro nos seguintes casos:

- nenhum grupo de caracteres foi selecionado;
- um grupo foi selecionado, mas ficou vazio após as exclusões;
- `length` é menor do que a quantidade de grupos selecionados.

Exemplo:

```bash
cargo run -- --lower --digits --symbols --length 2
```

Nesse caso, o comando falha porque 2 caracteres não são suficientes para satisfazer 3 requisitos ativos.

## Limitações Atuais

- não há configuração por arquivo;
- não há presets prontos de política de senha;
- não há medição de entropia;
- não há suporte a conjuntos customizados além de exclusões individuais.

## Licença

Projeto licenciado sob MIT.
