# Robustez do kernel

Usuários reais encontram casos extremos que os testes de referência (docs/oracle.md) nunca veem: faces que coincidem
por acaso, tangências, emendas onde outro corpo corta, paredes mais finas que uma casca. As
execuções de robustez criam projetos aleatórios, mas realistas, e verificam cada etapa.

## Como funciona

```
solidcraft-cli fuzz --from 0 --count 1000 --jobs 8 --timeout 90 --out runs/ --report report.md
solidcraft-cli fuzz-one <seed> --minimise      # one design, cut down to the steps that matter
```

São executadas `--count` sementes a partir de `--from`. Um intervalo que excede `u64::MAX` é
rejeitado antes de criar jobs ou diretórios de saída. A semente máxima
(`18446744073709551615`) é válida com `--count 1`; `--count 0` não executa nenhuma semente.

- **Projetos** (`crates/engine/src/fuzz.rs`): um gerador com semente começa com uma caixa, um perfil de
  esboço extrudado fechado por um arco (às vezes com entalhe) ou uma seção de revolução (voltas parciais ou
  completas, às vezes tocando o eixo). Depois adiciona até oito etapas: uniões e cortes de
  caixas, cilindros e esferas, arredondamentos e chanfros em arestas aleatórias, cascas, furos (simples,
  cegos, perfurados) e padrões retangulares e circulares da última operação. Posicionamentos se ajustam
  a uma grade de 2.5 mm e frequentemente às próprias faces do corpo, de modo que faces coincidentes e tangências aparecem
  o tempo todo, como em peças reais.
- **Verificações após cada etapa:** o comando passa; cada corpo é um sólido válido (cada casca
  fechada e com orientação consistente, todas as faces geram malha, volume positivo e finito:
  `Body::validity`); o volume muda no sentido correto (um corte nunca adiciona, uma união nunca remove).
- **Identidade booleana** (`crates/kernel/src/fuzz.rs`): para duas primitivas aleatórias na mesma
  grade (caixas, cilindros, esferas, toros), união, interseção e corte produzem sólidos válidos
  e V(A∪B) + V(A∩B) = V(A) + V(B), V(A−B) = V(A) − V(A∩B), com tolerância de 0.2 %.
- Cada semente é executada em seu próprio processo com um limite de tempo, para registrar também travamentos
  e falhas abruptas. Uma etapa que o kernel recusa como **ainda não suportada** (um arredondamento entre duas faces curvas,
  uma casca mais espessa que uma parede) é desfeita e contada separadamente, e o projeto continua: são
  limitações conhecidas, não falhas. Operações booleanas que falham contam como falhas mesmo quando sua
  mensagem diz "não suportado".
- `cargo test` executa um conjunto fixo de sementes que deve continuar sem falhas
  (`crates/engine/src/tests/robustness.rs`), junto com os casos minimizados já corrigidos.

## Referências fixas por semente

`tests/robustness/goldens.json` registra o que cada semente de 0 a 95 cria (no máximo oito etapas):
suas etapas, como terminou (ok, ou o tipo e o início do erro que a recusou) e, para o
projeto restante, o volume e a área totais dos corpos (com tolerância relativa de 1e-6) e suas
contagens de faces, arestas e cascas (exatas). `tests::robustness_goldens` falha em qualquer mudança, melhor ou
pior, para que uma alteração no kernel mostre cada semente afetada. Pull requests que alteram `crates/kernel`
executam uma a cada oito sementes (`sample_0` … `sample_3`, cerca de dois minutos); pushes para `main` e as
execuções noturnas executam todas (`all_0` … `all_3`).

Para registrar uma mudança intencional, forneça o motivo:

```
ROBUSTNESS_GOLDENS_BLESS="why it changed" cargo test --profile ci -p solidcraft-engine --lib robustness_goldens::all
```

As sementes alteradas e o motivo são adicionados a `notes` do arquivo; explique também na pull request.


Os registros comuns continuam sendo o padrão em todos os targets. Uma diferença numérica
revisada e observada em `x86_64-pc-windows-msvc` pode ser registrada separadamente em
`platforms` → `x86_64-pc-windows-msvc` no mesmo arquivo, com um `reason` não vazio
e um mapa `seeds`. Ali são aceitos apenas `area`, `volume`, `faces` e `edges`:
as medidas devem ser finitas e não negativas, e as contagens devem ser inteiras. Cada semente
listada já precisa ter um registro comum. Os valores específicos se aplicam somente àquele
target exato e àquela semente; os demais campos continuam comuns. Outras arquiteturas,
ambientes e targets usam o registro comum, sem curinga ou alternativa escolhida em tempo de
execução. Metadados de target malformados fazem a verificação falhar.

A comparação permanece igual: volume e área usam a mesma tolerância relativa de 1e-6, e as
contagens, etapas, resultados, erros e contagens de recusas continuam exatos. Registros por
target documentam o comportamento observado; eles não comprovam a causa da diferença nem a
correção geométrica. Antes de revisar um registro, execute todas as 96 sementes pelo fuzzer
público em grafos isolados do baseline limpo e do candidato, preserve as evidências e explique
cada nova diferença. Não arredonde as entradas do gerador nem ajuste o comparador para fazer
um registro passar.

Qualificação histórica: estas observações de 96 sementes, os seis overrides do Windows e as
duas passagens de Trunk se aplicam somente à base congelada `110d00d` com WIP PT-BR e Rust
1.99.0. Esse checkout concluiu `cargo xtask ci` em 2026-10-15 às 01:03:21 UTC. Isso não valida
a integração sobre `16c49c3`; verificações nativas, wasm, de corpus e de navegador das fontes
integradas continuam pendentes.

As observações registradas no Windows comparam todas as 96 sementes com a base limpa `110d00d`.
As sementes 1, 13, 23, 67, 92 e 94 já diferiam dos registros comuns naquela base.
A correção da identidade das estações do prisma remove emendas redundantes de paredes nas
sementes 13 e 71; a repetição dos mesmos comandos aceitos preserva área, volume, centroide,
validade e topologia mesclada. A semente 94 agora coincide com o registro comum, inclusive
no volume. Sua união final tem uma mudança real de volume medido mesmo com comandos idênticos;
um cálculo analítico independente do volume sustenta o novo resultado, com erro de tesselação
remanescente. Isso não comprova equivalência de material. Apenas as seis diferenças numéricas
restantes têm overrides do Windows. A execução no Linux ainda é necessária para determinar
o comportamento do candidato no target padrão da CI.

`ROBUSTNESS_GOLDENS_BLESS` é recusado no Windows para impedir a sobrescrita dos registros
comuns. Revise explicitamente os overrides numéricos do target suportado e seu motivo; registre
mudanças comuns intencionais no target padrão da CI. Ao atualizar sementes e notas comuns,
o gravador do target padrão preserva todo o bloco `platforms`.

## Novas tentativas booleanas bem-sucedidas

Quando um resultado booleano aceito exige uma nova tentativa que pode fazê-lo diferir do
exato (mover a ferramenta ou o atalho não verificado), a operação mostra um aviso na linha
do tempo. Outras tentativas trabalham em geometria temporária ou tolerâncias e mantêm o
resultado exato; são registradas, mas não geram aviso. `timeline.inspect` e `doc.inspect`
expõem cada tentativa em seus
`boolean_diagnostics`, incluindo a estratégia e a translação aplicada,
rotação, deslocamento de face e tolerância de tesselação em milímetros/radianos.

Um deslocamento comum dos dois operandos é revertido no resultado. Separação de faces coincidentes
e deslocamentos de furos descrevem geometria temporária de trabalho; não são
afirmações sobre o erro do modelo final. Uma face coincidente é separada somente
onde o espaço por onde se move contém o que a operação booleana deixaria ali de qualquer forma
(nada do destino para a face de um corte; material do destino para a de uma união);
uma face sem esse espaço permanece, e a alternativa planar exata é tentada primeiro.
Um pequeno ajuste da ferramenta altera o posicionamento relativo
dos operandos e informa tanto a magnitude da translação quanto o ângulo de rotação. Quando ele
falha ou deixa uma face estreita (menor que 1/200 da peça), também é tentado no sentido oposto:
um sentido pode empurrar uma face para dentro de uma face curva que ela apenas tocava (um corte
cuja extremidade é tangente a uma lateral arredondada) e deixar um pequeno trecho dela, enquanto
o outro sentido as separa (#897, #827).
Alternativas planares exatas e por partes informam deslocamento zero. As verificações existentes
de aceitação de volume e espaço continuam valendo para todos os resultados.

O comando que adiciona uma operação contém o mesmo `warning` em sua resposta, para que um
script ou agente veja uma dessas tentativas sem consultar a linha do tempo.

Uma união ou corte que precisou de qualquer nova tentativa é então limitado ao que pode fazer
(`crates/kernel/src/boolean_guard.rs`, #616): o resultado é fechado, uma união
nunca perde volume e um corte nunca ganha, e fora da caixa da ferramenta a superfície
do resultado é a do destino e a do destino é a do resultado (uma união ou
corte só pode alterar o destino onde a ferramenta está). Um corte só pode resultar vazio
quando a ferramenta cobre o destino. Um resultado que falha é recusado com um
erro indicando o problema e o local, em vez de ser retornado: um resultado errado
que parece correto é pior que um erro.

Relatórios pertencem à operação avaliada e permanecem após reconstruções em cache e
desfazer/refazer. Reabrir sem cache os gera novamente a partir do projeto incorporado. Ramos de novas tentativas
com falha ou descartados não contribuem com eventos de tentativas bem-sucedidas. Relatórios
são limitados a 1024 eventos por operação e identificam explicitamente o truncamento.

O JSON de fuzzing e o relatório de texto contam eventos de novas tentativas bem-sucedidas por estratégia para
projetos e verificações de identidade booleana separadamente, incluindo operações aninhadas.
São contagens de eventos, então uma única operação booleana ou semente pode contribuir com várias.
Relatórios truncados são contados separadamente, sem sugerir totais completos.

## Resultados

1000 sementes (0–999), 8 processos em uma máquina com carga alta (carga média 60–80), 90 s por
semente. Um "timeout" ali é principalmente um projeto com operações booleanas muito lentas (a semente 20 termina em
2 min 8 s sozinha), não um travamento. A execução "depois" teve mais desses casos: a máquina estava ainda mais ocupada
(suítes de testes executadas em paralelo), e operações booleanas com falha agora tentam mais alternativas antes de desistir;
menos projetos falham diretamente (27.1 % contra 36.3 %). Ambos os números são anteriores às duas últimas
correções abaixo (deslocamentos válidos de faces, ausência de panics na montagem final da operação booleana).

| Projetos | antes | depois |
|---|---|---|
| sem falhas | 49.8 % | 53.8 % |
| com falha | 36.3 % | 27.1 % |
| corpo inválido | 0.7 % | 0.9 % |
| volume no sentido errado | 0.2 % | 0.2 % |
| acima de 90 s | 13.0 % | 18.0 % |

| Identidade booleana | antes | depois |
|---|---|---|
| satisfeita | 85.4 % | 80.8 % |
| uma operação booleana falhou | 1.6 % | 1.2 % |
| não executada (o projeto excedeu o tempo) | 13.0 % | 18.0 % |

### Corrigido

- Uniões de contato face a face preservam divisões intencionais de Dividir face, inclusive em
  faces afastadas do contato. A etapa de colagem preserva suas linhas durante o reparo e no
  corpo resultante para operações posteriores. Regressões de contato circular parcial
  cobrem deslocamentos, rotação, ordem dos operandos, volume, área e histórico; o padrão da semente 16
  também é executado na suíte comum do mecanismo, com verificações analíticas de volume e área.
  A impressão visita um suporte analítico compartilhado apenas uma vez: as duas faces laterais de um
  cilindro não devem recortar o mesmo contorno em arcos redundantes.

- **Cantos de casca onde uma face curva encontra outra em ângulo** (a maior causa, 215 dos
  363 projetos com falha antes). Mover um vértice usava uma etapa linear, exata apenas para planos;
  um cilindro encontrando um plano em ângulo (um lado de arco raso) ficava fora da superfície deslocada.
  Agora isso é refinado por etapas de Newton sobre as superfícies deslocadas.
- **Arestas de casca que não são linhas nem arcos** (a parede de um furo encontrando um lado curvo): a
  aresta é reconstruída através de seus pontos deslocados.
- **Cascas de corpos em várias partes** ("this shell is not connected"): uma casca por parte.
- **Emendas de cilindros na grade.** As linhas de emenda de um cilindro ficavam nos eixos de seu sistema de coordenadas, exatamente
  onde faces posicionadas na mesma grade o cortavam, e operações booleanas falhavam ao longo da emenda. Emendas agora
  começam em um ângulo não alinhado: cilindros tangentes a faces de caixa por dentro ou por fora e cilindros
  cruzados que se tocam ao longo de uma linha agora funcionam.
- **Revoluções parciais de perfis que tocam o eixo** (um quarto ou meia volta de um retângulo
  a partir do eixo): a aresta no eixo deixou de varrer uma face de área zero, que fazia todas as tentativas
  booleanas falharem.
- **Operações booleanas em corpos com várias partes** (uma união que deixou dois volumes): quando o conjunto falha
  ou resulta vazio, processa-se uma parte por vez.
- **Afastar uma face** (faces coincidentes) verifica o resultado: a reconstrução exata podia deixar um
  cilindro inválido, e então o deslocamento geral é usado. Um ressalto sobre uma placa volta a unir.
- **Sem panics na montagem final booleana.** No código nativo, um panic dentro do kernel é capturado,
  mas a compilação web aborta em qualquer panic; o exemplo Carcaça plástica entrava em panic 8 vezes ao criar
  seus ressaltos e derrubava a página inicial no navegador. O shapeops incluído no repositório agora retorna
  `None`, e um teste cria todos os exemplos com um contador de panics. As execuções contam panics capturados
  por projeto ("Caught panics" no relatório), pois cada um derrubaria a compilação web.

### O primeiro tutorial (caixa, arredondamentos, casca)

Encontrados por QA nos primeiros passos de novos usuários, cada um com um teste do mecanismo comparado ao volume analítico
(`crates/engine/src/tests/robustness.rs`):

- **Casca de uma caixa arredondada** (arestas verticais r5, laço inferior r3, 2 mm, topo aberto): o arredondamento
  inferior percorre os cantos como toros fusiformes (o tubo mais largo que seu círculo), que o
  deslocamento agora aceita. Volume com erro relativo de até 1e-5 em relação ao analítico.
- **Casca mais espessa ou igual a um arredondamento** (arredondamentos r2 com casca de 2 mm): o interior do arredondamento
  colapsa em uma aresta viva, como no Fusion, em vez de "a cylinder shrinks to nothing".
- **Casca mais fina que um arredondamento tangente à face aberta** (1.5 mm): todas as faces se movem para
  dentro (tangências são mantidas), e a abertura é a cópia da face aberta varrida através do corpo.
- **Casca após um furo perfurado** (sua ponta é um cone).
- **Arredondar as arestas de uma caixa uma a uma**: uma aresta que encontra um arredondamento anterior
  remove esse arredondamento e arredonda ambos juntos, com o canto onde se encontram (`blend_corner.rs`,
  quaisquer raios).

### Velocidade

- Medir um corpo de 5000 faces (uma rosca modelada): de 13 s para 0.8 s.

### Limitações conhecidas (contadas como "não suportado", com mensagem clara)

- Uma casca mais espessa que uma parede ou degrau: as faces deslocadas se cruzariam e a cavidade se
  inverteria. O Fusion preenche esses locais com sólido; precisamos de um deslocamento que altere a topologia.
- Arredondamentos e chanfros entre duas faces curvas, ou terminando em faces que não são planares.

### Ainda falhando

- Uniões parciais de uma tampa e uma face (a extremidade de um cilindro apoiada parcialmente em uma face de caixa): mover
  uma face para dentro da outra adiciona material onde elas não se sobrepõem.
- Cascas de anéis parciais (um meio anel) cuja cavidade a operação booleana do truck rejeita, embora a
  mesma cavidade construída de outra forma corte corretamente.
- Operações booleanas muito lentas em corpos com muitas operações repetidas em padrão.
