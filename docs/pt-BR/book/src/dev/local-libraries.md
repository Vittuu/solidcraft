# Bibliotecas escalares locais

A primeira etapa de bibliotecas locais (#846) reutiliza os comandos existentes de aparência
e material físico. É um pequeno consumidor da [proposta de bibliotecas](libraries.md), não a
implementação completa de pacotes de recursos. Não acrescenta mapeamento de textura, veios de
madeira, shader PBR, iluminação HDRI, descoberta online ou versão do formato de documento.

## Fluxo no desktop

1. Aplique uma aparência nomeada de cor/opacidade por **Aparência**, ou defina um material
   físico com os comandos existentes. Uma aparência não altera a densidade física.
2. Em Aparência → **Biblioteca local**, escolha **Exportar biblioteca do projeto...**. A captura
   contém substituições explícitas de aparência de corpos/faces/componentes e materiais físicos
   locais do projeto, incluindo suas aparências padrão. Não exporta pintura importada do núcleo,
   entradas não atribuídas do catálogo integrado nem um rascunho não aplicado do seletor de cores.
3. Abra a biblioteca em outro projeto, pesquise suas aparências, selecione uma e escolha
   **Aplicar aparência da biblioteca**. Os destinos normais de corpo/face/componente são reutilizados.
   OK após essa aplicação imediata não acrescenta outra etapa de desfazer.
4. Salve o projeto. As definições escalares selecionadas são dados comuns copiados para ele;
   mover ou remover a biblioteca de origem não muda o resultado salvo.

Exportar não altera o projeto, seu estado de alterações não salvas ou histórico. Um arquivo
existente é recusado, a menos que **Substituir arquivo de biblioteca existente** seja selecionado
explicitamente. A exportação é uma captura das definições do projeto, não uma mesclagem automática
com entradas de um catálogo existente. Editar uma aparência/material copiado usa os comandos
normais e não altera a origem.

A integração combinada de renderização usa o seletor assíncrono #1127 integrado à main.
Edições do modelo e mudanças de aba aguardam a resposta enquanto desenho e redimensionamento
continuam. O consentimento para substituir é capturado antes da abertura; a exportação captura
o projeto quando `library.export` executa após escolher o destino. A identidade do documento é
verificada, e uma resposta antiga de biblioteca não pode substituir outro editor de Aparência.
Cancelar não lê nem grava bibliotecas. O candidato anterior #1114 deixou de ser dependência de
execução do consumidor atualizado.

O painel desktop aplica aparências. Importar/atribuir materiais físicos está disponível por
`library.apply`; definições importadas também aparecem na lista existente de materiais.
O navegador pode exportar uma biblioteca pelo serviço existente de download. A abertura
interativa de bibliotecas locais é um recurso desktop nesta etapa. Comandos do mecanismo no
navegador podem consumir um catálogo limitado já fornecido pelo VFS; compilar wasm não comprova
um importador interativo ou fluxo de download no navegador.

## Comandos

Todas as chamadas programáticas usam JSON e nunca abrem um seletor de arquivos:

```json
{"command":"library.export","params":{"path":"my-library.solidcraft-library.json"}}
{"command":"library.inspect","params":{"path":"my-library.solidcraft-library.json"}}
```

Inspecionar/exportar retorna um `catalog` com IDs qualificados de entradas e revisões exatas.
Use os valores retornados ao aplicar; não substitua uma revisão desconhecida por uma suposição:

```json
{"command":"library.apply","params":{
  "path":"my-library.solidcraft-library.json",
  "entry":"user.local/appearance-<returned-id>",
  "revision":"<returned-64-character-revision>",
  "targets":{"bodies":["Body1"]}
}}
```

Os destinos de aparência podem ser `bodies`, `components` ou `faces: [{body, point: [x,y,z]}]`.
Os pontos são milímetros no espaço global. São aceitos no máximo 1.024 destinos no total.
Objetos ausentes e destinos inválidos falham; o importador não inventa uma substituição próxima.

Uma entrada de material pode ser copiada sem destinos ou atribuída a corpos. O `name` opcional
seleciona explicitamente um novo nome de material físico na importação. Uma definição existente
idêntica é reutilizada; uma definição conflitante ou nome integrado é recusado. Nenhuma densidade,
aparência ou célula de configuração é sobrescrita silenciosamente. Criação e atribuição compartilham
uma transação externa de desfazer pelos comandos existentes de material. A massa continua sendo
volume × densidade; uma densidade genérica ilustrativa não representa uma classe de engenharia.

`library.export` também aceita `id` (padrão `user.local`), `name` (padrão `My library`),
`provenance` e `overwrite` (padrão false). Apenas atribuições explícitas e definições locais
são inspecionadas, sem avaliar geometria. Definições exatamente iguais são deduplicadas;
valores diferentes com o mesmo nome de exibição continuam como entradas distintas.

## Catálogo escalar v1

O marcador independente do formato JSON é `solidcraft-library/1`. Ele contém `id`, `name`
e um vetor `entries`. Cada entrada contém `id`, `revision`, `provenance` e uma `definition` tipada:

- `appearance`: `value: {name, color: [r,g,b], opacity}`; bytes sRGB e opacidade finita 0–1.
- `material`: `value: {name, density_g_cm3, appearance?}`; densidade finita acima de 0 e no máximo
  1.000 g/cm³, com uma aparência opcional da mesma forma escalar.

Todo objeto rejeita campos desconhecidos. Tipos não suportados (incluindo conteúdos de
textura/ambiente) e versões falham explicitamente, sem descartar campos nem tratar uma prévia
de cor como material texturizado. IDs de catálogo têm 1–64 bytes ASCII, começam com letra
minúscula e continuam com letras minúsculas, dígitos, `.`, `_` ou `-`. Um ID de entrada é
`<catalog-id>/<local-id>` com as mesmas regras de ID local; caminhos e rótulos não são IDs.

A revisão de uma entrada é o SHA-256 em minúsculas do JSON serde v1 compacto `[provenance, definition]`.
A ordem derivada dos campos da struct, a omissão de opcionais ausentes, o escape UTF-8 e a
formatação numérica f64 com ida e volta fazem parte desse contrato de escrita; não é uma
canonicalização arbitrária de JSON. Prefira `library.export` ou a API Rust `Entry::new`/`content_revision`
ao gerar revisões. Um escritor futuro incompatível muda a versão do catálogo. Os IDs padrão
de exportação contêm um tipo e 16 caracteres hexadecimais da revisão; colisões são rejeitadas,
nunca usadas para sobrescrever outra definição. IDs locais estáveis definidos explicitamente
podem manter sua identidade entre revisões. A aplicação exige a revisão exata selecionada.
Alterar outra entrada não muda a identidade da definição selecionada.

A proveniência contém `license` obrigatório, `author` opcional e `source_url` opcional.
São metadados fornecidos, não permissão verificada ou assinatura do publicador. URLs nunca
são acessadas. O padrão `LicenseRef-User-Provided` não afirma que dados do usuário têm licença
aberta. Nenhuma arte/dado externo é distribuído nesta etapa. Proveniência/revisões ficam no
catálogo: o primeiro consumidor copia valores escalares para registros `Look`/`UserMaterial`
existentes, sem um recibo persistente de origem/atualização. Incorporação completa de recursos,
proveniência do projeto, escopos e vínculos explícitos de atualização continuam propostos.

## Limites e comportamento em falhas

O leitor aceita no máximo 4 MiB de JSON codificado e 1.024 entradas. A decodificação de entradas
para nesse limite com crescimento falível do vetor. Nomes são aparados, não vazios e têm no
máximo 128 bytes UTF-8; os limites de licença/autor/URL são 128/256/2.048 bytes. Caracteres de
controle, identificadores inválidos, identidades duplicadas, revisões incorretas e escalares
não finitos falham. A exportação inspeciona no máximo 100.000 referências de origem e deduplica
em no máximo 1.024 entradas. São limites de codificação/trabalho, não garantia de memória ou latência.

Leituras nativas são limitadas a arquivos regulares; no navegador, verificam o tamanho no VFS
antes de clonar. Gravações nativas sem substituição reutilizam a publicação atômica de arquivo
novo; substituir explicitamente é atômico, sem ler/copiar um destino antigo arbitrariamente
grande. Todos os caminhos de comandos de biblioteca respeitam o escopo de arquivos MCP/controle
ativo antes de ler ou publicar, incluindo substituição. Publicação no VFS do navegador é síncrona
e reutiliza o serviço de download do host. Não há descoberta automática, busca de recursos,
resolução de recursos relativos, receita executável ou plugin novo.

Uma aplicação malsucedida preserva o documento e o histórico de desfazer/refazer. A origem é
lida e verificada antes da cópia; uma revisão selecionada desatualizada exige recarregar e
selecionar explicitamente. O projeto copiado continua independente após atribuição bem-sucedida,
desfazer/refazer e reabertura a frio. A biblioteca não depende da pilha pendente de renderização
nem altera o formato de documento. Evidências visíveis e resultados nativos/wasm reais ficam na PR;
verificações geométricas/de robustez são separadas e nenhuma referência é enfraquecida.

O consumidor PBR opaco faz a exportação v1 falhar explicitamente quando qualquer aparência
de corpo, componente, face ou padrão de material contém parâmetros PBR. Nem consentimento
de substituição nem arquivo de saída existente permitem uma conversão com perda; use o
transporte mais rico quando disponível. Importações de catálogos escalares continuam
selecionando explicitamente a resposta escalar.
