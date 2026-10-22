# Contêineres de projeto, versão 1

Novos projetos usam `.solidcraft`; `.solvecraft` continua como extensão legada suportada. A abertura detecta a assinatura do
cabeçalho local ZIP (`50 4b 03 04`) independentemente do nome; caso contrário, usa
o leitor existente de projetos JSON e as migrações. JSON simples continua sendo um formato de entrada/saída
autossuficiente totalmente suportado. `io::write_design` mantém seu contrato JSON;
`io::write_design_container` é o gravador de contêiner que pode retornar erros.

Os pacotes têm `container.json` com `{"format":"solidcraft-container/1"}` e
`document.json` com o documento paramétrico versionado (`solidcraft/N`, o formato definido em `crates/doc/src/format.rs`).
As versões independentes permitem que as migrações de documentos evoluam sem alterar as codificações de blobs.
Campos existentes do documento serializado são mantidos automaticamente, sem uma segunda cópia
do esquema do documento ou uma clonagem profunda dos dados incorporados.

Dentro de `document.json`:

- Operações de importação têm `step` vazio e uma referência `step_blob`.
- Importações de malha mantêm o nome e a cor opcional de cada malha, com `positions`
  e `triangles` vazios, além de uma referência `mesh_blob`.
- Imagens de referência mantêm posicionamento, formato e metadados de pixels, com `data` vazio e uma
  referência `image_blob`. Bytes brutos de imagem substituem base64 no pacote.

Referências são `blobs/<lowercase SHA-256>.<step|mesh|png|jpeg>`, com hash calculado sobre os dados
exatos não comprimidos. Conteúdo idêntico com a mesma codificação é gravado uma vez.
Partes obrigatórias, referências malformadas, dados incorporados ambíguos, hashes incorretos,
dados inválidos de malha e versões não suportadas produzem erros antes da adoção pela sessão.

## Codificação de malha

Blobs de malha começam com os oito bytes ASCII `SCMESH01`, seguidos da contagem de coordenadas
`u64` e da contagem de índices `u64`, em little-endian. Elas contam valores escalares, não vértices
ou triângulos. Em seguida vêm essa quantidade de coordenadas IEEE-754 `f64`, depois índices `u32`.
Ambas as contagens devem ser divisíveis por três; o comprimento em bytes deve corresponder exatamente. Coordenadas
devem ser finitas e ter magnitude de no máximo 1e9 mm; índices devem apontar para um vértice existente.
Nomes e cores permanecem em JSON. Zero com sinal e a precisão completa das coordenadas são preservados.

A política padrão `fast` armazena STEP e malhas binárias sem Deflate para salvar
e ler mais rápido. Pequenos metadados de documento usam Deflate de nível um; PNG/JPEG são
armazenados diretamente. Uma política explícita `compact` comprime dados STEP e de malha no
nível um. Os leitores suportam ambas as políticas. Isso prioriza deliberadamente operações interativas de
arquivos em relação à compressão máxima; STEP comprimido foi mais lento no benchmark de armazenamento.
Use `"compression":"compact"` com `file.save` ou `file.save_as` para priorizar o tamanho do arquivo,
ou `io::write_design_container_with_compression` a partir de Rust.

## Limites e recuperação

A entrada e cada parte expandida são limitadas a 256 MiB. O total declarado de partes
expandidas e os dados resolvidos são limitados, cada um, a 512 MiB, incluindo referências repetidas
no segundo limite. Há no máximo 10.000 entradas. Nomes de entradas duplicados/ambíguos,
travessia de caminhos, ZIP64 e pacotes multidisco são rejeitados. As entradas são lidas em memória;
nenhum caminho do arquivo compactado é extraído para o sistema de arquivos. Comprimentos exatos de partes e CRCs ZIP são
verificados, junto com hashes de blobs. Hashes detectam correspondência e corrupção, não confiabilidade.

O salvamento mantém o protocolo existente de arquivo temporário/flush/renomeação e o backup `.bak`.
A recuperação lê tanto JSON antigo quanto novos contêineres. OPFS do navegador, downloads e recuperação
usam o mesmo leitor/gravador de contêiner; exportação JSON e Salvar como continuam disponíveis.

## Exibição provisória salva

Salvamentos experimentais do mecanismo com `preview: true` podem incluir `cache/preview.json` e
`cache/preview.bin` quando as malhas avaliadas
de exibição global já estão em cache. Salvamentos comuns omitem esses dados redundantes de exibição
para evitar trabalho extra ao salvar; eles são separados do cache B-rep nativo abaixo. Salvar nunca triangula um corpo ainda não calculado para esse
cache. Movimentos pendentes de ocorrências, alterações ativas de esboço, erros de operações e prévias
grandes demais fazem com que ele seja omitido. O cache é limitado a 8 MiB, 1.024 corpos e 100.000 triângulos;
seu manifesto tem no máximo 64 KiB. Nomes/cores de corpos e fatias binárias contíguas de malha
usam a mesma codificação sem perda de malhas importadas. O tamanho extra do cache é opcional;
ele é omitido se deixar o contêiner perto do limite de tamanho de arquivo.

O manifesto inclui SHA-256 dos metadados exatos do documento (cujas referências de blobs
contêm hashes de conteúdo), os dados de prévia, `solidcraft-preview/1` e a versão
do pacote do aplicativo. Uma versão incompatível invalida o cache de exibição. Compilações de desenvolvimento
com a mesma versão de pacote podem mostrar brevemente uma imagem antiga antes de a reconstrução
autoritativa substituí-la; prévias nunca fornecem estado do modelo. Contêineres de rascunhos anteriores usando
`solvecraft-container/1` continuam legíveis, com suas prévias obsoletas tratadas como ausências de cache.
Dados de prévia ausentes, obsoletos, grandes demais ou malformados são uma ausência de cache, não uma falha
ao abrir o projeto. Todos os limites de segurança do arquivo compactado inteiro continuam se aplicando.

Abrir no desktop pode renderizar essas malhas em seu worker de arquivos existente e mostrar uma imagem salva
claramente identificada no diálogo de abertura antes de a reconstrução da linha do tempo terminar.
Usa apenas o instantâneo de arquivo do comando normal, sem uma segunda leitura. A imagem
é um sombreamento aproximado sem pintura de faces nem seleção. Um limite conservador de rasterização de 16 milhões
de pixels rejeita desenho excessivo antes da renderização, incluindo triângulos grandes repetidos
em um pequeno cache hostil. A imagem nunca é adotada como geometria
editável, usada para medidas ou usada para inicializar o estado avaliado do modelo. O projeto
original permanece intacto até que a sessão completamente reconstruída esteja pronta. Cancelar ou
qualquer mudança de documento ativo/revisão descarta tanto a prévia quanto a conclusão.

A leitura normal de projetos, a CLI e a abertura no navegador ainda reconstroem de forma autoritativa.
Caches B-rep nativos podem fornecer exibição provisória como descrito abaixo; nunca
fornecem estado avaliado da sessão. A imagem salva não é uma vista interativa.

## Leitor com acesso por posição (fase 2, primeiro incremento)

`io::DesignContainerReader<R: Read + Seek>` aceita uma fonte de contêiner, como um
arquivo ou cursor de memória. Ele inspeciona um sufixo limitado do diretório ZIP e os cabeçalhos
das entradas; depois, `preview()` lê as entradas opcionais de exibição e os metadados do documento.
`read_document()` consome o leitor e resolve e valida os blobs obrigatórios
de origem pelo mesmo arquivo compactado. Nenhum buffer do arquivo inteiro é necessário.
Os leitores existentes baseados em bytes também usam essa implementação; JSON mantém seu leitor
existente. `read_design_file` a integra a Abrir no desktop usando um único identificador de arquivo,
sem manter o contêiner completo em buffer. O VFS do navegador usa um cursor de memória.

Metadados usados para validar um cache provisório são mantidos em um buffer imutável
e reutilizados para a reconstrução autoritativa. Gravações simultâneas no mesmo arquivo, portanto,
mantêm esse documento ou falham na validação de CRC/hash de partes obrigatórias. O sucesso de uma prévia
opcional nunca valida blobs de origem nem torna um documento editável. Um blob obrigatório inválido ainda
faz falhar a leitura posterior do documento. Varreduras do diretório ZIP podem acessar um sufixo limitado de um
blob armazenado; adiar significa não carregar ou decodificar os dados completos, não zero
leituras físicas de sua faixa de bytes. Metadados opcionais de cache são limitados a 16 MiB;
metadados maiores não usam o cache e seguem o limite normal autoritativo de tamanho de partes.

## Cache nativo avaliado

Salvar/Salvar como do mecanismo podem incluir `cache/key.json` e `cache/brep.bin` quando
solicitado explicitamente com `cache: true` e a geometria já está calculada. O padrão é
`cache: false`, incluindo downloads do navegador, para evitar tempo extra ao salvar e tamanho extra de arquivo.
JSON e recuperação/salvamento automático
continuam independentes do cache nativo. Salvar nunca força posicionamento,
avaliação ou tesselação ainda não calculados. Esboços ativos, movimentos não registrados, erros, corpos de malha,
representações não suportadas pelo adaptador e limites excedidos fazem com que o cache seja omitido.

O manifesto registra `solidcraft-brep-cache/1`, a versão do pacote, a revisão explícita
do produtor `solidcraft-truck-adapter/2`, hashes SHA-256 exatos de documento/dados
e fatias contíguas de corpos nomeados. A revisão do produtor é mantida deliberadamente
quando a semântica do adaptador muda; não é uma impressão digital da árvore de código nem um script de compilação.
Os dados binários usam [o esquema e o codec B-rep portáteis do SolidCraft](native-brep.md),
incluindo topologia, curvas/superfícies, certificados de origem de faces, aparências e
histórico de operações. Estruturas serializadas do truck nunca são armazenadas. Um núcleo
diferente pode implementar o mesmo esquema; até que seu adaptador seja testado, ele conservadoramente
ignora o cache desse produtor e reconstrói o documento autoritativo.

Caches nativos limitam 4.096 corpos, 100.000 faces totais, manifesto de 1 MiB e dados
binários de 64 MiB. Corpos individuais têm os limites adicionais do esquema/codec.
O leitor valida metadados, manifesto, fatias e corpos antes de solicitar uma
malha provisória limitada. Blobs originais STEP/imagem/malha são resolvidos depois.
Nenhuma malha redundante de exibição é armazenada por padrão: `preview: true` continua sendo uma opção
separada para malhas de exibição já calculadas, que têm prioridade quando presentes.

Toda falha no cache opcional é uma ausência de cache. Mesmo um corpo forjado com soma de verificação válida
nunca se torna estado editável: Abrir cria uma nova sessão a partir do documento e dos
blobs originais e então substitui a imagem provisória. Seleção, medidas,
nomes persistentes e modelagem posterior usam essa sessão reconstruída. As proteções existentes
de cancelamento/revisão de documento também se aplicam. A compilação web mantém a reconstrução
autoritativa síncrona; portabilidade não implica avaliação web em segundo plano.

Uma regressão de leitor com contagem usa um blob de origem armazenado de 2,6 MB e obtém a prévia
salva lendo menos de 256 KiB, depois verifica que o documento autoritativo é
idêntico após resolver sua origem. Regressões de origem corrompida e ausência de prévia,
além de truncamento/mutações com sementes pelo leitor combinado, protegem a fronteira.
Trata-se de um teste de volume de entrada/saída, não de um benchmark de geometria ou primeiro quadro.

## Incrementos de revisão e independência do núcleo

A implementação é dividida nos seguintes incrementos de revisão após a
PR de contêiner da fase um (#327):

1. Entrada/saída com acesso por posição e seus testes de volume de leitura/entrada hostil (a API acima).
2. Um esquema B-rep versionado próprio do SolidCraft e codec binário limitado, testados sem
   construir um corpo do núcleo. Referências de topologia, usos orientados de arestas e laços
   de faces, domínios geométricos, definições de curvas e superfícies analíticas/spline, unidades,
   tolerâncias e orientação devem ter semântica explícita. Preserve certificados de origem
   de faces, nomes persistentes e histórico usando identidades próprias do SolidCraft.
   Nenhuma estrutura serializada do truck, layout de memória, ponteiro ou blob opaco de núcleo
   faz parte do contrato persistido. Nenhuma ida e volta STEP avaliada o substitui.
3. Adaptadores separados entre o esquema neutro e os núcleos suportados. Geometria
   ou origem não suportada é uma ausência de cache. Splines de interseção ajustadas explicitamente
   carregam seu alvo de ajuste e permanecem geometria provisória de exibição. Um esquema neutro
   de arquivo por si só não prova interoperabilidade: cada adaptador precisa de testes de ida e volta,
   modelagem e nomeação; a compatibilidade com um segundo núcleo não é verificada
   até que esse adaptador exista. Informações de núcleo/versão pertencem a metadados de produtor e
   capacidades, não à codificação de topologia. A invalidação conservadora de cache
   é permitida entre núcleos, enquanto a reabertura autoritativa do documento continua funcionando.
4. Integre leitura adiada da origem e exibição B-rep provisória ao worker de
   abertura com proteções de entrada imutável e documento/revisão. Reconstrua de forma autoritativa
   antes de editar, selecionar ou medir. B-rep sozinho não é uma sessão avaliada
   completa: esboços, construção, posicionamentos e outros estados não sólidos ainda importam.
5. Meça primeira exibição sem cache, tempo até poder editar, latência de salvamento, tamanho e pico de memória
   em um projeto público baseado em STEP. Inclua acertos/ausências de cache, JSON simples e alternativa
   do navegador e informe referências de desempenho e limitações em `docs/perf.md`.

O salvamento padrão deve continuar rápido: não force avaliação ou geração de malha sem cache apenas para
produzir um cache descartável. Não adicione malhas redundantes de exibição por padrão; a
prévia existente continua opcional. Tesselação por GPU é um trabalho independente. Continue
usando constantes de formato/aplicativo em vez de scripts de compilação com hash da árvore de código,
mantenha a compatibilidade JSON de Salvar como/MCP/scripts e as camadas legadas documentadas e
execute a validação wasm independente em cada revisão publicada do código.

## Miniatura portátil

Um salvamento com dados já calculados e `cache: true` pode adicionar `cache/thumbnail.png` e sua pequena chave
`cache/thumbnail.json`. A imagem é um PNG de 240 x 150, limitado a 256 KiB,
identificado por `solidcraft-thumbnail/1`, versão do aplicativo, SHA-256 exato do documento e
SHA-256 do PNG. Contém uma vista isométrica de corpos sólidos visíveis com suas
aparências. É apenas para exibição e independente do núcleo de geometria produtor.

Salvar usa apenas corpos já posicionados e malhas em cache, no máximo 1.024 corpos,
100.000 triângulos e 100.000 vértices. Uma verificação prévia também limita o trabalho das caixas de pixels
projetadas a quatro milhões de pixels superamostrados e as buscas de cores de faces a um milhão,
para que triângulos sobrepostos não tornem uma imagem pequena arbitrariamente custosa.
Nunca avalia ou tessela um modelo
ainda não calculado para produzir uma imagem. Alterações ativas, movimentos pendentes, erros do modelo, ausência de
geometria já calculada ou limites excedidos fazem com que a imagem seja omitida. Salvamentos comuns e downloads do navegador
usam `cache: false` por padrão e omitem o PNG; `cache: true` ativa a opção explicitamente. JSON simples
nunca contém o PNG. O mesmo gravador é usado pelos salvamentos do desktop e navegador.

O leitor de miniaturas com acesso por posição verifica a pequena chave, os metadados mantidos do documento,
tamanho/cabeçalho do PNG e hashes sem resolver blobs de origem. O leitor de Início/Arquivos recentes
tenta seu cache local de imagens, depois essa imagem portátil e então sua reconstrução existente
em segundo plano. A decodificação PNG também tem limites de dimensões e alocação.
Imagens ausentes, incompatíveis, truncadas ou malformadas são ausências de cache e nunca impedem
a abertura autoritativa do documento. Incorporar um PNG não instala um provedor de miniaturas
no explorador de arquivos do sistema operacional.

Leituras de miniaturas no desktop buscam posições dentro do arquivo. O VFS do navegador fornece, em vez disso, um buffer
limitado em memória; evita reconstruir geometria, mas não oferece
leituras OPFS por faixas. A camada de entrada/saída verifica o cabeçalho PNG e os hashes, enquanto a interface
consumidora executa a decodificação PNG completa e limitada.
