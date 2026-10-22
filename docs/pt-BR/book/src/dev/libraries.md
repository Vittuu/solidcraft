# Bibliotecas extensíveis: materiais, aparências e componentes

O primeiro [consumidor de bibliotecas escalares locais](local-libraries.md) implementa apenas
definições copiadas de cor/opacidade e densidade/aparência. Seu catálogo v1 separado não é o
sistema completo de pacotes de recursos/proveniência/atualização proposto abaixo. Descritores
de textura/ambiente continuam coordenados com o trabalho de renderização do mantenedor.

Proposta para [#785](https://github.com/bherbruck/solidcraft/issues/785),
[#786](https://github.com/bherbruck/solidcraft/issues/786) e
[#787](https://github.com/bherbruck/solidcraft/issues/787). Esta página descreve contratos
planejados e critérios de aceitação, não um formato de pacote implementado nem novos comandos. Ela
não inclui no escopo os espaços de trabalho adiados de Simulação ou Renderização.

## Reutilizar os caminhos existentes

Materiais físicos já têm densidades integradas e definições locais ao projeto
(`material.create`, `material.edit`, `material.delete`, `material.list`, `material.assign`).
Eles alimentam `component.properties`, permanecem ao reabrir e participam das configurações.
Aparências têm uma biblioteca integrada de cor/opacidade e substituições por corpo, face e componente.
`parts.library`, `parts.insert` e `parts.resize` já geram sete famílias de
fixadores, pinos e volumes envolventes de rolamentos como componentes editáveis.

Amplie esses caminhos. Um catálogo compartilhado fornece identidade, descoberta e proveniência;
cada tipo de entrada mantém seu próprio conteúdo e regras de aplicação. Um material físico
não é uma textura, e um componente reutilizável não é código executável de plugin.

## Pacotes portáteis e extensíveis de forma independente

Use um único envelope de pacote versionado e offline para três tipos: material físico, aparência
e família de componentes. Colaboradores do projeto e usuários podem criar pacotes sem
recompilar o SolidCraft. Comece com um diretório e um manifesto JSON, não com um novo formato de arquivo compactado
ou uma loja online. O esquema exato pertence à PR do carregador com limites, acompanhado
de exemplos verificados automaticamente; os requisitos de projeto são os seguintes:

| Registro | Significado obrigatório |
|---|---|
| Pacote | Versão do esquema, ID estável com namespace, revisão imutável, nome de exibição, autor, fonte e metadados de licença |
| Entrada | ID com escopo no pacote, revisão imutável, tipo, rótulo, categoria/etiquetas, conteúdo tipado e proveniência por entrada |
| Recurso | Caminho relativo, resumo criptográfico do conteúdo, contagem de bytes, papel da mídia, autor, fonte, licença e texto da licença incluído quando exigido |
| Dependência | Referência explícita a entrada/revisão; resolução acíclica limitada dentro de pacotes explicitamente selecionados |
| Seleção | ID do pacote + ID da entrada + revisão exata; rótulos e caminhos do sistema de arquivos não são identidades |

Revisões de pacote e versões de esquema são diferentes: as primeiras identificam o conteúdo,
as últimas descrevem como lê-lo. Alterar o conteúdo sob uma revisão existente é um erro.
Versões de esquema, tipos de entrada e capacidades obrigatórias desconhecidos produzem uma recusa clara;
nunca são interpretados silenciosamente como um formato antigo. Extensões opcionais precisam de um
namespace explícito e regras de preservação antes de serem aceitas.

Forneça três escopos independentes:

- **Integrado:** revisado com o SolidCraft e somente leitura no editor de bibliotecas.
- **Projeto:** pacotes abertos explicitamente para o projeto/equipe atual; nunca descobertos
  por execução silenciosa ou confiança automática em arquivos ao lado de um projeto.
- **Usuário:** pacotes locais que o usuário cria, importa, edita ou exporta. Acesso nativo a arquivos
  pertence ao aplicativo; usuários de navegador importam/exportam pelo fluxo de armazenamento/arquivos da web.

A ordem de descoberta é determinística. Rótulos duplicados podem coexistir, mostrando sua fonte.
Identidades/revisões qualificadas duplicadas com conteúdos diferentes são erros, não substituições
escolhidas pela ordem de varredura. Editar uma entrada integrada usa **Salvar uma cópia** em um namespace de usuário.
Atualizar um pacote é explícito e deixa inalteradas as instâncias existentes no projeto.

## Os projetos são donos das definições selecionadas

Ao atribuir/inserir, copie a definição, a revisão escolhida, os parâmetros, a proveniência e os
bytes dos recursos necessários para o mecanismo existente de contêiner/recursos do projeto. Mantenha a
identidade da fonte para inspeção e uma eventual ação explícita de Atualizar. Um projeto salvo deve
abrir, reconstruir e renderizar a aparência suportada sem o pacote original, a rede ou
o diretório pessoal de outro usuário. Instalar um pacote nunca altera um projeto nem o histórico de desfazer.

Atribuição/inserção/atualização é um comando normal do mecanismo, atômico e que pode ser desfeito. A prévia
não altera o estado. Uma importação com falha, dependência não resolvida, colisão de nomes ou receita não suportada
deixa intactos o documento e as pilhas de desfazer/refazer. Nomes de materiais e precedência de aparências
existentes mantêm seu significado legado; introduza IDs estáveis por uma migração documentada,
em vez de reinterpretar atribuições antigas baseadas em nomes. Mudanças no formato de projeto precisam de sua própria
versão/migração e testes de reabertura, separados da versão de esquema do pacote.

## Propriedades físicas (#785)

Mantenha independentes a seleção de material físico e de aparência, com um visual padrão opcional.
Converta explicitamente unidades de densidade importadas para a representação existente em g/cm³; a geometria
continua usando milímetros e radianos. Nunca infira unidades pela magnitude. Propriedades
precisam de fonte, edição/data, classe, condição de processamento e temperatura/umidade aplicável
quando relevante. Propriedades desconhecidas permanecem desconhecidas, não zero nem valores presumidos.

Comece com densidade e integração de massa pelos comandos existentes. Reserve extensões
tipadas para constantes elásticas, resistência ao escoamento/ruptura, propriedades térmicas e
eixos ortotrópicos, mas não as apresente como prontas para análise por elementos finitos até que seu modelo, unidades e
validação estejam implementados. Uma densidade genérica de madeira não é uma classe de material de engenharia.
Dados medidos pelo usuário podem ser armazenados com sua própria proveniência e editados sem alterar a
biblioteca integrada.

## Aparências de madeira e painéis (#786)

Comece com mapeamento local ao corpo, uma dimensão física explícita de repetição e um sistema de coordenadas do veio
editável pelo usuário. Sugira uma vez o eixo mais longo dos limites **locais** do corpo, usando a ordem X/Y/Z para extensões
iguais; armazene o sistema de coordenadas escolhido em vez de recalculá-lo a cada redimensionamento. Rotação
e escala manuais permanecem após reconstruções por parâmetros, posicionamento de instâncias, desfazer e reabertura. Instâncias
compartilhadas de componentes transformam o mesmo mapeamento local à definição com seu posicionamento.
Sistemas de coordenadas não finitos ou degenerados são recusados; a escolha automática continua sendo uma sugestão.

O veio visual não rotaciona o corpo, sua caixa delimitadora geométrica nem um sistema de coordenadas do
material bruto de fabricação. A direção estrutural do veio e o encaixe/listas de corte que consideram o veio são operações
distintas, coordenadas com [#137](https://github.com/bherbruck/solidcraft/issues/137).
Um efeito de sombreamento anisotrópico também não implica anisotropia mecânica.

Após o mapeamento básico de madeira, adicione aparências específicas de topo/base/borda para painéis: MDF, aglomerado,
compensado e laminado compacto; depois, acabamentos de melamina, laminado, linóleo e lâmina de madeira.
Camadas de aparência descrevem inicialmente apenas a apresentação. Espessura física ou
massa de acabamento devem ser entradas explícitas de modelagem/material, nunca um efeito invisível da pintura.
Atribuições de face/aresta exigem identidades duráveis e devem recusar uma associação perdida,
em vez de pintar uma face substituta próxima.

Mantenha explícitos o papel da textura, o espaço de cor, a convenção do mapa de normais e a escala física.
Coordene com o [contrato de capacidades de renderização](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/render-backends.md):
a área de visualização existente de cor/opacidade não comprova suporte a PBR ou texturas. Um renderizador
deve informar o que suporta; mapas indisponíveis usam uma alternativa documentada de prévia com
um aviso visível. A exportação preserva mapeamento/recursos suportados ou informa a perda.

## Famílias de componentes padrão e do usuário (#787)

Mantenha o caminho existente de inserção/redimensionamento de fixadores. Adicione primeiro receitas originais parametrizadas de BAR, SHS,
RHS e CHS, com unidades explícitas de comprimento, limites de parâmetros, sistemas de coordenadas de seção
e de inserção/conexão. Valide a espessura da parede em relação à dimensão externa e
informe se os cantos são vivos ou arredondados. Não apresente geometria aproximada como
perfil normativo certificado. UB/UC precisam dos raios e inclinações reais de mesa/alma; identificar uma
norma não concede direitos de redistribuição de suas tabelas publicadas de dimensões.

Uma revisão de família define um esquema limitado de parâmetros, uma receita permitida e uma tabela opcional
de dimensões revisada. Uma designação de tamanho identifica uma linha, não um nome de arquivo arbitrário.
Alterar o tamanho reconstrói a mesma identidade de componente e preserva conexões suportadas.
Receitas iniciais devem usar uma pequena lista permitida de operações comuns de modelagem. Nenhum
código nativo embutido, comando de shell, script, ID arbitrário de comando ou chamada de rede. Rejeite
ciclos, expansão excessiva e operações não suportadas antes de alterar o projeto.

Montagens personalizadas vêm depois dos componentes individuais: dependências limitadas, sistemas de coordenadas de conexão
nomeados, semântica explícita de instâncias compartilhadas/independentes e inserção desfeita em uma etapa.
Armazene os parâmetros selecionados e cópias das definições para que catálogos ausentes ou alterados não possam
substituir silenciosamente uma peça. Modelos de volume envolvente geométrico, incluindo os volumes envolventes
de rolamentos existentes, devem permanecer distinguíveis de modelos detalhados/prontos para fabricação.

## Fontes e licenciamento

O SolidCraft é MIT OR Apache-2.0. Código e receitas geradas devem seguir as regras de implementação
independente; nunca copie materiais, modelos, geometrias, ícones ou catálogos de CAD comercial.
Recursos de terceiros mantêm suas próprias licenças. Cada recurso integrado precisa de autor,
fonte, licença e uma linha em `ATTRIBUTION.md`, mesmo quando a atribuição é opcional sob CC0.
Registre hashes e datas de obtenção ao admitir arquivos reais. Uma licença declarada em um manifesto
é proveniência a revisar, não prova automática de permissão.

Fontes verificadas em 2026-10-13; esta proposta não inclui recursos ou conjuntos de dados externos:

| Fonte | Uso adequado / condição de admissão |
|---|---|
| [Licença do ambientCG](https://docs.ambientcg.com/license/) | Recursos para download e renderizações de prévia de materiais são CC0. Fonte candidata de texturas de madeira/painéis; escolha e atribua arquivos individuais. |
| [Licença do Poly Haven](https://polyhaven.com/license) | Recursos são CC0. Textos do site, logotipos e outros conteúdos têm direitos separados; o acesso à API tem termos próprios. Não copie código de implementação do fornecedor. |
| [Licença de dados estruturados do Wikidata](https://www.wikidata.org/wiki/Wikidata:Copyright) | Dados estruturados de entidades/propriedades são CC0; outros textos das páginas têm termos diferentes. Candidato para grandezas e identificadores individuais referenciados, não para valores padrão de engenharia sem verificação. Preserve as referências de medição originais, qualificadores e unidades. |
| [USDA Wood Handbook, revisão 2021](https://research.fs.usda.gov/fpl/wood-handbook) | Referência para umidade e propriedades físicas e mecânicas da madeira. Verifique direitos, espécie, unidades e condições da tabela específica antes de importar dados; isso não implica aprovação de um conjunto de dados inteiro. |
| [Direitos de dados/publicações do NIST](https://www.nist.gov/open/copyright-fair-use-and-licensing-statements-srd-data-software-and-technical-series-publications) e [termos do repositório](https://materialsdata.nist.gov/terms/) | Apenas referências candidatas de propriedades. Dados enviados usam termos escolhidos pelo fornecedor; disponibilidade pública não é uma licença geral de redistribuição. Revise cada conjunto de dados. |
| [Visão geral de PBR do Khronos](https://www.khronos.org/gltf/pbr) | Convenções públicas de renderização para papéis de texturas e propriedades de sombreamento; referência de projeto, não fonte de dados de materiais CAD. |
| Receitas originais do SolidCraft / medições do usuário | Receitas iniciais preferidas de componentes e prévias procedurais ilustrativas; preserve a autoria e identifique valores ilustrativos versus medidos. |

Prefira recursos CC0 para o pacote inicial de texturas e receitas originais para o pacote inicial
de componentes. Adie dados incertos ou restritivos, em vez de extrair sites de fornecedores,
copiar publicações de normas ou presumir que um arquivo CAD disponível para download possa ser redistribuído.

## Fases e critérios de aceitação

Cada PR de implementação referencia essas issues sem encerrá-las prematuramente.

| Fase | Entrega | Evidência exigida antes de considerá-la concluída |
|---|---|---|
| 0 | Esta proposta, revisão de fontes e roteiro | Contrato que o mantenedor possa revisar; distinguir funções existentes das planejadas; nenhuma alteração de execução ou formato. |
| 1 | Interpretador compartilhado de manifesto com limites, resolvedor e validação/importação/exportação de pacotes | Manifestos de projeto e usuário usam o mesmo esquema; conflitos determinísticos, versões/IDs inválidos, recursos/dependências ausentes, ciclos, travessia de caminhos e esgotamento de limites falham de forma atômica; nativo + wasm passam. |
| 2 | Integração do catálogo de materiais e aparências e interface no navegador | Criar/copiar/editar/importar/exportar pela interface; usar comandos existentes; massa analítica de uma caixa antes/depois da atribuição; configurações, desfazer e reabertura offline sem cache; capturas reais de tela. |
| 3 | Mapeamento de madeira e um pequeno pacote inicial revisado | Rotacionar/escalar o veio em um bloco seguro para divulgação e uma instância posicionada; sem inversão ao redimensionar; reabrir sem pacote; testes e capturas de capacidade/alternativa de renderização; atribuição de cada recurso distribuído. |
| 4 | Famílias parametrizadas de perfis e dados normativos de dimensões revisados | BAR: área = largura × altura; RHS/SHS com cantos vivos: área externa menos área interna; CHS: π(diâmetro_externo² − diâmetro_interno²)/4. Verificar volume = área × comprimento, presença da cavidade, sistema de coordenadas de inserção, redimensionamento, conexões, desfazer e reabertura offline; capturas de tela. |
| 5 | Montagens personalizadas, camadas de face/aresta de painéis e catálogos ampliados | Inserção aninhada com limites, semântica compartilhada/independente, associações duráveis de faces, atualizações explícitas e informação de perdas; testes com medições e capturas para cada fluxo. |

Após a fase 1, a integração de materiais/aparências e o trabalho em famílias de perfis podem avançar
independentemente. O mapeamento de madeira depende de suporte real do renderizador, não de famílias de perfis.
A fase 5 consiste em várias PRs separadas, cada uma seguindo seu fluxo anterior relevante, em vez
de uma mudança que acople inserção de montagens à pintura de faces.

Mantenha interpretação/E/S de recursos em `io`, definições selecionadas duráveis em `doc`, comandos de aplicação
e validação em `engine`, interpretação específica de renderizador em `render`/seus
adaptadores e interface de descoberta/edição em `ui-egui`. Camadas inferiores não dependem da interface
nem de diálogos de arquivo da plataforma. Reutilize a eliminação de recursos duplicados do contêiner; evite um segundo armazenamento de blobs.

Antes de alocar/decodificar, imponha limites de bytes/entradas/textos/dependências do manifesto, bytes
totais de recursos, dimensões de imagem/bytes decodificados e expansão de receitas. Caminhos devem ser
relativos, normalizados e confinados ao pacote selecionado, incluindo o tratamento de links simbólicos. A fase 1
deve publicar limites exatos e testar suas fronteiras. Não obtenha automaticamente arquivos ausentes nem
instale extensões executáveis. Importação/exportação explícita basta para a primeira entrega.

Execute `cargo xtask ci` para cada conjunto de implementação e confira os resultados remotos da PR. Testes de
geometria usam valores analíticos; mudanças de interface e de CAD observáveis incluem evidências reais do aplicativo.
As três issues permanecem abertas até que seus fluxos individuais passem por esses critérios.
