# Superfícies opacas de produto

Esta é a primeira etapa integrada de consumo de #1004, sob #813/#620. Usa o espaço Render,
a geometria de produto, a atribuição de aparência e o host de trabalhos PNG existentes.
Não há segundo renderizador ou conjunto externo de dados de materiais.

## Aplicar e preservar uma resposta

```json
{"command":"appearance.assign","params":{"bodies":["Specimen"],"color":"#b47846","pbr":{"metallic":1,"roughness":0.3,"dielectric_ior":1.5}}}
```

O editor de Aparência tem uma caixa **PBR opaco** e campos separados de Metalicidade,
Rugosidade e IOR dielétrico. Ativá-la define explicitamente a opacidade como um.
Veja a resposta em Render ou exporte um PNG de produto; Projeto mantém sua prévia simples
de cor. Aplicar/OK, reutilizar amostras e selecionar componentes/faces usam o comando existente
de atribuição. Uma substituição escalar posterior de face substitui a resposta PBR do corpo
naquela face; não herda implicitamente a resposta óptica.

Parâmetros são finitos e limitados: metalicidade [0,1], rugosidade perceptual [0.045,1],
IOR dielétrico relativo [1,3]. Campos ópticos desconhecidos e opacidade não numérica ou
fora da faixa são recusados antes de alterar o projeto. PBR exige opacidade exatamente um.
IOR controla a reflexão da fração não metálica; não habilita refração. Omitir `pbr` ou
fornecer null em uma atribuição seleciona a resposta escalar anterior. A atribuição substitui
uma aparência inteira; não é uma alteração parcial de parâmetros.

`material.create/edit` aceitam o mesmo bloco `pbr` opcional junto à aparência/cor padrão.
Uma edição apenas de densidade mantém toda a aparência existente. Alterações da resposta
visual não modificam densidade, geometria, posicionamento ou eixos de engenharia.
Nenhum valor óptico ou mecânico de uma classe é inferido de nome ou cor.

O formato **21** salva `pbr` nas aparências existentes de corpo/face/componente e padrões
de material. Campos ausentes mantêm o sombreamento escalar. Configurações de renderização do
formato 20 e aparências antigas são atualizadas sem reinterpretar; leitores antigos recusam
arquivos recém-salvos em vez de descartar sua resposta óptica. Desfazer/refazer, copiar e
reabrir a frio usam os valores pertencentes ao projeto, sem depender de arquivos externos.

O catálogo escalar v1 atual não representa PBR. Converter/exportar recusa uma aparência PBR
ou padrão de material físico em vez de descartar seus parâmetros silenciosamente. Amostras
no projeto preservam e reutilizam a resposta. Uma versão mais rica do catálogo é uma etapa separada.

## Consumidores reais e cor

Os hosts integrados instalados `gpu-viewport` e `cpu-product` anunciam o recurso `pbr`.
Solicitações o exigem quando a cena de produto capturada contém PBR. O descritor legado
estático `cpu-raster` mantém suas capacidades anteriores. Nenhum host ganha suporte a HDRI,
refração, renderização progressiva/final ou EXR.

As cores são decodificadas de sRGB8 para sRGB linear uma vez no adaptador compartilhado de
documento/cena. Vértices PBR da GPU carregam refletância linear e parâmetros ópticos. O
rasterizador de CPU interpola posição global e normais com correção de perspectiva; a GPU
usa a interpolação correspondente de fragmentos. Ambos usam a [BRDF opaca](render-surface.md)
original e três luzes direcionais neutras fixas em coordenadas globais Z para cima. Somam
BRDF × radiância incidente relativa × cosseno da luz, aplicam mapeamento tonal Reinhard de
prévia por canal e codificam exatamente uma vez. Attachments GPU lineares e sRGB têm caminhos
distintos de transferência de saída.

Esse estúdio fixo não tem sombras projetadas, iluminação de ambiente, reflexões indiretas,
emissão, anisotropia, revestimento, absorção ou transmissão. As magnitudes de luz são valores
relativos de prévia, não lux calibrados ou dados espectrais. O modelo de espalhamento único
pode escurecer metais rugosos. Aparências legadas transparentes mantêm mistura alfa ordenada
aproximada, inclusive diante de uma superfície PBR opaca; isso não é óptica de vidro.
Controles não padrão de exposição/ambiente continuam explicitamente não suportados.

Uploads de cena mantêm o limite de 128 MiB, contabilizando os vértices PBR maiores.
A admissão do mapa de faces da CPU contabiliza descritores ópticos retidos. As políticas
existentes de captura imutável, propriedade, cancelamento, recusa de resultados antigos e
exportação PNG continuam valendo. Geometria inválida/não finita ou normais nulas são recusadas.

## Validação e exemplo original

`examples/render-opaque-studio.json` constrói quatro esferas originais e uma base, variando
independentemente metalicidade e rugosidade com a mesma cor base. Seus valores são um estudo
óptico ilustrativo, não dados de engenharia de classes reais de material.

O teste de produto fora da tela usa o callback/pipelines realmente instalados em Vulkan,
incluindo attachments lineares e sRGB. Para quatro combinações de metal/rugosidade, cada
comparação verifica 12.454 pixels internos contra a saída PNG da CPU; o erro médio observado
por canal fica abaixo de 0.2/255, sem pixels além do limite de discrepância 12/255.
Silhuetas são excluídas porque imagens CPU usam superamostragem 2x e o alvo GPU do teste usa
1x. É um resultado restrito de paridade, não certificação de navegador ou GPU universal.

Testes de comandos verificam geometria/massa inalteradas, recusa atômica, desfazer/refazer,
reabertura a frio, precedência de padrão físico/face, preservação de aparências escalares
do formato 20 atualizadas para 21 e recusa explícita de catálogo v1. Uma regressão com cliques
no editor verifica uma transação e reabertura do mesmo corpo selecionado. Mídia nativa e
resultados exatos das verificações da revisão atual pertencem à PR. Compilar wasm e interagir
realmente com o navegador são evidências separadas.
