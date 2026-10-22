# SolidCraft

[English](../../README.md)

**Um aplicativo de CAD 3D paramétrico de código aberto desenvolvido inteiramente em Rust.**

Crie esboços, aplique restrições e construa sólidos com uma linha do tempo de operações paramétricas. Altere uma dimensão
e veja seu modelo ser reconstruído. Execute no desktop, no navegador ou deixe agentes de IA
modelarem pelo servidor MCP integrado.

![SolidCraft 0.8: modele uma caixa para Raspberry Pi 4 a partir de um projeto vazio: um esboço cotado com parâmetros, extrusão, arredondamentos e casca, um rebordo arredondado com uma seleção, espaçadores repetidos nos furos de fixação da placa, recortes para portas, ranhuras de ventilação, "Pi 4" gravado com uma fonte, um material PETG com sua massa em Propriedades e uma alteração de altura em Parâmetros](https://raw.githubusercontent.com/bherbruck/solidcraft/main/docs/screenshots/demo-0.8.gif)

*Uma caixa para Raspberry Pi 4 a partir de um projeto vazio: um esboço cotado com os parâmetros da placa, extrusão, arredondamento, casca, o rebordo arredondado com uma seleção, um espaçador M2.5 repetido no retângulo de furos de 58 × 49 mm, recortes para portas, ranhuras de ventilação, "Pi 4" gravado em DejaVu Sans Bold e um material PETG definido no projeto, com sua massa em Propriedades; depois, altura de 24 → 32 mm em Parâmetros e tudo é reconstruído. Reprodução a 9×. [Vídeo completo](https://github.com/bherbruck/solidcraft/raw/refs/heads/main/docs/screenshots/demo-0.8.mp4), gravado com o [script de demonstração](https://github.com/bherbruck/solidcraft/blob/main/examples/demo/record_ui.sh).*

## Download

[![Baixar para Windows][badge-windows]][windows-msi]
[![Baixar para macOS][badge-macos]][macos-dmg]
[![Baixar para Linux][badge-linux]][linux-appimage]
[![Obter na Snap Store][badge-snap]][snap-store]
[![Experimente no navegador][badge-web]][web-app]

- **Windows:** baixe e execute o [instalador][windows-msi].
  Também disponível para PCs [ARM64][windows-arm64-msi] e [32 bits][windows-x86-msi], ou como
  [zip portátil][windows-portable] que funciona sem instalação.
  Com [WinGet](https://learn.microsoft.com/windows/package-manager/winget/):
  `winget install bherbruck.SolidCraft`.
- **macOS:** abra a [imagem de disco][macos-dmg] e arraste o SolidCraft para Aplicativos
  (Apple silicon e Intel).
- **Linux:** baixe o [AppImage][linux-appimage], permita sua execução como programa e abra-o.
  Ou instale o [.deb][linux-deb] (Ubuntu, Debian, Mint) ou [.rpm][linux-rpm] (Fedora,
  openSUSE), ou use [Flatpak](book/src/guide/getting-started.md#linux-flatpak).
  Com Snap: `sudo snap install solidcraft` ([Snap Store][snap-store] ou o [arquivo .snap][linux-snap]).
  Para ARM64: [AppImage][linux-arm64-appimage], [.deb][linux-arm64-deb], [.rpm][linux-arm64-rpm].
- **Web:** [experimente no navegador][web-app], sem instalação.

Os downloads ainda não são assinados. No Windows, se o SmartScreen exibir um aviso, escolha **Mais informações** e depois
**Executar assim mesmo**. No macOS, na primeira vez, clique com o botão direito no SolidCraft em Aplicativos e escolha **Abrir**.

Versões anteriores, somas de verificação e outros formatos estão na [página de versões](https://github.com/bherbruck/solidcraft/releases).
Os detalhes da instalação, a atualização de versões anteriores e a compilação a partir do código-fonte estão em
[Primeiros passos](book/src/guide/getting-started.md).

[badge-windows]: https://img.shields.io/badge/Windows-Download-0078D6?style=for-the-badge&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0id2hpdGUiPjxwYXRoIGQ9Ik0yIDJoOS41djkuNUgyek0xMi41IDJIMjJ2OS41aC05LjV6TTIgMTIuNWg5LjVWMjJIMnpNMTIuNSAxMi41SDIyVjIyaC05LjV6Ii8%2BPC9zdmc%2B
[badge-macos]: https://img.shields.io/badge/macOS-Download-333333?style=for-the-badge&logo=apple&logoColor=white
[badge-linux]: https://img.shields.io/badge/Linux-AppImage-E95420?style=for-the-badge&logo=linux&logoColor=white
[badge-snap]: https://img.shields.io/badge/Snap-Snap_Store-82BEA0?style=for-the-badge&logo=snapcraft&logoColor=white
[badge-web]: https://img.shields.io/badge/Web-Try_in_browser-2EA44F?style=for-the-badge&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgc3Ryb2tlPSJ3aGl0ZSIgc3Ryb2tlLXdpZHRoPSIyIj48Y2lyY2xlIGN4PSIxMiIgY3k9IjEyIiByPSIxMCIvPjxwYXRoIGQ9Ik0yIDEyaDIwTTEyIDJhMTUgMTUgMCAwIDEgMCAyME0xMiAyYTE1IDE1IDAgMCAwIDAgMjAiLz48L3N2Zz4%3D
[windows-msi]: https://github.com/bherbruck/solidcraft/releases/latest/download/solidcraft-windows-x64.msi
[windows-arm64-msi]: https://github.com/bherbruck/solidcraft/releases/latest/download/solidcraft-windows-arm64.msi
[windows-x86-msi]: https://github.com/bherbruck/solidcraft/releases/latest/download/solidcraft-windows-x86.msi
[windows-portable]: https://github.com/bherbruck/solidcraft/releases/latest/download/solidcraft-windows-x64-portable.zip
[macos-dmg]: https://github.com/bherbruck/solidcraft/releases/latest/download/solidcraft-macos-universal.dmg
[linux-appimage]: https://github.com/bherbruck/solidcraft/releases/latest/download/solidcraft-linux-x86_64.AppImage
[linux-deb]: https://github.com/bherbruck/solidcraft/releases/latest/download/solidcraft-linux-x86_64.deb
[linux-rpm]: https://github.com/bherbruck/solidcraft/releases/latest/download/solidcraft-linux-x86_64.rpm
[linux-arm64-appimage]: https://github.com/bherbruck/solidcraft/releases/latest/download/solidcraft-linux-aarch64.AppImage
[linux-arm64-deb]: https://github.com/bherbruck/solidcraft/releases/latest/download/solidcraft-linux-aarch64.deb
[linux-arm64-rpm]: https://github.com/bherbruck/solidcraft/releases/latest/download/solidcraft-linux-aarch64.rpm
[linux-snap]: https://github.com/bherbruck/solidcraft/releases/latest/download/solidcraft-linux-x86_64.snap
[snap-store]: https://snapcraft.io/solidcraft
[web-app]: https://bherbruck.github.io/solidcraft/app/

**Documentação:** o [livro do SolidCraft](book/src/introduction.md) (guia do usuário,
guia do desenvolvedor e referência de comandos; fontes em [docs/book](book/src/introduction.md)).

**Estado: inicial, mas utilizável para peças simples.** 75 das 76 peças públicas de referência
correspondem ao volume, à área, ao centro de massa e à caixa delimitadora calculados analiticamente ou pelo FreeCAD
([docs/oracle.md](oracle.md)); a outra tem um defeito acompanhado.
Essas verificações não estabelecem equivalência geométrica ou de fluxo de trabalho completa. Os comandos disponíveis no SolidCraft
estão listados por área em [docs/features.md](features.md); o estado e os planos estão em [ROADMAP.md](ROADMAP.md).
O trabalho em aberto e os defeitos conhecidos estão nas [issues do GitHub](https://github.com/bherbruck/solidcraft/issues),
e sua ajuda é bem-vinda: veja [CONTRIBUTING.md](CONTRIBUTING.md).

## Feito com SolidCraft

<a href="https://github.com/0xlillith/solidcraft-rover"><img src="../screenshots/rover-build.gif" alt="SELENE, um veículo explorador científico de 0xlillith, montando-se enquanto a linha do tempo percorre suas 3.323 etapas no SolidCraft: chassi, rodas, eletrônica, suspensão, braço de amostragem, painéis solares, antena parabólica e torre da câmera" loading="lazy"></a>

*SELENE, um veículo explorador científico construído inteiramente pelo servidor MCP do SolidCraft por
[@0xlillith](https://github.com/0xlillith/solidcraft-rover): 3.323 etapas editáveis da linha do tempo que
são reconstruídas sem erros, reproduzidas aqui a 2× de velocidade.
[Vídeo completo](https://github.com/bherbruck/solidcraft/raw/refs/heads/main/docs/screenshots/rover-build.mp4)
· [baixe o modelo editável](https://github.com/0xlillith/solidcraft-rover/releases/tag/v1.0.0).*

## O que funciona

- **Exportação da vista atual em DXF** em Arquivo → Exportar vista atual como DXF, com arestas ocultas
  omitidas ou tracejadas, posições dos corpos visíveis e a câmera atual. Veja [DXF da vista](view-dxf.md).

- **Esboços** em planos de origem, planos deslocados ou faces planas: linhas, retângulos, círculos,
  arcos, polígonos, ranhuras, elipses, splines por pontos de ajuste/controle, cônicas e texto; geometria de construção,
  projeção, recorte, arredondamentos e chanfros, incluindo cantos na origem.
  Novas vinculações a faces mantêm as identidades de construção suportadas e as ocorrências escolhidas;
  fontes ausentes expõem um fluxo explícito de reparo do plano. Veja [vinculações a faces](face-attachments.md).
  Splines por pontos de ajuste explicitamente fechadas e deslocamentos associativos fechados usam interpolação C2 periódica.
  Deslocamentos convexos suportados de linhas e arcos mantêm sua identidade quando um deslocamento para dentro consome arcos
  de canto, e as extrusões vinculadas seguem o contorno atual. Romper a associação congela
  o contorno e mantém as extrusões existentes utilizáveis.
  O desenho e a seleção de curvas se adaptam juntos ao zoom e à escala da tela, incluindo círculos pequenos
  no ajuste da vista e esboços em componentes posicionados.
- **Restrições e cotas** resolvidas por nosso próprio solucionador. Pontos que as restrições fixam em relação
  à geometria conhecida são posicionados exatamente por construção (interseções de lugares geométricos e decomposição de agrupamentos
  rígidos, segundo Owen 1991 e Fudos–Hoffmann 1997); Levenberg–Marquardt resolve o restante,
  e o posto de sua matriz jacobiana fornece os graus de liberdade restantes. Restrições: coincidência,
  ponto sobre curva, horizontal/vertical, paralelismo, perpendicularidade, tangência, curvatura (G2), igualdade,
  concentricidade, colinearidade, ponto médio, simetria, fixação; cotas lineares, alinhadas, horizontais, verticais,
  de ponto a linha, raio, diâmetro, ângulo, abertura de arco, comprimento de arco e comprimento de ranhura controladas
  por parâmetros. Alterações com restrições excessivas são rejeitadas; a geometria totalmente restringida fica preta.
  Arrastar resolve o esboço a cada quadro e reconstrói o modelo uma vez, quando o arraste termina.
  Selecione uma linha ou suas duas extremidades, digite sua dimensão e pressione Enter para confirmar uma única etapa de desfazer;
  com Cota de esboço, a sequência é D, clique, digite, Enter. Ajustar mantém a geometria do esboço afastada da paleta e dos controles de navegação.
- **Perfis** identificados automaticamente, incluindo regiões divididas por linhas e furos dentro de contornos.
  Contornos adaptativos completos recém-capturados mantêm as identidades das fontes externas e dos furos
  quando muda a quantidade de trechos da spline. Contornos parciais ou ambíguos não se qualificam.
- **Operações** em uma linha do tempo paramétrica: extrusão, revolução, varredura, loft, arredondamento/chanfro,
  casca, ângulo de saída, furos, roscas, padrões, espelhamento, divisão, combinação, movimento e geometria de construção.
  A linha do tempo é reconstruída incrementalmente; as operações podem ser editadas, suprimidas, renomeadas, reordenadas
  e excluídas. A geometria suportada e as limitações atuais são descritas em [ROADMAP.md](ROADMAP.md).
  Novas revoluções preservam o posicionamento angular simétrico e bilateral com eixos invertidos;
  projetos mais antigos mantêm a convenção salva ao serem reabertos ou editados.
- **Parâmetros e configurações** com expressões que reconhecem unidades (`2 * width + 5 mm`,
  `angle / 2`), regras ordenadas, supressão de operações e materiais/aparências dos corpos por linha.
- **Montagens** com componentes e ocorrências aninhados, juntas, estudos de movimento, conjuntos de contato,
  interferência e propriedades de massa que consideram os materiais. Transferências de corpos registradas preservam o histórico
  de modelagem e o posicionamento global; fios de esboço 3D mantêm seus sistemas de coordenadas durante as alterações.
- **Chapas metálicas** com flanges, bainhas, dobrar/desdobrar/dobrar novamente, recortes de material, planificações,
  cavidades prismáticas rasas preservadas, contornos DXF exatos de linhas/arcos/círculos e tabelas de dobras;
  operações e regras para caixas plásticas. A profundidade e o lado da cavidade sobrevivem ao desdobramento e à redobra e aparecem
  separados dos contornos dos furos passantes na prévia plana e no DXF.
- **Arquivos**: contêineres de projeto (`.solidcraft`) com dados binários de malha sem perda;
  JSON simples continua suportado para abrir, Salvar como e exportar. Abrir no desktop pode mostrar uma
  imagem provisória salva de caches de geometria já calculada durante a reconstrução anterior à edição; os caches nunca
  substituem o projeto paramétrico. Exportação para STEP (AP242 com nomes de corpos, cores e
  componentes como montagens; faces de revolução como planos, cilindros, cones, esferas e toros nativos),
  IGES 5.3 (B-reps de sólidos manifold com nomes e cores), 3MF, STL (binário/ASCII) e OBJ, e
  montagens com juntas como URDF para simuladores de robôs (docs/urdf.md);
  3MF, STL e OBJ são importados como corpos de malha (renderizam, medem, movem e exportam; operações de sólidos
  exigem corpos B-rep). O comando `mesh.convert` reconstrói peças fechadas suportadas de seção constante
  como sólidos paramétricos com faces planas e cilíndricas circulares;
  importação STEP (sólidos AP203/AP214/AP242, montagens, unidades, nomes, cores) e IGES
  (B-reps de sólidos manifold ou superfícies recortadas e delimitadas costuradas em sólidos) como operação-base
  sobre a qual operações posteriores são construídas — abra um arquivo `.step`/`.stp`/`.igs`/`.iges`, insira-o em um
  projeto ou solte-o na janela ([docs/step-import.md](step-import.md)).
- **Renderização:** superfícies opacas podem usar metalicidade, rugosidade e IOR dielétrico
  na vista integrada de produto e em PNGs de prévia por CPU. Cores simples continuam suportadas.
  Iluminação direta fixa é uma prévia; HDRI, refração e mapas de textura não estão implementados.
  Veja [superfícies opacas](render-opaque-pbr.md).
- **Malhas de exibição** do nosso planejador de tesselação: cada face vira um plano de pontos
  (u, v) e triângulos que compartilha nós de arestas com as vizinhas, formando uma malha estanque.
  Facetas seguem tolerância de corda, ângulo e maior lado, com células de no máximo 2:1 em
  direções retas. Faces limitadas por caixa e de quatro cantos usam grades estruturadas.
  Faces recortadas são divididas em regiões de quatro lados: quadrados ao redor de furos,
  borboletas em faces redondas, traçados de motocicleta a partir de cantos côncavos (Lyon et al.
  2019). Cada região usa uma grade de Coons. Arestas de interseção são avaliadas por Newton
  em ambas as superfícies. Um compute shader avalia os planos da cena inteira em um despacho,
  com fallback CPU se a GPU não conseguir. Planejar e avaliar os 78 corpos de referência leva
  cerca de 65 ms, 50–65× mais rápido que o gerador anterior.

- **Aplicativo para desktop** (egui + wgpu): barra de ferramentas com abas de espaços de trabalho, árvore do modelo, linha do tempo, vista 3D com
  vista de eixos e barra de navegação, ferramentas de esboço, diálogos de operações, paleta de comandos e um canal de controle
  de linhas JSON para que agentes e testes possam controlá-lo.
  Erros de operações e a Ajuda podem abrir um rascunho editável de **Criar issue** com contexto relevante
  e uma captura de tela opcional revisada; abrir o GitHub não envia o relatório.
- **CLI sem interface gráfica** `solidcraft-cli`: execute scripts de comandos, meça (volume, área, centro de massa,
  contagens de faces/arestas/vértices), exporte, renderize imagens PNG e reproduza receitas de referência.
- **Servidor MCP** `solidcraft-cli mcp`: agentes de IA modelam, medem e visualizam peças sem interface gráfica ou
  no aplicativo em execução (veja abaixo).

## Compilar e executar

```sh
cargo run --release -p solidcraft -- --sample          # desktop app with a sample part
cargo run --release -p solidcraft-cli -- commands      # the command registry
cargo run --release -p solidcraft-cli -- run examples/bracket.json --out bracket.step
cargo run --release -p solidcraft-cli -- snapshot examples/bracket.json --out bracket.png
cargo run --release -p solidcraft-cli -- render examples/render-studio.json --out product.png
cargo run --release -p solidcraft -- part.step          # open a STEP file as a new design
cargo run --release -p solidcraft-cli -- eval part.step # measure an imported STEP file
cargo xtask ci                                          # fmt, clippy, tests, assets, layers, wasm
```

Compilações para Windows: `cargo xwin build --release --target x86_64-pc-windows-msvc -p solidcraft`.

No Windows e no Linux, o SolidCraft detecta um 3Dconnexion SpaceNavigator ou SpaceMouse conectado pela
interface HID multieixos. Mova o botão para deslocar e ampliar a vista; incline ou gire-o para rotacionar a vista 3D.
Ao editar um esboço, o botão desloca e amplia sem alterar a vista do esboço. A página Navegação
em Preferências tem um interruptor e um controle de velocidade para o mouse 3D.

No Linux, o SolidCraft lê `/dev/hidrawN` diretamente, portanto o usuário conectado precisa de acesso ao
dispositivo. Uma regra udev que adiciona a etiqueta `uaccess` faz isso, por exemplo,
`/etc/udev/rules.d/70-3d-mouse.rules` para dispositivos com o ID de fabricante da 3Dconnexion:

```
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="256f", TAG+="uaccess"
```

SpaceNavigators antigos usam o ID de fabricante da Logitech `046d`; adicione uma regra com seu ID de produto (veja `lsusb`)
em vez de selecionar todos os dispositivos Logitech. Recarregue com `sudo udevadm control --reload && sudo udevadm trigger` e reconecte o dispositivo. O sandbox
do Flatpak não expõe dispositivos hidraw, portanto ali o mouse 3D só funciona com compilações nativas por enquanto.
Dispositivos que enviam os seis eixos no relatório HID 1 (alguns caseiros fazem isso) são reconhecidos pelo
descritor de relatório, então também funcionam.

No navegador: `cd apps/solidcraft-web && trunk build --release --public-url ./` (WebGPU ou
WebGL2 com `?webgl`; `?sample` abre a peça de exemplo).

## Scripts

Um script é uma lista JSON de comandos, os mesmos que a interface executa:

```json
{"commands": [
  {"command": "sketch.create", "params": {"plane": "XY"}},
  {"command": "sketch.rectangle.two_point", "params": {"p0": [0, 0], "p1": [40, 30]}},
  {"command": "solid.extrude", "params": {"distance": 20}},
  {"command": "solid.fillet", "params": {"edges": [[0, 0, 10]], "radius": 3}}
]}
```

## Agentes de IA (MCP)

`solidcraft-cli mcp` é um servidor [Model Context Protocol](https://modelcontextprotocol.io) via
stdio. Sem interface gráfica, executa sua própria sessão de projeto; com `--connect 127.0.0.1:PORT`, controla o aplicativo
iniciado com `solidcraft --control PORT`, para que você possa acompanhar a modelagem do agente. Ferramentas:
`list_commands`, `execute`, `batch`, `inspect_design`, `measure`, `body_topology`,
`set_parameter`, `screenshot` (retorna a imagem), `export`, `undo`, `redo`, `new_design`, `open`,
`save`. As chamadas de ferramentas acessam arquivos somente na pasta do projeto aberto (quando está dentro da pasta pessoal,
nunca na própria pasta pessoal ou em uma pasta oculta), na pasta de recuperação do SolidCraft e nas pastas fornecidas com
`--allow-path DIR`. Claude Code: `claude mcp add solidcraft -- solidcraft-cli mcp`. Detalhes, configuração
de clientes e uma sessão de exemplo: [docs/mcp.md](mcp.md).

## Arquitetura

<!-- ANCHOR: architecture -->
| Camada | Crates | Função |
|---|---|---|
| L0 | `geom` | vetores, planos, perfis, malhas e suas medidas |
| L1 | `sketch`, `kernel`, `render` | solucionador de restrições e perfis; a fronteira do núcleo B-rep (truck); matemática da vista e descrição da cena sem interface gráfica |
| L2 | `doc` | parâmetros, expressões, a linha do tempo de operações e sua avaliação incremental |
| L3 | `io` | arquivos de projeto, exportação STL/OBJ/STEP/3MF, operações de importação STEP e 3MF/STL |
| L4 | `engine` | a sessão e o registro de comandos (tudo é um comando) |
| L5 | `gpu`, `ui-egui`, `mcp` | avaliação por compute shader de planos de tesselação de exibição; a interface de desktop substituível; o servidor MCP (sem interface gráfica ou conectado ao aplicativo) |
| apps | `solidcraft`, `solidcraft-cli` | aplicativo para desktop, CLI sem interface gráfica |
<!-- ANCHOR_END: architecture -->

## Contribuir

Está vindo do Autodesk Fusion? SolidCraft usa fluxos conhecidos de CAD paramétrico: esboços
com restrições, operações de sólidos, uma linha do tempo editável, parâmetros e montagens. Veja o
[roteiro](ROADMAP.md) para operações suportadas e limitações atuais.

Veja [CONTRIBUTING.md](CONTRIBUTING.md): como compilar e testar, o mapa do código, as regras de desenvolvimento independente
e licenças e onde a ajuda é mais necessária.

## Licença

MIT OR Apache-2.0, à sua escolha. Desenvolvido sob as
[regras de contribuição independente](project/CLAUDE.md); código e recursos de terceiros mantêm suas
licenças e atribuições documentadas.

SolidCraft é independente. O comportamento é projetado a partir de documentação pública, convenções
gerais de CAD e comentários dos usuários. Os testes de referência usam valores analíticos e ferramentas de código aberto.
SolidCraft não tem afiliação com a Autodesk e não é patrocinado nem endossado por ela. Autodesk e
Fusion são marcas registradas da Autodesk, Inc.
