# Arquitetura

O SolidCraft é um workspace Cargo organizado em camadas. Uma crate depende apenas de crates de camadas inferiores, e
`cargo xtask layers` faz a compilação falhar caso contrário.

{{#include ../../../README.md:architecture}}

O aplicativo web, `apps/solidcraft-web`, é a interface de desktop compilada para wasm com
[trunk](https://trunkrs.dev). `xtask` contém as ferramentas próprias do repositório: `cargo xtask ci`,
`layers`, `assets`, `features`, `oracle` e `book`.

Algumas regras decorrem dessa organização em camadas:

- **Somente `crates/kernel` acessa o truck.** Tudo acima dela usa os tipos próprios do SolidCraft:
  corpos, faces, arestas, malhas e medidas. Veja [A interface do kernel](kernel.md).
- **O documento não conhece comandos nem a interface.** `crates/doc` contém os parâmetros,
  a linha do tempo de operações e a árvore de componentes, e os avalia para formar um modelo.
- **Tudo o que o usuário pode fazer é um comando** em `crates/engine`. Veja
  [O mecanismo de comandos](commands.md).
- **A interface pode ser substituída.** Nada abaixo de `ui-egui` depende de egui. A CLI e o servidor
  MCP executam o mesmo mecanismo sem uma janela. Imagens sem interface gráfica (`solidcraft-cli snapshot`, capturas MCP,
  `ui.render`, miniaturas) são geradas na GPU com os mesmos pipelines da área de visualização
  (`crates/gpu`, `capture`): o mecanismo descreve a cena (`view::scene`) e o host instala o
  renderizador (`engine::capture`). Os níveis, em ordem: GPU de hardware, adaptador por software (Mesa
  lavapipe ou llvmpipe, WARP) e, por último, o rasterizador por CPU de `crates/render`, que não desenha
  análise de superfície, destaques ou legendas. `SOLIDCRAFT_GPU=auto` percorre os três; `hardware` e `software`
  permanecem em seu nível (erro quando ele não está disponível); `cpu` força o rasterizador. A resposta de cada imagem
  identifica seu `renderer` (`gpu`, `gpu-software`, `cpu`), e uma imagem por CPU inclui um aviso.
  `solidcraft-cli gpu` lista os adaptadores e o nível escolhido pela política.
