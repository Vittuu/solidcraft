# Carregamento de projetos

## Arestas criadas por booleanas: cada ponto avaliado uma vez (2026-10-16)

Uma operação booleana gera malhas dos dois operandos com truck-meshalgo, que primeiro divide
cada aresta em uma polilinha (`bounded_polyline`, na cópia mantida no repositório). Uma aresta
criada por uma booleana anterior é uma curva de interseção. Cada ponto dela é encontrado por
uma busca em grade em cada uma das duas superfícies, com 51 × 51 avaliações de uma superfície
NURBS em cada uma, antes de Newton. A divisão avaliava novamente as duas extremidades de cada
trecho; assim, 2 de suas 4 avaliações por trecho repetiam um ponto que ela já tinha.
As análises de desempenho de `main` (01b1b705) na máquina abaixo atribuíram à divisão 42 % do tempo
de CPU para abrir o rover, 58 % de `bench-agent` e 82-85 % de um cubo de roda perfurado ou uma união de tubos.

Cada trecho agora carrega os pontos de suas extremidades, e seu ponto médio é tanto o ponto de
verificação quanto uma extremidade de cada metade: 2 avaliações por trecho. Os caminhos alternativos
para amostras que não podem ser avaliadas (#1364) são mantidos: um ponto médio que falha é NaN como
ponto de verificação e usa a aproximação da curva como extremidade, exatamente como antes.
A polilinha é a mesma, bit a bit: `edge_polyline_tests` a compara com a divisão anterior
(pontos, ordem e número de avaliações) nas curvas de interseção de um cilindro perfurado e em um círculo.

`solidcraft-cli` em release de `main` (73493b94) e o mesmo com a alteração, 3 pares alternados
(antes/depois, depois/antes, antes/depois), cada execução em um novo processo cronometrado com
`/usr/bin/time -v`, iniciado quando a média de carga de 1 minuto estava abaixo de 2.5. Medianas,
com a faixa das 3 execuções. Laptop de 2018: Intel Core i7-8565U (4 núcleos / 8 threads, 15 W),
7.4 GB, Ubuntu (Linux 7.0), conectado à tomada:

| Execução | Avaliação | Malhas | Tempo decorrido | Tempo de CPU | Pico de RSS |
|---|---:|---:|---:|---:|---:|
| `bench-open`, `examples/acceptance/sensor_enclosure.json` | 3,203 → 3,089 ms | 6 → 6 ms | 3.2 → 3.1 s | 8.7 → 8.5 s | 91 → 91 MB |
| `bench-open`, o corpo cilíndrico de um cubo de roda com um furo radial passante | 966 → 817 ms | 878 → 481 ms | 1.9 → 1.4 s (-24 %) | 4.4 → 2.8 s (-37 %) | 27 → 30 MB |
| `bench-open`, o quadro da bicicleta de `bench-agent` (uma união de tubos inclinados) | 8,204 → 6,277 ms | 1,711 → 893 ms | 9.9 → 7.2 s (-28 %) | 33.3 → 20.6 s (-38 %) | 44 → 46 MB |
| `bench-open`, um rover (3,324 operações, 1,395 corpos) | 26,783 → 22,792 ms | 7,960 → 6,046 ms | 35.7 → 29.8 s (-17 %) | 129.0 → 101.9 s (-21 %) | 2,076 → 2,079 MB |
| `bench-agent` (construção 33.5 → 30.2 s, -10 %) | 10,553 → 8,565 ms | 2,778 → 2,081 ms | 47.6 → 41.1 s (-14 %) | 124.0 → 91.0 s (-27 %) | 1,428 → 1,432 MB |

A avaliação e as malhas de `bench-agent` correspondem à reabertura do projeto salvo. O tempo de
CPU é a medida mais estável nesse processador de 15 W: suas faixas antes e depois não se sobrepõem
nos quatro casos com arestas de booleanas (o cubo de roda: 3.0-4.4 s antes, 2.5-2.9 s depois;
o rover: 113.6-133.5 s, 101.0-102.5 s), enquanto a execução "antes" do primeiro par, iniciada com o
processador frio, se sobrepõe aos tempos decorridos "depois" dos dois casos curtos. A caixa de sensor
não muda: seu tempo é gasto em buscas em grade na geração de malhas de faces, que esta alteração
não modifica. Uma série anterior em `main` (8535d229), em uma máquina com menos atividade, apresentou
-38 %, -35 %, -18 % e -16 % de tempo decorrido nos mesmos quatro casos. O tempo de CPU economizado
é próximo de metade da participação da divisão na análise de desempenho. A fase de malha também
melhora; portanto, essas malhas de exibição também chamam o gerador de malha de referência em algum
ponto (o local não foi rastreado aqui; `tess::edges`, a divisão própria de arestas dos planos de
exibição, já avaliava cada ponto uma vez). No rover, a segunda casca (que dá nomes às faces de
todos os corpos e, por isso, gera malhas para todos) passa de 7.8 s para 5.7 s.

Nenhum modelo mudou. `solidcraft-cli eval` dos cinco projetos (a bicicleta como `bench-agent` a
salvou) é igual com ambos os binários, exceto pelos milissegundos por operação: corpos, volumes,
áreas, centros, caixas delimitadoras, contagens topológicas e avisos. `bench-open` gera malhas
com as mesmas contagens de triângulos, e as bicicletas salvas pelos dois binários diferem apenas
no nome do documento.

## Montagens STEP: uma peça repetida gera malha uma vez (2026-10-14)

O leitor já converte cada item B-rep uma vez e o guarda para as instâncias que o repetem.
Cada instância ainda virava um corpo com sua própria geometria (necessário para operações de
corte e união em sua posição), e cada corpo gerava sua malha de exibição: tesselação, marcas de
emenda e tangência, a etapa mais cara de abrir grandes montagens. Uma placa com 60 de cada
resistor gerava a malha do mesmo resistor 60 vezes.

Uma instância posterior agora mostra a malha da primeira, posicionada pela diferença entre
os dois posicionamentos: os mesmos triângulos, faces e marcas de arestas, movidos. A malha
é criada uma vez, pela primeira instância desenhada. Apenas rotação e translação são
compartilhadas assim (outros casos geram sua malha), e qualquer alteração do que o corpo
mostra (linhas de divisão; qualquer operação, que cria um corpo novo) remove o vínculo.
Medidas, booleanas e exportação leem a geometria própria como antes; `solidcraft-cli eval`
de uma montagem fica inalterado.

Duas montagens geradas (projetos SolidCraft com cópias posicionadas de componentes, exportadas
como montagens STEP AP214), `solidcraft-cli bench-open`, build release, máquina Linux de 16
núcleos compartilhada:

| arquivo | corpos | fase de malha antes | depois | CPU antes | depois | pico RSS antes | depois |
|---|---:|---:|---:|---:|---:|---:|---:|
| 4 peças, 500 instâncias (parafusos com anel toroidal, suportes) | 901 | 2,215 ms | 139 ms | 28.4 s | 4.1 s | 918 MB | 749 MB |
| 10 peças arredondadas com 8 pinos, 600 instâncias | 5,401 | 1,028 ms | 85 ms | 16.7 s | 6.0 s | 1,095 MB | 783 MB |

Ambas importam as mesmas quantidades de triângulos (4.404.412 e 4.316.412). O plano de
exibição de cada instância (a vista GPU desenha corpos por planos) ainda é gerado por
instância; compartilhá-lo da mesma forma é a próxima etapa.

## Projetos construídos por agentes: trabalho por comando que crescia com o projeto (2026-10-14)

Um projeto construído por agente via MCP contém muitos comandos pequenos: um cilindro por
raio, uma caixa por placa da corrente, uma peça criada na origem e seu componente movido.
Cada comando percorria todo o projeto; o milésimo custava muito mais que o primeiro e um
projeto de 1.300 corpos levava minutos para construir e mostrar. `solidcraft-cli bench-agent`
constrói esse projeto (bicicleta gerada: cerca de 1.260 comandos, 1.280 corpos, 1.250 operações,
rodas criadas na origem e movidas), mede cada comando e o necessário para desenhar o resultado
(estado global e malhas de exibição), e abre o projeto salvo como `bench-open`. O script é
gerado, sem arquivo de exemplo; `--scale` muda as contagens e `--script-out` grava os comandos
para reproduzir em outro lugar (`engine.execute` um por vez pelo controle reproduz o desktop).

O que crescia com o projeto, por comando:

- **Componentes movidos eram posicionados novamente após cada alteração.** `world_state`
  transformava cada corpo de um componente com ocorrência movida e dava uma malha vazia nova;
  cada comando gerava essas malhas novamente (duas rodas: 150 corpos e dois toros, 130-190 ms
  por comando). Agora guarda corpos posicionados e reutiliza os de definição (mesmo corpo,
  mesma célula de exibição) e posicionamento inalterados.
- **Um nome `dN` livre pesquisava o projeto uma vez por número** (`next_d_name`), quadraticamente
  na quantidade de entradas nomeadas, para cada nova entrada de operação.
- **Cada expressão avaliava todos os parâmetros** (`Document::eval`), várias vezes por comando
  (cada coordenada, cada tamanho). Agora parâmetros só são avaliados se a expressão citar um.
- **`used_by` da tabela pesquisava todos os parâmetros para cada um** (quadrático); agora é
  uma passagem. O desktop também deixa de reunir sugestões de expressões (onze tabelas de
  parâmetros) a cada alteração quando nenhuma caixa de texto tem foco.

`solidcraft-cli bench-agent`, build release, Linux de 16 núcleos compartilhado com outras
compilações (carga média 15-20), uma execução de cada:

| por comando, por tamanho do projeto | antes: comando | antes: exibição | depois: comando | depois: exibição |
|---|---:|---:|---:|---:|
| 200 comandos (227 corpos) | 6.0 ms | 129 ms | 1.8 ms | 2.7 ms |
| 600 comandos (621 corpos) | 19.9 ms | 169 ms | 4.9 ms | 2.4 ms |
| 1,000 comandos (1,021 corpos) | 45.1 ms | 187 ms | 9.8 ms | 4.5 ms |
| 1,200 comandos (1,221 corpos) | 56.8 ms | 194 ms | 12.5 ms | 5.6 ms |
| construção completa (1,260 comandos) | 241 s | | 16.3 s | |

A coluna de comando ainda cresce (impressões digitais da linha do tempo e valores de
parâmetros são percorridos uma vez por comando), mas lentamente. A saída `solidcraft-cli eval`
do corpus de projetos, exemplos, scripts de regressão, bicicleta e projeto de 3.300 operações
construído por agente permanece igual, exceto pelos tempos.

A vista reconstruía e enviava novamente toda a cena a cada alteração (triângulos e arestas de
todos os corpos, 60 MB para a bicicleta com corpos construídos na CPU). Agora constrói a cena
em blocos de poucos corpos, divididos pela própria chave do corpo para que acrescentar/remover
altere só seu bloco; mantém bytes e buffers GPU de cada bloco enquanto ele reaparece igual.
As caixas dos corpos (ajuste, seleção, sombra no chão) são guardadas por malha da mesma forma.
Construída comando a comando pelo controle (`engine.execute`, depois `ui.inspect`) no desktop
sob Xvfb com GL por software:

| desktop, bicicleta construída pelo controle | main | com #1148 e isto |
|---|---:|---:|
| trabalho da UI por alteração com ~1,000 corpos (`frame_ms`) | 1,374 ms | 18 ms |
| construção da cena com ~1,000 corpos (`build_ms`) | 41 ms | 1.4 ms |
| construção completa | 1,409 s | 210 s |

A maior parte do restante é renderização por software de 770 mil triângulos por quadro.

## Importação STEP: cascas e sólidos de itens sem cópias (2026-10-13)

Um arquivo que lista uma definição de produto muitas vezes (um parafuso usado 70 vezes, um painel em várias
montagens) é importado item por item, e o B-rep de cada item era copiado profundamente duas vezes antes de
ser encaminhado: uma por quem chama `Solid::try_new`, que clona as cascas para que o caminho de falha
ainda as tenha, e outra pelo próprio cache do leitor, que mantinha o sólido para um item posterior
que pudesse listar a mesma definição.

`read_item` agora executa essas quatro verificações (não vazio, conectado, fechado, sem vértices singulares) em
cascas emprestadas, para que uma casca aprovada seja movida para o sólido em vez de clonada. O leitor
mantém cada sólido preparado em um `Arc`, e um item conta como *uma vez* quando uma representação
o lista e exatamente um percurso de definição de produto o alcança: o sólido desse item é movido para fora
do cache com `Arc::try_unwrap` em vez de copiado, pois nenhum item posterior pode solicitá-lo. O
percurso também acompanha alvos `MAPPED_ITEM`, portanto um item alcançado por um mapeamento no nó de outro
produto ainda é convertido uma vez - uma segunda conversão repetiria os avisos desse item. Um arquivo
cujo percurso excede o limite de trabalho (`MAX_CLOSURE_WORK`, 65 536 nós) recorre integralmente ao
caminho de cópia, assim como qualquer arquivo com ocorrências de componentes, que percorre novamente uma definição por
instância.

Nada observável mudou. Quinze conjuntos de teste (primitivas, um vazio, um arredondamento, uma revolução, cores,
unidades em centímetros, 70 duplicatas de uma definição, um arquivo danificado com mapeamentos, uma montagem de duas instâncias
e uma expandida; 99 corpos) fornecem o mesmo registro de importação - nome, caminho, sinalizador de fechamento,
faces do arquivo, origem de faces, topologia e contagens mescladas, caixa delimitadora, volume, área, centroide, pintura,
avisos e árvore - exceto os milissegundos por operação, e todas as quinze idas e voltas STEP de entrada/saída
são idênticas byte a byte.

Dois arquivos STEP planos gerados, cronometrados como na tabela acima (mesmo par de binários, nenhum outro trabalho
em execução); quatro execuções alternadas de cada um, duas para o arquivo de 31 MB:

| Arquivo | Pico de RSS de referência | Pico de RSS corrigido | Importação |
|---|---:|---:|---:|
| 2500 caixas, 9.5 MB | 150.1 MB | 145.9 MB | 1.81 s → 1.74 s |
| 2500 caixas arredondadas, 31 MB | 428 MB | 433 MB | 70.3 s → 71.0 s |

As faixas da primeira linha não se sobrepõem (150.0-150.6 MB contra 145.0-146.0 MB), e a economia é
a cópia de casca que deixou de ser feita para cada item. As execuções do segundo arquivo ficam entre 422 MB
e 444 MB em ambos os casos, portanto duas execuções de cada um não esclarecem o resultado: ali a conversão da geometria
domina a cópia por item. Um item que mantém todas as conversões - uma montagem ou um arquivo acima
do limite de trabalho - permanece inalterado.

## Planos de tesselação para exibição (#303, 2026-10-12)

`Body::display_mesh` gera malha a partir de um `TessPlan` (`crates/kernel/src/tess`): cada aresta é
amostrada uma vez, e ambas as faces usam exatamente esses nós (fechamento estanque por construção); faces
planas são uma triangulação Delaunay com restrições apenas de seus nós de arestas; faces delimitadas por
sua caixa de parâmetros são grades estruturadas conectadas aos nós das arestas; outras faces de quatro cantos
são grades de Coons; somente faces curvas efetivamente recortadas (furos, mais de quatro cantos) usam
a Delaunay com restrições do spade, com refinamento de flecha e Ruppert. Um corpo que o planejador não consegue
tratar é convertido em malha por truck-meshalgo como antes, que também continua gerando malhas para modelagem
(operações booleanas, medição, rastreamento de origem). Avaliar um plano (pontos e normais de superfície) ocorre na
CPU (`TessPlan::evaluate`) ou em um shader de computação (`solidcraft-gpu`).

Compilação de versão, Apple M4 Max, medianas de cinco. Placa de 400 furos (200 × 200 × 5 mm, 806
faces), `cargo test --release -p solidcraft-gpu bench_plate -- --ignored --nocapture`:

| Tolerância | Gerador de malha de referência | Plano | Avaliação na CPU | Avaliação na GPU | Triângulos (referência / plano) |
|---|---:|---:|---:|---:|---:|
| 0.3 mm | 23.3 ms | 11.9 ms | 1.1 ms | 1.1 ms | 20,812 / 20,812 |
| 0.03 mm | 54.2 ms | 31.6 ms | 2.5 ms | 1.9 ms | 76,812 / 52,812 |
| 0.006 mm | 118.9 ms | 65.1 ms | 5.3 ms | 3.1 ms | 302,412 / 104,012 |

O planejamento (amostragem de arestas, projeção de parâmetros, triangulação) domina; a avaliação leva
poucos milissegundos em ambos os casos, portanto a avaliação na GPU economiza pouco até o planejamento ficar mais barato. O
tempo da GPU inclui envio e leitura de retorno.

## Preparação paralela de corpos STEP (2026-10-12)

A análise de desempenho de um conjunto STEP original de 48 cilindros antes de alterar o importador mediu
3.17 ms de leitura, 6.06 ms de reconstrução de B-reps e 45.58 ms de preparação/orientação de corpos.
As cinco importações completas tiveram mediana de 59.21 ms. A malha custosa de orientação,
em vez do analisador de texto, foi o primeiro alvo de paralelização de #287.

A importação no desktop agora prepara sólidos independentes consecutivos em uma representação com
até oito workers. Cada lote contém no máximo 64 sólidos. Leitura, reconstrução B-rep,
percurso da montagem, resolução de unidades no primeiro uso, nomeação e coleta de resultados continuam ordenados.
Itens mapeados encerram um lote. Menos de quatro sólidos usam o caminho serial; wasm sempre o usa.
Falhar na criação de uma thread recorre ao trabalho serial, e falhas do núcleo/worker continuam sendo
erros por corpo informados pelo importador.

Windows x64, AMD Ryzen AI Max+ 395 (16 núcleos / 32 processadores lógicos), perfil `ci`
otimizado, sem compilação simultânea: três novos processos de teste por conjunto, cinco pares
seriais/paralelos alternados em cada processo. São importações completas do núcleo sem cache,
incluindo leitura e preparação de corpos; geração/exportação do conjunto e geração posterior de malhas
para exibição no desktop ficam fora da cronometragem. Ambos os caminhos usam o mesmo texto de entrada e código.

| Conjunto original gerado | Corpos | Mediana serial | Mediana de 8 workers | Redução |
|---|---:|---:|---:|---:|
| Cilindros, raio 5 mm, altura 12 mm | 48 | 57.54 ms | 17.38 ms | 69.8% |
| Cilindros, raio 5 mm, altura 12 mm | 512 | 896.31 ms | 313.06 ms | 65.1% |
| Toros, raio maior 20 mm, raio menor 4 mm | 48 | 269.05 ms | 51.90 ms | 80.7% |
| Caixas, 10 × 15 × 12 mm | 512 | 247.97 ms | 90.15 ms | 63.6% |
| Caixas, 10 × 15 × 12 mm | 4 | 1.49 ms | 1.02 ms | 31.5% |
| Um cilindro (alternativa serial) | 1 | 1.22 ms | 1.29 ms | alternativa serial |

Há 15 amostras por célula. Os modos de cilindro único executam código serial idêntico; sua
diferença mediana de 0.07 ms está dentro das faixas sobrepostas de 1.11–1.57 / 1.13–1.52 ms. As execuções de 512 cilindros variaram de 567.40–983.61 ms
em série e 197.72–345.51 ms em paralelo; a variação de escalonamento/temperatura é relevante. Esses
resultados sintéticos não estabelecem uma aceleração para todas as montagens ou para Abrir completo no desktop.
Em particular, peças contendo apenas um sólido não são paralelizadas entre nós de montagem.
Escalonamento entre peças, reconstrução B-rep paralela e malhas posteriores de exibição continuam pendentes.

Reproduza após compilar, sem outras compilações em execução:

```sh
cargo test -p solidcraft-kernel --profile ci benchmark_independent_step_solids --no-run
SOLIDCRAFT_STEP_BENCH_COUNT=512 SOLIDCRAFT_STEP_BENCH_KIND=cylinder \
  cargo test -p solidcraft-kernel --profile ci benchmark_independent_step_solids \
  -- --ignored --nocapture --test-threads=1
```

Use `torus` ou `box` para os outros conjuntos e repita em três novos processos.
`SOLIDCRAFT_STEP_BENCH_FIXTURE` opcionalmente grava o STEP original para inspeção nativa.
Nenhum limite de tempo é executado na CI. Regressões comparam importações seriais e multithread com
diferentes quantidades de workers: malhas de exibição exatas, medidas, identidades de origem de corpos/faces,
nomes, cores/opacidade, unidades, posicionamentos de montagens rotacionadas, duplicatas e falhas parciais
através da fronteira de lote. A suíte pública de referências analíticas/FreeCAD também verifica exportação/importação
STEP; seus resultados atuais são informados em docs/oracle.md.
## Armazenamento de projetos: contêiner e dados binários (#303, fase um)

Contêineres de projeto evitam formatar/analisar coordenadas de malhas importadas como números JSON.
O serializador de metadados substitui campos incorporados antes de criar valores JSON, sem
clonar o documento inteiro nem duplicar seu esquema. Coordenadas binárias mantêm precisão `f64`,
índices mantêm `u32`, e SHA-256 elimina duplicatas de dados idênticos. O leitor
verifica limites do arquivo compactado, CRCs, hashes, comprimentos exatos, coordenadas finitas e limites de índices.

Reproduza as medições de armazenamento com um executável otimizado de testes de entrada/saída no perfil `ci`:

```sh
cargo test -p solidcraft-io --profile ci benchmark_ -- --ignored --nocapture --test-threads=1
```

Os benchmarks geram uma malha original de 100,000 vértices/100,000 triângulos e um STEP
contendo 512 caixas separadas de 10 mm. Conjuntos são gravados sob `CARGO_TARGET_DIR`
(ou o diretório `target` da crate). Cada um compara nove amostras de codificação/decodificação;
amostras de salvamento atômico incluem codificação, flush do arquivo temporário, substituição e criação
de `.bak` e alternam a ordem JSON/contêiner. Os documentos são comparados após a decodificação.
São medições de armazenamento, separadas da avaliação da linha do tempo ou da latência do primeiro quadro.

No Windows x64, com base no upstream `e6e0a7e` antes do paralelismo STEP (#312), três
execuções otimizadas consecutivas, sem compilação simultânea, forneceram
as seguintes medianas das três medianas de nove amostras (milissegundos, modo rápido):

| Conjunto / operação | JSON | Contêiner |
|---|---:|---:|
| Tamanho da malha, bytes | 15,139,115 | 3,600,828 |
| Codificação de malha | 23.336 | 6.094 |
| Decodificação de malha | 57.175 | 2.962 |
| Salvamento atômico de malha | 52.924 | 14.856 |
| Tamanho STEP, bytes | 1,998,793 | 1,950,056 |
| Codificação STEP | 1.507 | 1.473 |
| Decodificação STEP | 1.479 | 1.318 |
| Salvamento atômico STEP | 7.213 | 7.087 |

Três execuções alternadas de `solidcraft-cli bench-open` em novos processos para cada codificação STEP
retornaram os mesmos 512 corpos e 6,144 triângulos. A mediana de leitura mais análise foi 2.274 ms
para JSON e 2.151 ms para o contêiner; a mediana de avaliação foi 218.559/236.915 ms e a de geração
de malhas de exibição 203.160/197.790 ms, respectivamente. A geometria domina, e esses resultados
variáveis não estabelecem uma melhora de abertura STEP de ponta a ponta. Novos processos
não implicam um cache frio de arquivos do sistema operacional, e esses não são tempos do primeiro quadro no desktop.

O modo rápido armazena dados STEP e de malha diretamente. `"compression":"compact"` explícito
os comprime no nível um do Deflate; é uma troca entre espaço e CPU. No experimento inicial
STEP de 512 caixas, a compressão reduziu 1,998,793 bytes JSON para 578,659 bytes de pacote,
mas aumentou a mediana de codificação de 1.68 para 5.02 ms e a de decodificação de 1.76 para
4.18 ms. Essa regressão explica por que a compressão não é o padrão. PNG/JPEG são armazenados
diretamente nas duas políticas; pequenos metadados paramétricos usam Deflate de nível um.

JSON simples continua como formato suportado de Salvar como/exportar; salvamentos `.json` mantêm JSON, e
scripts podem escolher explicitamente qualquer um dos formatos. A CLI detecta pacotes antes de tentar
analisar JSON. Recuperação de desktop/navegador e downloads do navegador também usam o contêiner.
Veja [o formato versionado de contêiner](design-container.md) para estrutura e limites.

Essa fase não restaura B-reps avaliados, não pula a importação STEP nem torna instantâneas reconstruções
de geometria sem cache. Uma chave de cache correspondente não basta para confiar em geometria não validada.
O incremento B-rep nativo descrito abaixo fornece exibição provisória, não restauração
de sessão editável. Ele é separado do paralelismo de importação de #287.

## Medições de prévia opcional de malha (#303, fase um)

A implementação de desktop da fase um pode mostrar uma imagem salva experimental enquanto o worker de arquivos reconstrói a geometria.
Exige uma chamada explícita de Salvar/Salvar como do mecanismo com `preview: true`. Salvamentos comuns omitem
malhas de exibição para evitar o custo adicional medido; essas medições são anteriores ao cache B-rep nativo.
O cache contém apenas malhas avaliadas de exibição já calculadas, limitadas a 8 MiB, 1,024
corpos e 100,000 triângulos. Modelos maiores ou não calculados são salvos sem ele. A geometria em cache
nunca é usada para editar, medir ou selecionar. Impressões digitais de origem/documento e
validação dos dados rejeitam caches desatualizados ou malformados; cancelamento e verificações de documento/revisão
impedem um resultado em segundo plano de substituir trabalho mais recente.

A imagem em cache é preparada no worker de arquivos a partir do mesmo instantâneo de arquivo do
comando Abrir normal. Acrescenta trabalho antes da reconstrução, em vez de evitá-la, portanto
meça separadamente a disponibilidade da prévia e a disponibilidade da edição completa. A exibição em cache também
acrescenta tamanho de arquivo e trabalho de salvamento; os resultados acima, exclusivos de armazenamento, excluem esses dados opcionais.

Para um benchmark de salvamento no mecanismo, primeiro gere o conjunto STEP original de 512 caixas
com o benchmark ignorado de entrada/saída, depois execute o benchmark de sessão com dados já calculados:

```sh
cargo test --workspace --profile ci benchmark_step_storage -- --ignored --nocapture --test-threads=1
cargo test --workspace --profile ci benchmark_warm_design_save_with_preview -- --ignored --nocapture --test-threads=1
```

Defina `CARGO_TARGET_DIR` como o diretório absoluto target do workspace em ambos os comandos.
O benchmark do mecanismo alterna nove salvamentos atômicos JSON/contêiner, inclui o cache opcional
de exibição e verifica tanto a decodificação da prévia quanto a identidade autoritativa do documento.
Três execuções no Windows, sem compilação ou suíte de testes simultânea, mediram medianas de salvamentos atômicos
JSON de 7.679/7.610/7.630 ms e de contêiner com prévia de
10.319/10.633/10.541 ms. A decodificação da prévia levou 0.964/0.961/1.130 ms; o contêiner
tinha 2,510,490 bytes. Essa prévia opcional custa tempo de salvamento nesse conjunto STEP.
Uma abertura real de caixa de sensor no desktop em e8a8e0f preparou sua imagem salva em
24.320 ms; reconstrução de geometria, preparação da exibição e preparação de miniaturas levaram
8,913.212 ms. O mesmo experimento de prévia agora é explicitamente opcional; a disponibilidade completa
da geometria ainda exige reconstrução.
Para as fases de abertura real no desktop, inicie com `SOLIDCRAFT_PROFILE_OPEN=1`: `ready_ms`
informa a preparação da imagem salva, enquanto o registro normal de fases informa reconstrução,
preparação de exibição e conclusão de miniaturas. Nenhum dos registros mede o primeiro quadro exato da GPU.

## Abertura com cache nativo (#303)

O cache B-rep nativo portátil é opcional com `cache: true` em salvamentos de contêiner
com dados já calculados, sem duplicar malhas de exibição. Salvamentos comuns e downloads do navegador
usam `cache: false` por padrão: as medições abaixo não justificam impor o tempo
adicional de salvamento e o tamanho extra de arquivo a todos os projetos. Abrir no desktop lê metadados e
geometria opcional por um arquivo com acesso por posição, produz uma imagem provisória limitada,
depois resolve as fontes originais e reconstrói a sessão completa antes de editar.
Isso melhora a oportunidade de exibição antecipada, não o custo da avaliação autoritativa
de operações. Um cache ausente, inválido ou incompatível usa a reconstrução original.
OPFS/downloads do navegador compartilham o formato; a abertura no navegador ainda reconstrói de forma síncrona.

Os benchmarks reproduzíveis do mecanismo separam salvamento atômico, disponibilidade de malha provisória,
reconstrução e disponibilidade final de malhas de exibição:

```sh
cargo test -p solidcraft-io --profile ci benchmark_step_storage -- --ignored --nocapture --test-threads=1
cargo test -p solidcraft-engine --profile ci benchmark_native_cache_save -- --ignored --nocapture --test-threads=1
cargo test -p solidcraft-engine --profile ci benchmark_native_cache_open -- --ignored --nocapture --test-threads=1
```

Use um `CARGO_TARGET_DIR` absoluto. O benchmark de salvamento usa por padrão o documento STEP gerado
de 512 caixas e aceita `SOLIDCRAFT_BENCH_FILE` para outro projeto público.
Alterna nove salvamentos de cada codificação, incluindo flush/substituição/backup, e
verifica a identidade do documento e a presença de cache nativo. Nunca mede a avaliação inicial do conjunto
como trabalho de salvamento. Abrir executa uma vez por processo; `SOLIDCRAFT_BENCH_ENCODING`
seleciona 0=JSON, 1=contêiner sem cache, 2=cache nativo, 3=cache nativo compacto.
Alterar somente a versão do aplicativo em `cache/key.json` fornece um controle de ausência de cache.
Novos processos não implicam um cache frio de arquivos do sistema operacional. O tempo do callback
do mecanismo é a disponibilidade de malha provisória, não um quadro renderizado no desktop; use o registro
de desempenho do desktop e capturas de tela reais para conferir a imagem salva e o modelo concluído.

Medido em 2026-10-13 no Windows 11, Ryzen AI MAX+ 395 (16 núcleos/32 threads), perfil `ci`,
com revisão do codec neutro `75381fa`, envelope `SCBREP03`, produtor
`solidcraft-truck-adapter/2` e revisão de integração de abertura `3243422` no upstream `2880525`
(0.6.0). Esses resultados substituem as medições anteriores do protótipo. Comparam codificações
no mesmo código e não atribuem ao cache melhorias de modelagem do upstream.
As execuções começaram após terminarem as compilações locais e a suíte completa de CI; atividades
comuns do desktop continuaram. Valores de salvamento são medianas de nove salvamentos atômicos
alternados. Valores de abertura são medianas de cinco novos processos por codificação, alternando
ordem direta/inversa. A memória é o pico de conjunto de trabalho de cada processo pelo
`GetProcessMemoryInfo` do Windows, amostrado a cada 10 ms, depois a mediana desses cinco
processos; exclui janela de desktop/GPU.

O conjunto STEP original é uma matriz de 32 por 16 cubos de 10 mm, separados por 12 mm, exportada
pelo SolidCraft: 512 corpos e 6,144 triângulos finais. Seu JSON incorpora cerca de 2 MB de
STEP. Todas as codificações reabrem sem erros de operações e com essas mesmas contagens.

| Codificação STEP | Bytes | Salvamento atômico, ms |
|---|---:|---:|
| JSON | 1,998,826 | 12.616 |
| Contêiner, sem cache | 1,950,084 | 10.516 |
| Cache nativo, rápido | 4,694,035 | 47.755 |
| Cache nativo, compacto | 837,067 | 44.015 |

| Codificação STEP | Malha provisória, ms | Reconstruído, ms | Malhas finais, ms | Pico do conjunto de trabalho, MiB |
|---|---:|---:|---:|---:|
| JSON | nenhuma | 175.842 | 748.911 | 40.102 |
| Contêiner, sem cache | nenhuma | 175.885 | 722.306 | 40.504 |
| Cache nativo, rápido | 62.099 | 228.794 | 747.976 | 40.883 |
| Cache nativo, compacto | 65.981 | 235.403 | 762.120 | 40.918 |
| Ausência de cache nativo | nenhuma | 169.039 | 647.617 | 40.453 |

O script público `examples/acceptance/sensor_enclosure.json`, salvo como projeto
JSON, tem 21 operações, dois corpos e 4,364 triângulos finais nessa revisão.

| Codificação da caixa de sensor | Bytes | Salvamento atômico, ms |
|---|---:|---:|
| JSON | 52,281 | 5.243 |
| Contêiner, sem cache | 5,545 | 4.969 |
| Cache nativo, rápido | 379,095 | 12.646 |
| Cache nativo, compacto | 72,603 | 14.111 |

| Codificação da caixa de sensor | Malha provisória, ms | Reconstruído, ms | Malhas finais, ms | Pico do conjunto de trabalho, MiB |
|---|---:|---:|---:|---:|
| JSON | nenhuma | 3,979.063 | 3,984.593 | 70.438 |
| Contêiner, sem cache | nenhuma | 4,038.127 | 4,042.006 | 71.789 |
| Cache nativo, rápido | 241.541 | 4,269.124 | 4,274.272 | 70.465 |
| Cache nativo, compacto | 236.739 | 4,271.254 | 4,275.159 | 70.148 |
| Ausência de cache nativo | nenhuma | 4,136.517 | 4,140.903 | 69.941 |

A exibição nativa fica disponível mais cedo, ao custo de codificação, tamanho de arquivo e trabalho
anterior à reconstrução. Não torna o projeto completo editável mais cedo, e
esses resultados não estabelecem uma redução de memória. A disponibilidade final de malhas variou
de 611 a 856 ms para STEP e de 3.84 a 4.60 s para a caixa; pequenas diferenças
nas medianas de abertura completa não devem ser interpretadas como acelerações de modelagem. O armazenamento compacto
reduz o tamanho e o tempo de salvamento atômico desse exemplo STEP, mas custa um pouco mais de tempo
de salvamento na caixa. O padrão `cache: false` mantém o caminho de salvamento da fase um;
`compression: "compact"` é uma escolha explícita entre espaço e CPU. Geometria maior ou não suportada
pode exceder os limites documentados e reabrir sem uma imagem provisória.

Uma verificação separada no desktop usou um novo processo do aplicativo 0.6.0 para cada codificação
da caixa, sem compilação local ou suíte de testes em execução. A imagem provisória do cache
nativo foi preparada em 429 ms e capturada visivelmente durante a abertura;
o modelo concluído foi adotado em 6.48 s. JSON não tinha imagem provisória e
terminou em 6.08 s. São verificações visuais únicas, não medianas de cronometragens repetidas;
o worker informou 6.10/5.66 s, respectivamente, e observações externas incluem
escalonamento da interface e solicitações de capturas de tela. Durante a abertura nativa, 94 inspeções do canal de
controle tiveram tempo mediano de ida e volta de 20.8 ms e máximo de 106.5 ms (JSON:
95 inspeções, 20.3/36.9 ms). A imagem provisória é explicitamente identificada como
em verificação antes da edição; o quadro final mostra Case e Lid reconstruídos e
sua linha do tempo editável de operações. Isso demonstra feedback antecipado, não conclusão
mais rápida da avaliação de operações nem uma garantia de latência máxima da interface.

## Preparação de origem de extrusão analítica

Certificados de extrusão de novo corpo agora preparam os segmentos analíticos limitados do esboço uma vez
por passagem de certificação, em vez de alocar segmentos e resolver seus pontos para cada
contorno gerado de tampa e face lateral. Projetos salvos executam o mesmo código durante reconstruções sem cache.
Tolerâncias exatas de contenção, rejeição de origens ambíguas e o limite de trabalho de certificação
permanecem inalterados. Segmentos preparados usam um esboço avaliado imutável emprestado e não são mantidos
entre alterações.

O microbenchmark reproduzível é:

```sh
cargo test -p solidcraft-doc --profile ci benchmark_prepared_extrude_sources -- --ignored --nocapture
```

Três execuções otimizadas de `ci` no Windows x64, durante outra compilação, compararam o laço anterior
de reconstrução de segmentos e o comparador preparado no mesmo processo. Para 1,000 linhas
analíticas e 1,000 segmentos gerados de faces, a mediana de tempo caiu de **45.90 ms para
12.30 ms** (73% menos, cerca de 3.7x mais rápido). A preparação é incluída, e todas as identidades retornadas de
origem são comparadas. Isso mede somente certificação, não extrusão completa, latência de abertura
de arquivos ou operações booleanas; não estabelece aceleração de carregamento de ponta a ponta.
Os custos restantes de reconstrução de geometria e importação sem cache são acompanhados em
[#255](https://github.com/bherbruck/solidcraft/issues/255); o trabalho de certificação é
[#254](https://github.com/bherbruck/solidcraft/issues/254).

## Reconstruções de geometria sem cache após 0.3.1

Três comparações alternadas em processos novos na mesma máquina Windows e perfil `ci`
otimizado, sem compilação simultânea, comparam o código inalterado 0.3.1 com as alterações de projeção
de linhas retas e reutilização de refinamento exato de superfícies:

| Projeto salvo | Antes, mediana da avaliação | Depois | Redução |
|---|---:|---:|---:|
| Caixa pública de sensor, 21 operações, 52 KB | 11.96 s | 8.85 s | 26.0% |
| Montagem local de caixa Pi, 25 operações, 2,990 corpos | 14.75 s | 11.57 s | 21.6% |

São tempos de reconstrução de geometria, não tempos completos de abertura no desktop. O conjunto público
pode ser gerado com `solidcraft-cli run examples/acceptance/sensor_enclosure.json
--save sensor.solidcraft --quiet`, depois medido com `solidcraft-cli bench-open
sensor.solidcraft`. Cada benchmark inicia um novo processo; isso não mede um acerto
em cache já calculado da linha do tempo. A montagem local é privada e não faz parte do repositório.

Segmentos retos agora usam projeção algébrica protegida do ponto mais próximo em vez de amostrar
65 pontos e executar iterações de Newton. Outros tipos de curvas mantêm o comportamento existente.
O refinamento de faces curvas reutiliza avaliações de derivadas e pontos médios com chaves exatas de entradas
finitas e caches locais limitados. Mantém as verificações originais de tolerância, emendas e singularidades,
passagens de refinamento e limites de triângulos.

Posicionamentos identidade de montagem mantêm o cache compartilhado de malhas de exibição do corpo da definição.
Posicionamentos alterados continuam criando malhas separadas em suas próprias coordenadas. A preparação de nomes
de faces permanece antecipada durante o posicionamento: adiá-la apenas transferiu trabalho custoso para
miniaturas e a primeira consulta de face, portanto essa abordagem foi excluída.

Ambos os conjuntos salvos mantêm exatamente os mesmos nomes de corpos, contagens de faces/arestas/vértices/cascas,
limites, volumes e áreas de superfície. Testes específicos comparam coordenadas e normais de refinamento com
e sem cache bit a bit em superfícies periódicas, singulares e transformadas.

Para analisar fases no desktop, defina `SOLIDCRAFT_PROFILE_OPEN=1` antes de iniciar o aplicativo.
Abrir interativo informa tempos de geometria, posicionamento, tesselação, miniaturas e total do
worker em stderr sem registrar o caminho de origem. Miniaturas corresponderam a cerca de 0.1 segundos da
abertura da montagem lenta e não eram o principal gargalo. Ainda restam custos de operações booleanas e
importação sem cache; essas mudanças não tornam instantânea a abertura de todos os documentos.

### Geometria preparada de montagens e seleção

A geometria global preparada agora é identificada pelo documento, modelo avaliado e contexto exato de
posicionamento, em vez da revisão da sessão. Selecionar uma face ou aresta, alterar visibilidade
ou mover a vista não descarta mais os corpos e malhas de montagem já preparados.
Alterações de geometria, movimentos pendentes de ocorrências, mudanças de contexto de autoria, desfazer/refazer e retorno da
linha do tempo ainda invalidam o cache. Sessões temporárias compartilham geometria preparada enquanto suas
entradas correspondem, e adotar uma sessão calculada mantém seu cache já preparado.

Na montagem privada de Pi com 2,990 corpos, uma medição correspondente de controle no desktop Linux reduziu
o tempo de ida e volta de seleção de aresta de 9.24 s para 0.57 s. A abertura inicial ficou essencialmente inalterada
(33.54 s para 33.32 s); isso corrige uma interrupção separada de seleção, não a avaliação de geometria sem cache.
Três novas execuções do desktop Windows com o cache corrigido mediram mediana de 15.87 s até
ficar pronto: 13.04 s de geometria, 2.20 s de posicionamento, 0.53 s de geração de malhas e 0.07 s de miniaturas. Usaram
o perfil `ci` otimizado e renderização Vulkan real, com configurações e caches isolados.
Os tempos de Linux e Windows são medições separadas e não devem ser comparados como aceleração.

`solidcraft-cli bench-open design.solidcraft` informa linhas JSON de leitura de arquivo,
decodificação JSON e migrações (incluindo reparo de nomes de entradas), avaliação de linha do tempo, operações
individuais e posicionamento global/geração de malhas de exibição. `--parse-only` interrompe antes da avaliação de geometria.
A cronometragem de análise inclui validação e reparo do documento, não somente o analisador JSON.

Windows x64, perfil `ci` otimizado, medido durante outras compilações:

| Projeto | Análise/reparo | Avaliação | Malhas de exibição |
|---|---:|---:|---:|
| Caixa de 19 operações, 34 KB | 1.2 ms | 14.9 s | 6 ms |
| Montagem de 25 operações, 15.2 MB, 2,990 corpos | 21 ms | 12.5 s | 2.1 s |
| 1,000 caixas independentes geradas, 185 KB | 5 ms | 0.82 s | 0.15 s |
| 5,000 caixas geradas em cinco grades, 10 operações | 0.45 ms | 17.6 s | 0.81 s |

As caixas gastam a maior parte da avaliação em uniões e cortes, não na análise. Os projetos salvam definições
de operações; abri-los reconstrói sua geometria. Esses tempos permanecem após a correção de reparo.

Uma linha do tempo gerada separada com nomes de entradas ausentes reproduziu minutos de trabalho no leitor:

| Linha do tempo (geometria retrocedida) | Análise/reparo antes | Depois |
|---|---:|---:|
| 2,000 caixas, 6,000 entradas sem nomes | 78.1 s | 10–20 ms |
| 5,000 caixas, 15,000 entradas sem nomes | interrompido após 131 s, ainda reparando | 26–46 ms |

Antes, cada entrada ausente reiniciava a busca de `d1`, `d2`, …, percorrendo o documento para
cada candidato. Isso se aproxima de trabalho cúbico à medida que entradas sem nomes se acumulam. O carregamento agora reserva
nomes uma vez e atribui nomes ausentes com um cursor que avança. Nomes existentes, nomes em operações posteriores,
colisões de parâmetros e reutilização de nomes excedentes mantêm o comportamento anterior.
Testes de regressão comparam os reparos antigo e em lote e carregam uma linha do tempo de 5,000 operações sem nomes.

# Desempenho da vista

Medido pelo canal de controle (`ui.inspect` `frame_ms`: tempo de CPU de um quadro da interface, que
inclui seleção sob o cursor) na placa de exemplo repetida em padrão 20 × 20 (400 corpos, cerca de
600k triângulos), janela de 1600 × 1000 sob Xvfb, compilação de versão. Ponteiro: 60 movimentos do cursor sobre
o modelo; órbita: arraste com o botão direito.

| | mediana do ponteiro | p90 do ponteiro | mediana da órbita |
|---|---|---|---|
| antes | 18.7 ms | 37.5 ms | 21.0 ms |
| depois | 1.9 ms | 2.4 ms | 2.6 ms |
| depois, árvore do modelo oculta | 0.4 ms | — | 0.4 ms |

O que mudou:
- **A seleção descarta por caixa delimitadora.** Caixas por corpo ficam em cache até o modelo mudar. O
  teste de raios de faces ignora corpos cuja caixa o raio não atinge ou na qual entra atrás do impacto mais próximo até então.
  Testes de arestas e vértices ignoram corpos cuja caixa projetada na tela não está sob o cursor.
- **Seleções são reutilizadas** enquanto cursor, câmera e modelo permanecem inalterados, e **nenhuma seleção é executada
  enquanto a vista se move** (órbita, deslocamento, zoom).
- **A extensão do modelo fica em cache** (define o intervalo de profundidade da vista e era recalculada de todas as
  malhas várias vezes por quadro).

Com 49 corpos, o quadro já levava 2.8 ms (ponteiro) antes dessas alterações. O tempo restante de
quadro com a árvore do modelo mostrada é principalmente a árvore listando cada linha de corpo. Seleção por buffer de IDs
na GPU não foi necessária nesse tamanho.

## Árvore do modelo

Mesmo modelo e método (400 corpos, pasta Corpos aberta, 60 movimentos do ponteiro sobre a vista):

| | mediana | p90 |
|---|---|---|
| Árvore do modelo mostrada, antes | 1.95 ms | 2.71 ms |
| Árvore do modelo mostrada, depois | 0.72 ms | 1.19 ms |
| Árvore do modelo oculta | 0.30 ms | 0.72 ms |

O que mudou: linhas roladas para fora da vista apenas reservam sua altura. Não registram widgets
nem pintam nada (nenhum layout de texto, ícone ou botão de olho/recolher), portanto o custo acompanha as linhas na
tela em vez das linhas na árvore. Rolagem, Localizar na árvore do modelo e arrastar e soltar ainda reconhecem
o lugar de cada linha.

# Desempenho de recálculo

`solidcraft-cli bench-edit <script> <param> "(<old>) * 1.02"` constrói um projeto a partir de seu script
(cronometrando a construção), depois mede uma alteração de parâmetro, a restauração da expressão
antiga do parâmetro, desfazer e refazer. Cada célula é antes → depois do cache de avaliação, com a quantidade
de operações recalculadas entre parênteses. Compilação de versão em uma máquina compartilhada com outros jobs de compilação (carga média de cerca de 45; os
tempos variam ±50 % entre execuções, portanto compare as contagens de recálculos e os zeros). "Inicial" altera um parâmetro do
primeiro esboço ou operação; "final", um das últimas operações.

| Projeto | Alteração | Construção | Alteração | Restaurar | Desfazer | Refazer |
|---|---|---|---|---|---|---|
| placa de exemplo, padrão 20 × 20 | inicial (`width`) | 7587 ms (10) → 1764 ms (10) | 7969 ms (10) → 1691 ms (10) | 7557 ms (10) → 0 ms (0) | 6534 ms (0) → 0 ms (0) | 4570 ms (0) → 0 ms (0) |
| placa de exemplo, padrão 20 × 20 | final (`d9`) | 7870 ms (10) → 1699 ms (10) | 3944 ms (1) → 812 ms (1) | 4039 ms (1) → 0 ms (0) | 1464 ms (0) → 0 ms (0) | 1398 ms (0) → 0 ms (0) |
| oracle 58 suporte de celular | inicial (`d1`) | 163 ms (6) → 140 ms (6) | 228 ms (5) → 140 ms (5) | 171 ms (5) → 0 ms (0) | 158 ms (0) → 0 ms (0) | 130 ms (0) → 0 ms (0) |
| oracle 58 suporte de celular | final (`d4`) | 144 ms (6) → 195 ms (6) | 118 ms (1) → 118 ms (1) | 145 ms (1) → 0 ms (0) | 149 ms (0) → 0 ms (0) | 187 ms (0) → 0 ms (0) |
| oracle 59 base de caixa | final (`d8`) | 11582 ms (11) → 11179 ms (11) | 5772 ms (1) → 5742 ms (1) | 5606 ms (1) → 0 ms (0) | 6023 ms (0) → 0 ms (0) | 6119 ms (0) → 0 ms (0) |
| oracle 60 flange de tubo | inicial (`d1`) | 8137 ms (7) → 8348 ms (7) | 7569 ms (6) → 10143 ms (6) | 8354 ms (6) → 0 ms (0) | 8607 ms (0) → 0 ms (0) | 10034 ms (0) → 0 ms (0) |
| oracle 60 flange de tubo | final (`d6`) | 7661 ms (7) → 11990 ms (7) | 67 ms (1) → 361 ms (1) | 69 ms (1) → 0 ms (0) | 84 ms (0) → 0 ms (0) | 66 ms (0) → 0 ms (0) |
| oracle 61 rasgo de chaveta de eixo | inicial (`d1`) | 1723 ms (6) → 3016 ms (6) | 1785 ms (5) → 2745 ms (5) | 1760 ms (5) → 2323 ms (5) | 1667 ms (0) → 0 ms (0) | 1719 ms (0) → 0 ms (0) |
| oracle 61 rasgo de chaveta de eixo | final (`d4`) | 1905 ms (6) → 2247 ms (6) | 1734 ms (1) → 2101 ms (1) | 1682 ms (1) → 0 ms (0) | 1719 ms (0) → 0 ms (0) | 1876 ms (0) → 0 ms (0) |
| oracle 62 dissipador de calor | inicial (`d1`) | 382 ms (6) → 643 ms (6) | 362 ms (5) → 697 ms (5) | 233 ms (5) → 568 ms (5) | 245 ms (0) → 0 ms (0) | 360 ms (0) → 0 ms (0) |
| oracle 62 dissipador de calor | final (`d5`) | 479 ms (6) → 688 ms (6) | 270 ms (1) → 657 ms (1) | 321 ms (1) → 0 ms (0) | 413 ms (0) → 0 ms (0) | 283 ms (0) → 0 ms (0) |
| oracle 63 articulação de dobradiça | inicial (`d1`) | 1305 ms (8) → 1671 ms (8) | 1329 ms (7) → 2397 ms (7) | 1971 ms (7) → 0 ms (0) | 1705 ms (0) → 0 ms (0) | 1321 ms (0) → 0 ms (0) |
| oracle 63 articulação de dobradiça | final (`d4`) | 1430 ms (8) → 2072 ms (8) | 1387 ms (1) → 2073 ms (1) | 1480 ms (1) → 0 ms (0) | 1192 ms (0) → 0 ms (0) | 1035 ms (0) → 0 ms (0) |

O que mudou: toda avaliação de operação fica em cache por uma chave composta pelas próprias entradas da operação
(definição e valores de parâmetros), pelo contexto do documento que ela pode ler (planos de construção,
regras de chapa metálica e plástico, componentes) e por um resumo do modelo de partida, no qual
cada corpo é identificado pela avaliação que o produziu (corpos que uma operação repassa
inalterados mantêm sua identidade). A mesma operação nas mesmas entradas nunca é avaliada duas vezes:
restaurar um valor, desfazer, refazer, uma prévia seguida pelo mesmo comando e as avaliações
repetidas enquanto um script constrói o projeto reutilizam resultados anteriores. O cache é compartilhado por um
modelo e suas cópias (prévias, sessões temporárias) e mantém os últimos 4096 resultados dentro de 64 MiB.

Uma alteração ainda recalcula a operação editada e todas as posteriores cujo modelo de partida
mudou; nessas peças, todas as operações posteriores atuam no corpo editado, portanto alterações iniciais recalculam
quase tudo, como devem. `cached_evaluation_matches_from_scratch` verifica que avaliações com cache e
desde o início fornecem os mesmos modelos (corpos, volumes e áreas com precisão de 1e-6, topologia,
erros) durante alterações, supressão, retorno, desfazer e refazer. Restaurar uma dimensão de esboço
ainda pode recalcular (oracle 61, 62): resolver o esboço move seus pontos por ruído de arredondamento, portanto
suas entradas não são exatamente as antigas. A primeira alteração de parâmetro de oracle 59 falha (uma expressão
da receita que o `* 1.02` do benchmark não consegue escalar) e é omitida.

## Pares de prismas, medidas de corpos e limite do cache de avaliação (2026-10-13)

Um projeto cujas operações repetem uma forma (padrão de furos, placa cortada por muitas ferramentas, peça construída
por um corpo em crescimento) paga vários custos por operação que uma reconstrução pagava repetidamente.

**O tamanho de um corpo é medido uma vez, não uma vez por chamada.** Cada tolerância do núcleo é uma fração da
diagonal da caixa delimitadora do corpo, e `Body::size()` percorria todos os vértices do sólido para calculá-la.
`Body::new` já lê esses vértices para orientar o corpo, portanto mantém a diagonal que mediu
ali; um corpo construído manualmente ou um corpo de malha ainda mede uma vez.

**O mesmo volume não é medido duas vezes.** Uma operação booleana mede os volumes de ambos os operandos para limitar seu
resultado, e o corpo retornado se torna o operando da próxima operação, portanto uma linha do tempo de *n* cortes
gerava malha de cada corpo intermediário duas vezes. `Body::volume_at(tol)` mantém a última medição do
sólido, compartilhada por suas cópias (compartilham o sólido), e os limites a solicitam com a mesma tolerância todas
as vezes.

**Um par de prismas é decidido em 2D antes de gerar malha de qualquer operando.** A operação booleana geral cria uma
malha refinada de ambos os operandos antes de decidir qualquer coisa, e essa malha corresponde à maior parte do custo: um
padrão ou esboço com vários perfis reconstrói a malha do alvo inteiro uma vez por ferramenta. Uma
placa e uma ferramenta que a atravessa são prismas em uma direção, que `prism::prism_boolean` decide
exatamente a partir de suas seções 2D - e esse caminho era executado *depois* das malhas. Um corte ou uma interseção
agora o consulta primeiro e retorna sem criá-las. Um par para o qual o caminho não tem resposta usa o caminho
antigo inalterado, com malhas e tudo; uma união ainda consulta após o atalho amostrado. Uma resposta é enviada
de volta a esse atalho intencionalmente: quando o caminho 2D retorna um de seus operandos (uma ferramenta que
não atinge o alvo ou o cobre inteiro), o caminho amostrado retorna esse mesmo corpo, para que um corpo intacto
mantenha sua identidade na linha do tempo (`ops::prism_fast_path_tests`).

**Um posicionamento lê sua orientação da matriz.** Posicionar um corpo (uma instância de padrão, uma
cópia alinhada) antes gerava malha da cópia inteira para decidir para onde suas faces apontavam e contar as
faces ausentes na malha grosseira. Um posicionamento rígido mantém ambos como estavam; um espelhado (um
determinante negativo) vira o sólido do avesso. Ambos agora são lidos da matriz, portanto somente a
geometria é transformada.

**O cache de avaliação é limitado pelos bytes que mantém.** Mantinha os últimos 4096 resultados, e um
resultado é tão grande quanto o modelo que contém, portanto uma longa sessão de edição mantinha cada estado substituído:
nada descartava entradas que não eram mais referenciadas. Agora também mantém no máximo 64 MiB, descartando as mais antigas
primeiro, e sempre mantém o resultado mais recente, para que uma única avaliação nunca seja descartada imediatamente
após ser criada.

`solidcraft-cli` de versão, `eval <design> --quiet`, tempo de relógio do processo inteiro e pico de RSS
por `/usr/bin/time -v`, alternando os binários de referência e corrigido, mediana de 12 execuções de cada um
(8 para a placa de 400 furos e a placa de ventilação), sem outra compilação ou teste em execução (Fedora Linux, AMD
Ryzen 7 5700X, 8 núcleos / 16 threads, 32 GB):

| Projeto | Referência | Corrigido | Variação | Pico de RSS de referência | Pico de RSS corrigido |
|---|---:|---:|---:|---:|---:|
| caixa de 400 × 400 × 6 mm, um esboço com 100 círculos cortados através dela | 1.24 s | 0.83 s | -33% | 151 MB | 119 MB |
| A mesma placa com 400 círculos no esboço | 5.67 s | 4.04 s | -29% | 556 MB | 481 MB |
| Placa de ventilação, um padrão 10 × 8 de um corte | 0.10 s | 0.08 s | -20% | 22 MB | 22 MB |
| padrão 14 × 14 de uma caixa arredondada (196 cópias) | 6.70 s | 6.52 s | -2.6% | 67 MB | 67 MB |
| 60 esboços, 60 extrusões em um corpo em crescimento | 8.80 s | 8.88 s | +0.9% | 225 MB | 225 MB |
| `examples/acceptance/sensor_enclosure.json` | 3.81 s | 3.30 s | -13% | 198 MB | 196 MB |

As placas de furos são o caso para o qual existe o caminho de prismas: uma operação booleana cuja ferramenta é um prisma com 100 ou 400
seções, em um esboço com essa quantidade de círculos, portanto somente a malha da ferramenta é economizada pelo caminho.
Uma variante da árvore corrigida que consultava o caminho na ordem antiga - após as malhas dos operandos -
mediu a placa de 100 furos em 1.27 s e 157 MB (faixa de 1.22-1.30 s), os próprios números da referência
(1.25 s e 156 MB nas mesmas execuções, faixa de 1.21-1.33 s). As duas linhas de placas, e o pico que
elas eliminam, são o caminho de prismas.

Os últimos três projetos são o outro caso: suas operações booleanas não se encaixam nisso. O projeto de 196 cópias
e o de 60 esboços ficam dentro de 2.6% e 0.9%, com faixas sobrepostas. As medianas da caixa de sensor
diferem 13%, mas suas execuções ficam em qualquer lugar entre 3.0 s e 5.0 s, e as duas faixas
se sobrepõem (3.01-4.97 s contra 3.00-4.89 s), portanto sua linha não demonstra um resultado em qualquer sentido e é mantida como
medida. As medidas de corpos, o posicionamento e o limite de cache não são o que essas linhas
medem: interrompem trabalho que o conjunto repete raramente demais para aparecer (um percurso oculto de vértices, um operando
convertido em malha duas vezes para um número, uma malha de orientação por posicionamento, um cache sem limite de bytes).

Nenhum modelo mudou. O registro de avaliação de 21 projetos (10 a 400 círculos em uma placa, grades de caixas de 16 × 16 a
31 × 31 com e sem arredondamentos, grades de 2500 corpos, montagens de 40 a 500 corpos, 30 e
60 esboços, padrão de 196 instâncias, exemplo incluído e caixa de sensor) é o mesmo modelo
antes e depois - corpos, volumes, áreas, centros, caixas delimitadoras, contagens de topologia e avisos -
exceto os milissegundos por operação. A
suíte de testes do núcleo passa, incluindo quatro novos testes do caminho de prismas e de sua regra de identidade.

Reproduza após compilar uma versão, sem outras compilações em execução: cronometre
`target/release/solidcraft-cli eval <design> --quiet` com `/usr/bin/time -v`, alternando os
binários de referência e corrigido, e compare o centro das execuções. O conjunto (20 projetos gerados:
placas com furos, grades de caixas, montagens, peças com muitos esboços) e os scripts de medição estão em
https://github.com/sahenjp/solidcraft-perf sob `perf/`.

## Carregamento no desktop e miniaturas recentes (2026-10-10)

A caixa para Arduino Uno (19 operações, 34 KB) e a caixa para Raspberry Pi 5 (24 operações,
61 KB) são decodificadas em cerca de 1 ms cada. Uma compilação de versão gasta aproximadamente 12.1 s e 8.2 s,
respectivamente, avaliando suas geometrias nesta máquina Windows. Alterar a codificação do documento
não elimina essas operações booleanas.

Abrir interativo, cliques em arquivos recentes, arrastar/soltar e inserção STEP/malha agora executam em um
worker, incluindo a preparação dos corpos em coordenadas globais e das malhas de exibição. O projeto existente
continua disponível até o resultado ficar pronto; um job que falhe ou seja cancelado não pode substituí-lo.
A inserção preserva arquivo, estado salvo, histórico anterior de desfazer e configurações da vista.
Cópias da linha do tempo compartilham sua célula de malha preguiçosa, e adotar uma sessão preparada mantém seu cache
global já calculado em vez de transformar e tesselar seus componentes novamente na interface.
A verificação de faces ausentes de um corpo STEP reutiliza sua tesselação de orientação quando as tolerâncias
correspondem e sua orientação não foi invertida.

Testes reais no desktop (perfil `ci`, com outras compilações/testes em execução) abriram as caixas Arduino e
Pi em 15.2 s e 10.5 s. Durante esses jobs, os tempos medianos de ida e volta de `ui.inspect` foram 20.5 ms
e 8.3 ms; os mais lentos foram 33.7 ms e 28.6 ms. Isso corrige o congelamento da janela, não o
custo restante da avaliação de geometria sem cache.

Miniaturas recentes são arquivos PNG de 240 × 150 em `SolidCraft/thumbnails` sob o diretório de cache
da plataforma (`LOCALAPPDATA` no Windows). Suas chaves incluem caminho de origem, tamanho
do arquivo e horário de modificação. Abrir e Salvar renderizam a partir do modelo já em memória. Arquivos recentes
em cache decodificam o PNG sem ler nem reconstruir um documento e são executados antes de recentes sem cache
e miniaturas de exemplos. Imagens ausentes ou inválidas recorrem à reconstrução em segundo plano.
Nenhuma miniatura ou imagem base64 é adicionada ao JSON do projeto. Um teste de regressão verifica que os
bytes de origem permanecem inalterados, uma imagem em cache funciona mesmo sem um arquivo de origem
que possa ser analisado e alterar a origem invalida o cache.

`solidcraft-cli bench-open` também aceita STEP/IGES. Para arquivos de intercâmbio, sua fase `parse`
inclui construir e verificar o B-rep importado, não apenas decodificar a sintaxe STEP.

O teste real carregou ambos os cartões recentes em cache em 21 ms e inseriu o Pi STEP leve de 2 MB
(27 corpos) na caixa em 279 ms; desfazer restaurou os dois corpos originais. O STEP completo
da placa de 77 MB (2689 definições de sólidos, 45,689 faces) ainda não tinha terminado após
300 s. Sua janela continuou atendendo inspeções e Cancelar, e cancelar preservou a
caixa. Construção STEP complexa continua sendo um problema pendente de desempenho do núcleo.

# Solucionador de esboços e arrastes (2026-10-11)

Incógnitas que as restrições determinam uma após a outra a partir das constantes (as linhas próprias de cada uma
com posto completo e bem condicionado, lendo apenas constantes e incógnitas determinadas antes dela)
são eliminadas exatamente: valores posicionados pela solução construtiva inicial e cujas restrições são satisfeitas
permanecem constantes enquanto Levenberg–Marquardt resolve o restante, e a análise de posto (graus de
liberdade, geometria determinada) executa apenas no restante. Um componente sem ligação com constantes usa
primeiro um ponto como constante (sua translação). Todas as resoluções com fixação de um quadro de arraste ao vivo
compartilham um limite de trabalho de decomposição de 1M (cerca de 16 ms; o próprio de uma resolução, 20M, é cerca de 0.3 s), e os
passos de arraste são um quarto do menor tamanho fixado em vez de um oitavo, exceto depois que uma solução inicial
esgotar o limite: o restante desse arraste avança em oitavos, como antes, e a continuação mantém
o ramo que a solução inicial não posiciona mais.

`cargo run --release -p solidcraft-sketch --example solve_bench`, macOS arm64, carga média de
45–75 (melhor de 5):

| Grade (canto de origem fixo, linhas horizontais, colunas verticais, cada aresta com uma distância) | Início | Antes | Depois |
|---|---|---:|---:|
| 10 × 10 | na solução | 0.36 ms | 0.21 ms |
| 10 × 10 | ±1 mm | 0.34 ms | 0.21 ms |
| 15 × 15 | na solução | 1.41 ms | 0.41 ms |
| 15 × 15 | ±1 mm | 1.39 ms | 0.40 ms |
| 20 × 20 | na solução | 4.06 ms | 0.72 ms |
| 20 × 20 | ±1 mm | 4.44 ms | 0.73 ms |

A solução inicial constrói todos os pontos da grade, portanto LM não recebe variáveis (um teste as conta). Antes, o
tempo era a análise densa de posto da grade inteira. Uma faixa de 100 retângulos sem nada fixado,
resolvida inteira a partir de um canto movido, ainda custa 20–27 ms: nada é construído ali, e LM
itera sobre cada ponto com equações normais densas (uma solução esparsa seria o próximo passo).

Quadros de arraste, `sketch.drag` + `world_state()` por quadro (60 quadros de 0.5 mm, melhor de 5; a
coluna "antes" é uma execução do mesmo benchmark no código anterior):

| Esboço | Pontos | ms/quadro por script antes → depois | ms/quadro ao vivo antes → depois | `drag_end` ms antes → depois |
|---|---:|---:|---:|---:|
| 100 retângulos | 401 | 2.77 → 2.37 | 1.13 → 1.02 | 1.44 → 1.37 |
| 100 retângulos, retrocedidos sob uma extrusão | 401 | 2.49 → 2.33 | 1.13 → 0.97 | 1.42 → 1.28 |
| faixa conectada de 100 retângulos | 401 | 100.9 → 2.32 | 1.32 → 1.20 | 53.6 → 1.41 |
| faixa de 100 retângulos, retrocedida sob uma extrusão | 401 | 203.8 → 2.23 | 1.31 → 1.18 | 54.0 → 1.42 |
| treliça rígida de 40 triângulos, fixada em uma extremidade e arrastada na outra | 42 | — → 6.8 | — → 6.3 | — → 0.59 |

A faixa é livre (nada fixado), portanto suas resoluções completas (quadros por script, `drag_end`) executavam a
análise densa de posto em 800 variáveis; com um canto tomado como constante, tudo é
eliminado. A resolução com fixação de um quadro ao vivo já era barata: a solução inicial posiciona a faixa, e LM
retorna imediatamente. Em um quadro ao vivo da faixa, a solução inicial lê as restrições em cerca de 0.15 ms, portanto um
cache dessa leitura por gesto não compensa sua invalidação (seus sinais dependem das
posições de referência, que mudam a cada passo).

## Custo de salvamento de miniatura portátil (#303)

A geração de miniatura portátil exige `cache: true` explícito. Salvamentos comuns e
downloads do navegador usam `cache: false` por padrão, portanto não têm o trabalho de miniatura
nem os bytes adicionais medidos abaixo.

Medido em 2026-10-13 na mesma máquina Windows e perfil `ci` acima, mas
com **SolidCraft 0.7.0** em ambos os casos: `d3d215b` antes da alteração de miniatura e
revisão de miniatura `e180cdc` depois. Essas medições são anteriores à integração
do upstream `9f88537`; alterações posteriores de geometria não estão incluídas nesses tempos.
Um executável preservado da referência
e o novo executável rodaram o `benchmark_native_cache_save` existente nos mesmos
documentos STEP e de sensor. Três lotes pareados alternaram a ordem antes/depois;
cada lote fez nove salvamentos atômicos alternados por codificação. Os valores abaixo
são medianas das três medianas de lotes, não um percentil de todos os salvamentos individuais.
Nenhuma compilação, suíte de testes ou modelo de desktop estava em execução durante essas medições.

| Codificação STEP de 512 cubos | Antes do PNG (ms) | Depois do PNG (ms) | Bytes do contêiner antes → depois |
|---|---:|---:|---:|
| JSON simples | 10.334 | 9.676 | 1,998,826 → 1,998,826 |
| Contêiner, `cache: false` | 8.850 | 9.013 | 1,950,084 → 1,950,084 |
| Cache nativo, rápido | 45.323 | 53.473 | 4,694,035 → 4,720,385 |
| Cache nativo, compacto | 42.739 | 48.643 | 837,067 → 863,417 |

O próprio PNG tem 25,916 bytes; sua chave e os registros ZIP aumentam o acréscimo para
26,350 bytes. Medianas de lotes de cache rápido variaram de 41.585–47.779 ms antes e
53.212–56.556 ms depois. O compacto variou de 36.672–43.040 ms antes e
47.688–51.724 ms depois. O aumento de aproximadamente 8 ms no salvamento rápido é um custo
medido nesse exemplo, não um limite geral de latência. Bytes JSON e bytes de blobs de
documento/origem foram comparados entre as duas implementações e permaneceram idênticos.

A caixa de sensor deliberadamente recebe **nenhum PNG portátil**: suas caixas projetadas
de triângulos somam 4,293,457 pixels superamostrados, acima do limite de trabalho de quatro milhões.
O contêiner nativo rápido permanece com 379,099 bytes. Suas medianas de lotes de salvamento
são 14.714 ms antes (12.187–14.956) e 13.728 ms depois (13.439–14.648); elas
variam dentro das faixas observadas de execuções e não estabelecem uma aceleração. Início mantém
sua alternativa existente de geração de miniaturas para esse caso. Uma pequena contagem de triângulos
por si só não garante elegibilidade, pois faces grandes sobrepostas custam mais.

Verificação real no desktop: copie o contêiner STEP para um novo caminho público temporário,
inicie SolidCraft 0.7.0 com esse caminho em Recentes e um cache local vazio de miniaturas,
e mostre Início sem abrir o projeto. A imagem portátil ficou pronta em uma
verificação observada de 47.6 ms; nenhum PNG local foi criado, e o documento ativo permaneceu
vazio. A captura de tela mostra a imagem do arquivo transferido enquanto imagens de exemplos incluídos
ainda carregam. É uma verificação visual funcional única, não um benchmark do primeiro
quadro. A regressão separada de leitor com contagem comprova o adiamento de blobs de origem.
