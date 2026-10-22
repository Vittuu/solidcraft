# Espaço Render integrado (R3)

R4a amplia este contrato de vista com um executor CPU de imagens de produto anunciado
separadamente e controles Verificar/Iniciar/Cancelar/Exportar. Veja [trabalhos de renderização](render-jobs.md)
para o protocolo assíncrono, limites de imagem, revisões capturadas e exportação PNG.
O snapshot legado `ui.render` continua sendo um caminho separado.

Escolha **Render** no seletor de espaços após concluir uma operação de modelagem.
O renderizador GPU integrado mostra uma cena de produto separada. Voltar a Projeto restaura
sua câmera, preferência de projeção e política existente de desenho da vista.
Nenhum segundo mecanismo é instalado ou selecionável.

## Cena de produto

`engine::product_scene::extract` consome geometria avaliada definitiva, reutilizando o
posicionamento global/de ocorrências do documento e Arcs de malha. Aparências de corpo,
componente, material, face e importadas preservam precedência e opacidade existentes.
A geometria de origem nunca é transformada novamente nem alterada para servir ao renderizador.

O contrato compartilhado L1 `ProductScene`/`ProductMesh` contém malhas no espaço global,
cores sRGB resolvidas, opacidade e substituições de aparência por face. Índices de faces são
o mapeamento existente entre malha e B-rep, resolvido por referências de aparência do documento;
não são novos nomes persistentes ou identidades de faces derivadas dos triângulos.

A política inicial de produto não inclui linhas de grade/eixos/arestas, esboços, objetos de
construção, cores de seleção ou ponteiro, cortes de seção, análises, esmaecimento do componente
ativo, imagens de referência ou decalques. A visibilidade na árvore do modelo ainda controla
quais corpos aparecem. A vista Projeto e snapshots CPU mantêm suas políticas atuais.

A extração recusa um modelo desatualizado de arraste ao vivo. O comando explícito de leitura
`render.scene {}` segue o limite normal de avaliação do mecanismo e retorna metadados limitados:
nomes de corpos, quantidades de triângulos, cores/opacidade resolvidas, quantidade de
substituições de faces e limites. Não retorna malhas, altera o documento ou acrescenta uma
etapa de desfazer. A extração admite no máximo 16.384 corpos visíveis.

## Configurações e câmera

O painel direito prepara alterações de fundo e câmera de uma configuração nomeada.
**Salvar configuração** registra a captura atual da `Camera` existente por
`render.settings.set`, em uma etapa de desfazer; outros campos R2 mantêm seus valores.
**Recarregar configurações salvas** descarta o rascunho. Cada documento aberto tem seu rascunho.

**Restaurar câmera salva** executa `render.settings.apply {name}`, preservando alvo,
distância, yaw, pitch, roll, FOV e eixo vertical como estado de vista. Não edita o documento
nem estabiliza um modelo desatualizado. A interface respeita o eixo vertical em Render;
a câmera de Projeto é restaurada ao voltar. Arrastar orbita, arrastar com o botão do meio
desloca, rolar aplica zoom e **Ajustar produto** enquadra corpos visíveis sem esboços ou seleção.

Rascunhos de prévia não marcam o documento como alterado até serem salvos. Preferências de
backend desconhecido e modos não suportados continuam como intenção salva: o resolvedor
estrito informa a solicitação não suportada. **Usar prévia integrada** é uma mudança explícita
do rascunho, nunca um fallback automático. Não há seletor de modo final ou segundo mecanismo
nesta etapa. Campos salvos de exposição/ambiente/saída não passam a afetar sombreamento ou exportação.

## Capacidades e recursos do host

`render.backends` sempre mantém o executor integrado de imagem prévia/PNG `cpu-raster`.
Somente um host GPU instalado pela interface acrescenta `gpu-viewport`, anunciando
**Prévia / Vista**, sem formatos de imagem ou recursos HDRI/refração/progressivo. O
[consumidor PBR opaco](render-opaque-pbr.md) instalado acrescenta `pbr`. O host informa
limites efetivos do dispositivo criado e attachment de profundidade: no máximo 4096 pixels
por dimensão e 4.194.304 pixels, dentro da política de admissão de 256 MiB de memória temporária
de pixels. A estimativa inclui o custo MSAA selecionado de cor/profundidade; exclui geometria,
custo do driver e malhas de origem em cache. Não mede a memória GPU total.

Metadados puros de capacidade ficam na sessão/host, fora da persistência e histórico,
e sobrevivem a substituição de documento, sessões temporárias e troca de abas. Nenhum tipo
wgpu entra nas crates de mecanismo ou documento. Um host sem GPU/buffer de profundidade
instalado informa indisponibilidade da prévia de produto, sem usar silenciosamente o snapshot
opaco CPU para aproximar aparências transparentes.

Lotes GPU de produto são separados dos de Projeto, compartilhando dispositivo e pipelines
instalados. Alterações de modelo/visibilidade/aparência atualizam a cena; mover a câmera
atualiza uniforms sem reconstruir vértices. Seleção e seção não alteram buffers de produto.
Uploads de vértices têm limite de 128 MiB, verificado antes da alocação; triângulos inválidos/
não finitos e opacidade inválida falham. Reinstalar o host invalida uploads. Não é um protocolo
geral de perda de dispositivo, trabalhos ou cancelamento; esses mecanismos pertencem a R4.

Transparência usa a aproximação raster existente de mistura alfa distante/próxima, sem
refração ou iluminação HDR. PBR opaco usa um lote separado de superfícies no mesmo renderizador.
R3 em si não acrescentou exportação; R4a fornece trabalhos PNG CPU de produto. Não há galeria
de resultados ou dependência Bevy. Configurações salvas chegaram no formato 21, junto com
parâmetros ópticos opcionais em aparências existentes.
`ui.render` continua sendo a API CPU de snapshot de Projeto; `ui.screenshot` captura o espaço
Render real. Exportar imagens de produto usa `render.start` e `render.export`, em vez da API
de snapshot de Projeto.

## Reproduzir

Execute o exemplo procedural original `examples/render-studio.json` pelo mecanismo/CLI e
abra o projeto salvo no aplicativo. Selecione Render, ajuste fundo ou câmera e salve uma
configuração. O exemplo usa caixas/cilindros originais e atribuições simples de cor/opacidade;
não usa arquivos, arte ou materiais de CAD comercial. Evidências reais de Windows Vulkan/MSAA
estão anexadas a [#673](https://github.com/bherbruck/solidcraft/issues/673).
