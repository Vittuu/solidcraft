# Atribuições

| Arquivo | Autor | Fonte | Licença |
|---|---|---|---|
| examples/render-studio.json | colaboradores do SolidCraft | Script procedural original de caixa/cilindro para o espaço Render integrado | MIT OR Apache-2.0 |
| examples/render-opaque-studio.json | colaboradores do SolidCraft | Estudo procedural original de esferas variando independentemente fração metálica opaca e rugosidade; sem recursos externos ou dados de engenharia | MIT OR Apache-2.0 |
| crates/ui-egui/src/pbr_gpu.rs | colaboradores do SolidCraft | Tradução WGSL original da superfície opaca e referência fixa de estúdio do SolidCraft, incorporada como fonte Rust; sem shaders externos | MIT OR Apache-2.0 |
| assets/locales/pt-BR.json | colaboradores do SolidCraft | Tradução original das mensagens do SolidCraft para português brasileiro | MIT OR Apache-2.0 |
| assets/locales/protocol/pt-BR.json | colaboradores do SolidCraft | Traduções originais para português brasileiro dos textos de apresentação da CLI e do MCP | MIT OR Apache-2.0 |
| assets/locales/commands/pt-BR.json | colaboradores do SolidCraft | Documentação original de parâmetros de comandos em português brasileiro | MIT OR Apache-2.0 |
| assets/locales/diagnostics/pt-BR.json | colaboradores do SolidCraft | Traduções originais para português brasileiro dos diagnósticos e relatórios próprios do projeto | MIT OR Apache-2.0 |
| assets/locales/diagnostics/arguments.json | colaboradores do SolidCraft | Campos auditados de modelos de diagnóstico que contêm prosa própria aninhada do projeto | MIT OR Apache-2.0 |
| assets/locales/book/pt-BR.json | colaboradores do SolidCraft | Traduções originais para português brasileiro da navegação, busca e área de transferência do mdBook | MIT OR Apache-2.0 |
| assets/documentation/screenshots/source/sample-plate-pt-BR.png | colaboradores do SolidCraft | Captura original do SolidCraft 0.10.0 com interface em português brasileiro e a placa pública de exemplo; fonte de sample-plate.png gerado para o manual em português | MIT OR Apache-2.0 |
| docs/pt-BR/book/src/generated/sample-plate.png | colaboradores do SolidCraft | Cópia de docs/screenshots/sample-plate.png gerada por cargo xtask book (ignorada pelo git) | MIT OR Apache-2.0 |
| docs/pt-BR/book/src/generated/bracket.png | colaboradores do SolidCraft | Cópia da renderização original do SolidCraft gerada por cargo xtask book (ignorada pelo git) | MIT OR Apache-2.0 |
| docs/pt-BR/book/src/generated/enclosure.png | colaboradores do SolidCraft | Cópia da renderização original do SolidCraft gerada por cargo xtask book (ignorada pelo git) | MIT OR Apache-2.0 |
| docs/pt-BR/book/src/generated/revolve.png | colaboradores do SolidCraft | Cópia da renderização original do SolidCraft gerada por cargo xtask book (ignorada pelo git) | MIT OR Apache-2.0 |
| docs/pt-BR/book/src/generated/sheet-metal.png | colaboradores do SolidCraft | Cópia da renderização original do SolidCraft gerada por cargo xtask book (ignorada pelo git) | MIT OR Apache-2.0 |

Todo recurso não relacionado a código neste repositório (imagens, ícones, fontes, peças de exemplo) está listado aqui com
seu autor, sua fonte e sua licença. `cargo xtask assets` (parte de `cargo xtask ci`) falha se um arquivo
de recurso estiver ausente desta tabela.

**Política (obrigatória):** SolidCraft não contém **ícones, imagens, arte, fontes,
modelos, materiais ou textos de ajuda da Autodesk.** Todo recurso é um trabalho original dos colaboradores do SolidCraft ou
material de terceiros sob licença aberta. Capturas de tela de software da Autodesk nunca são incluídas no repositório.

Código editável de desenho, projetos nativos e exportações distribuídas estão reunidos em `assets/`.

## Arquivos

| Arquivo | Autor | Fonte | Licença |
|---|---|---|---|
| assets/archive/icons/macos-v1/exports/icon.svg | colaboradores do SolidCraft | adaptação original para macOS da marca SolidCraft desenhada em código, editada e exportada com VectorCraft; Apple HIG e modelos de produção consultados apenas como referências de layout | MIT OR Apache-2.0 |
| assets/icons/app/macos/source/SolidCraft.icon/icon.json | colaboradores do SolidCraft | composição aprovada no Icon Composer e configurações de aparência para a arte original do SolidCraft | MIT OR Apache-2.0 |
| assets/icons/app/macos/source/SolidCraft.icon/Assets/02-grid.svg | colaboradores do SolidCraft | grade de perspectiva original com alfa radial elíptico, exportada do VectorCraft | MIT OR Apache-2.0 |
| assets/icons/app/macos/source/SolidCraft.icon/Assets/02-grid-dark.svg | colaboradores do SolidCraft | grade de perspectiva azul original com alfa radial elíptico, exportada do VectorCraft para a aparência escura | MIT OR Apache-2.0 |
| assets/icons/app/macos/source/SolidCraft.icon/Assets/03-solid-blue-cutout.svg | colaboradores do SolidCraft | sólido CAD original em azul saturado com recortes transparentes de topologia, editado e exportado com VectorCraft | MIT OR Apache-2.0 |
| assets/icons/app/macos/exports/Assets.car | colaboradores do SolidCraft | compilação por Apple actool de assets/icons/app/macos/source/SolidCraft.icon para macOS; contém apenas arte original | MIT OR Apache-2.0 |
| assets/archive/icons/macos-v1/source/icon.vectorcraft | colaboradores do SolidCraft | fonte editável do VectorCraft para assets/archive/icons/macos-v1/exports/icon.svg, com camadas nomeadas de faces e fundo | MIT OR Apache-2.0 |
| assets/archive/icons/macos-v1/exports/icon.png | colaboradores do SolidCraft | exportação sRGB transparente de 1024 × 1024 de assets/archive/icons/macos-v1/exports/icon.svg a partir do VectorCraft | MIT OR Apache-2.0 |
| assets/icons/app/macos/exports/icon.icns | colaboradores do SolidCraft | alternativa para macOS compilada pelo actool da Apple a partir da composição aprovada de assets/icons/app/macos/source/SolidCraft.icon | MIT OR Apache-2.0 |
| assets/archive/icons/macos-v1/exports/icon.iconset/icon_16x16.png | colaboradores do SolidCraft | exportação nativa por sips a partir de assets/archive/icons/macos-v1/exports/icon.png | MIT OR Apache-2.0 |
| assets/archive/icons/macos-v1/exports/icon.iconset/icon_16x16@2x.png | colaboradores do SolidCraft | exportação nativa por sips a partir de assets/archive/icons/macos-v1/exports/icon.png | MIT OR Apache-2.0 |
| assets/archive/icons/macos-v1/exports/icon.iconset/icon_32x32.png | colaboradores do SolidCraft | exportação nativa por sips a partir de assets/archive/icons/macos-v1/exports/icon.png | MIT OR Apache-2.0 |
| assets/archive/icons/macos-v1/exports/icon.iconset/icon_32x32@2x.png | colaboradores do SolidCraft | exportação nativa por sips a partir de assets/archive/icons/macos-v1/exports/icon.png | MIT OR Apache-2.0 |
| assets/archive/icons/macos-v1/exports/icon.iconset/icon_128x128.png | colaboradores do SolidCraft | exportação nativa por sips a partir de assets/archive/icons/macos-v1/exports/icon.png | MIT OR Apache-2.0 |
| assets/archive/icons/macos-v1/exports/icon.iconset/icon_128x128@2x.png | colaboradores do SolidCraft | exportação nativa por sips a partir de assets/archive/icons/macos-v1/exports/icon.png | MIT OR Apache-2.0 |
| assets/archive/icons/macos-v1/exports/icon.iconset/icon_256x256.png | colaboradores do SolidCraft | exportação nativa por sips a partir de assets/archive/icons/macos-v1/exports/icon.png | MIT OR Apache-2.0 |
| assets/archive/icons/macos-v1/exports/icon.iconset/icon_256x256@2x.png | colaboradores do SolidCraft | exportação nativa por sips a partir de assets/archive/icons/macos-v1/exports/icon.png | MIT OR Apache-2.0 |
| assets/archive/icons/macos-v1/exports/icon.iconset/icon_512x512.png | colaboradores do SolidCraft | exportação nativa por sips a partir de assets/archive/icons/macos-v1/exports/icon.png | MIT OR Apache-2.0 |
| assets/archive/icons/macos-v1/exports/icon.iconset/icon_512x512@2x.png | colaboradores do SolidCraft | exportação nativa por sips a partir de assets/archive/icons/macos-v1/exports/icon.png | MIT OR Apache-2.0 |
| docs/screenshots/sample-plate.png | colaboradores do SolidCraft | captura de tela do próprio SolidCraft (`solidcraft --sample`) | MIT OR Apache-2.0 |
| docs/screenshots/demo-0.8.gif | colaboradores do SolidCraft | gravação de tela original da interface real de desktop do SolidCraft 0.8 em inglês modelando uma caixa original para Raspberry Pi 4 (sem logotipos ou marcas; o texto gravado usa DejaVu Sans Bold do sistema), gerada com examples/demo/record_ui.sh e sua sobreposição original de ponteiro animado (reprodução a 9×) | MIT OR Apache-2.0 |
| docs/screenshots/demo-0.8.mp4 | colaboradores do SolidCraft | gravação de tela original da interface real de desktop do SolidCraft 0.8 em inglês modelando uma caixa original para Raspberry Pi 4 (sem logotipos ou marcas; o texto gravado usa DejaVu Sans Bold do sistema), gerada com examples/demo/record_ui.sh e sua sobreposição original de ponteiro animado (reprodução a 9×) | MIT OR Apache-2.0 |
| docs/screenshots/rover-build.gif | 0xlillith (modelo); colaboradores do SolidCraft (gravação) | gravação de tela da interface real de desktop do SolidCraft reproduzindo a linha do tempo do veículo explorador SELENE (`Rover-recorded.solidcraft` de https://github.com/0xlillith/solidcraft-rover, um modelo original de 0xlillith construído pelo MCP do SolidCraft); as letras do modelo seguem os contornos de Ubuntu Light (Canonical Ltd., Ubuntu Font Licence 1.0, conforme indicado nesse repositório); o emblema é o ícone do projeto SolidCraft (reprodução a 2×) | MIT OR Apache-2.0 |
| docs/screenshots/rover-build.mp4 | 0xlillith (modelo); colaboradores do SolidCraft (gravação) | gravação de tela da interface real de desktop do SolidCraft reproduzindo a linha do tempo do veículo explorador SELENE (`Rover-recorded.solidcraft` de https://github.com/0xlillith/solidcraft-rover, um modelo original de 0xlillith construído pelo MCP do SolidCraft); as letras do modelo seguem os contornos de Ubuntu Light (Canonical Ltd., Ubuntu Font Licence 1.0, conforme indicado nesse repositório); o emblema é o ícone do projeto SolidCraft | MIT OR Apache-2.0 |
| assets/branding/web/source/og.png | colaboradores do SolidCraft | cartão original de prévia social de 1280 × 640 (a marca, o nome e o slogan do SolidCraft), servido pelo site de documentação para prévias de links | MIT OR Apache-2.0 |
| docs/book/src/generated/sample-plate.png | colaboradores do SolidCraft | cópia de docs/screenshots/sample-plate.png feita por `cargo xtask book` (ignorada pelo git) | MIT OR Apache-2.0 |
| docs/book/src/generated/bracket.png | colaboradores do SolidCraft | renderizado por `cargo xtask book` (`solidcraft-cli snapshot examples/bracket.json`; ignorado pelo git) | MIT OR Apache-2.0 |
| docs/book/src/generated/enclosure.png | colaboradores do SolidCraft | renderizado por `cargo xtask book` (`solidcraft-cli snapshot docs/book/scripts/enclosure.json`; ignorado pelo git) | MIT OR Apache-2.0 |
| docs/book/src/generated/revolve.png | colaboradores do SolidCraft | renderizado por `cargo xtask book` (`solidcraft-cli snapshot docs/book/scripts/revolve.json`; ignorado pelo git) | MIT OR Apache-2.0 |
| docs/book/src/generated/sheet-metal.png | colaboradores do SolidCraft | renderizado por `cargo xtask book` (`solidcraft-cli snapshot docs/book/scripts/sheet-metal.json`; ignorado pelo git) | MIT OR Apache-2.0 |
| assets/fonts/liberation-sans/LiberationSans-Regular.ttf | Red Hat, Inc. (Liberation Fonts 2.x; dados digitalizados Google) | https://github.com/liberationfonts (Debian fonts-liberation), sem modificações; texto da licença em assets/fonts/liberation-sans/OFL.txt; usada para os contornos de Texto de esboço | SIL OFL 1.1 |
| assets/fonts/biz-udp-gothic/BIZUDPGothic-Regular.ttf | The BIZ UDGothic Project Authors | https://github.com/googlefonts/morisawa-biz-ud-gothic (Google Fonts), sem modificações; texto da licença em assets/fonts/biz-udp-gothic/OFL.txt; usada como fonte japonesa da interface web | SIL OFL 1.1 |
| tests/oracle/17-loft/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/17-loft/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/23-bracket-multifeature/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/23-bracket-multifeature/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/28-blind-pocket-fillets/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/28-blind-pocket-fillets/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/30-fillet-after-shell/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/30-fillet-after-shell/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/31-fillet-l-all-edges/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/31-fillet-l-all-edges/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/34-chamfer-then-fillet/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/34-chamfer-then-fillet/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/35-fillet-curved-curved/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/35-fillet-curved-curved/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/36-corner-blend-two-radii/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/36-corner-blend-two-radii/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/37-pyramid-apex-blend/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/37-pyramid-apex-blend/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/39-tee-pipe/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/39-tee-pipe/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/41-tube-radial-holes/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/41-tube-radial-holes/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/42-overlapping-patterns/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/42-overlapping-patterns/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/47-draft-curved-pocket/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/47-draft-curved-pocket/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/51-loft-rect-circle-offset/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/51-loft-rect-circle-offset/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/56-holes-on-curved-face/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/56-holes-on-curved-face/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/58-phone-stand/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/58-phone-stand/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/59-enclosure-base/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/59-enclosure-base/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/61-shaft-keyway/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/61-shaft-keyway/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/63-hinge-knuckle/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/63-hinge-knuckle/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |
| tests/oracle/65-sketch-gear-tooth/reference.stl | colaboradores do SolidCraft | triangulação de nossa própria receita (tests/oracle/65-sketch-gear-tooth/recipe.json) construída com FreeCAD como ferramenta externa por tools/freecad-oracle | MIT OR Apache-2.0 |

| assets/appearances/library.rs | colaboradores do SolidCraft | Dados de recursos ou código de desenho originais do SolidCraft, reunidos sem redesenho | MIT OR Apache-2.0 |
| assets/fonts/biz-udp-gothic/OFL.txt | The BIZ UDGothic Project Authors | Licença acompanhante da fonte, sem modificações | SIL OFL 1.1 |
| assets/fonts/liberation-sans/OFL.txt | Red Hat, Inc.; colaboradores de Liberation Fonts | Licença acompanhante da fonte, sem modificações | SIL OFL 1.1 |
| assets/icons/app/linux/exports/hicolor/128x128/apps/ai.storyteller.solidcraft.png | colaboradores do SolidCraft | Gerado a partir de assets/icons/app/shared/source/app_icon.rs com os codificadores originais CLI/ICO | MIT OR Apache-2.0 |
| assets/icons/app/linux/exports/hicolor/16x16/apps/ai.storyteller.solidcraft.png | colaboradores do SolidCraft | Gerado a partir de assets/icons/app/shared/source/app_icon.rs com os codificadores originais CLI/ICO | MIT OR Apache-2.0 |
| assets/icons/app/linux/exports/hicolor/24x24/apps/ai.storyteller.solidcraft.png | colaboradores do SolidCraft | Gerado a partir de assets/icons/app/shared/source/app_icon.rs com os codificadores originais CLI/ICO | MIT OR Apache-2.0 |
| assets/icons/app/linux/exports/hicolor/256x256/apps/ai.storyteller.solidcraft.png | colaboradores do SolidCraft | Gerado a partir de assets/icons/app/shared/source/app_icon.rs com os codificadores originais CLI/ICO | MIT OR Apache-2.0 |
| assets/icons/app/linux/exports/hicolor/32x32/apps/ai.storyteller.solidcraft.png | colaboradores do SolidCraft | Gerado a partir de assets/icons/app/shared/source/app_icon.rs com os codificadores originais CLI/ICO | MIT OR Apache-2.0 |
| assets/icons/app/linux/exports/hicolor/48x48/apps/ai.storyteller.solidcraft.png | colaboradores do SolidCraft | Gerado a partir de assets/icons/app/shared/source/app_icon.rs com os codificadores originais CLI/ICO | MIT OR Apache-2.0 |
| assets/icons/app/linux/exports/hicolor/512x512/apps/ai.storyteller.solidcraft.png | colaboradores do SolidCraft | Gerado a partir de assets/icons/app/shared/source/app_icon.rs com os codificadores originais CLI/ICO | MIT OR Apache-2.0 |
| assets/icons/app/linux/exports/hicolor/64x64/apps/ai.storyteller.solidcraft.png | colaboradores do SolidCraft | Gerado a partir de assets/icons/app/shared/source/app_icon.rs com os codificadores originais CLI/ICO | MIT OR Apache-2.0 |
| assets/icons/app/shared/source/app_icon.rs | colaboradores do SolidCraft | Dados de recursos ou código de desenho originais do SolidCraft, reunidos sem redesenho | MIT OR Apache-2.0 |
| assets/icons/app/shared/source/windows_ico.rs | colaboradores do SolidCraft | Dados de recursos ou código de desenho originais do SolidCraft, reunidos sem redesenho | MIT OR Apache-2.0 |
| assets/icons/app/windows/exports/solidcraft-executable.ico | colaboradores do SolidCraft | Gerado a partir de assets/icons/app/shared/source/app_icon.rs com os codificadores originais CLI/ICO | MIT OR Apache-2.0 |
| assets/icons/app/windows/exports/solidcraft.ico | colaboradores do SolidCraft | Gerado a partir de assets/icons/app/shared/source/app_icon.rs com os codificadores originais CLI/ICO | MIT OR Apache-2.0 |
| assets/icons/ui/source/glyphs.rs | colaboradores do SolidCraft / VectorCraft | Glifos originais; geometria dos controles da janela adaptada de VectorCraft titlebar.rs (https://github.com/storytold/vectorcraft) | MIT OR Apache-2.0 |
| assets/icons/ui/source/icons.rs | colaboradores do SolidCraft | Dados de recursos ou código de desenho originais do SolidCraft, reunidos sem redesenho | MIT OR Apache-2.0 |
| assets/icons/ui/source/markers.rs | colaboradores do SolidCraft | Dados de recursos ou código de desenho originais do SolidCraft, reunidos sem redesenho | MIT OR Apache-2.0 |
| assets/icons/ui/source/navigation.rs | colaboradores do SolidCraft | Dados de recursos ou código de desenho originais do SolidCraft, reunidos sem redesenho | MIT OR Apache-2.0 |
| assets/icons/ui/source/pointer.rs | colaboradores do SolidCraft | Dados de recursos ou código de desenho originais do SolidCraft, reunidos sem redesenho | MIT OR Apache-2.0 |
| assets/locales/de.json | colaboradores do SolidCraft | Dados de recursos ou código de desenho originais do SolidCraft, reunidos sem redesenho | MIT OR Apache-2.0 |
| assets/locales/en.json | colaboradores do SolidCraft | Dados de recursos ou código de desenho originais do SolidCraft, reunidos sem redesenho | MIT OR Apache-2.0 |
| assets/locales/fr.json | rlartot | Tradução francesa do catálogo inglês, contribuída ao upstream | MIT OR Apache-2.0 |
| assets/locales/ja.json | colaboradores do SolidCraft | Dados de recursos ou código de desenho originais do SolidCraft, reunidos sem redesenho | MIT OR Apache-2.0 |
| assets/locales/ko.json | colaboradores do SolidCraft | Dados de recursos ou código de desenho originais do SolidCraft, reunidos sem redesenho | MIT OR Apache-2.0 |
| assets/locales/ru.json | Alex Pro (del000) | Tradução russa do catálogo inglês, contribuída ao upstream | MIT OR Apache-2.0 |
| assets/locales/uk.json | Igor Bronovskyi (BrunIF) | Tradução ucraniana do catálogo inglês, contribuída ao upstream | MIT OR Apache-2.0 |
| assets/locales/zh.json | colaboradores do SolidCraft | Dados de recursos ou código de desenho originais do SolidCraft, reunidos sem redesenho | MIT OR Apache-2.0 |
| assets/materials/library.rs | colaboradores do SolidCraft | Dados de recursos ou código de desenho originais do SolidCraft, reunidos sem redesenho | MIT OR Apache-2.0 |
| assets/platforms/linux/ai.storyteller.solidcraft.desktop | colaboradores do PhotoCraft / SolidCraft | Modelos de empacotamento do PhotoCraft (https://github.com/storytold/photocraft), adaptados para SolidCraft | Apache-2.0 |
| assets/platforms/linux/ai.storyteller.solidcraft.metainfo.xml.in | colaboradores do PhotoCraft / SolidCraft | Modelos de empacotamento do PhotoCraft (https://github.com/storytold/photocraft), adaptados para SolidCraft | Apache-2.0 |
| assets/platforms/linux/ai.storyteller.solidcraft.mime.xml | colaboradores do PhotoCraft / SolidCraft | Modelos de empacotamento do PhotoCraft (https://github.com/storytold/photocraft), adaptados para SolidCraft | Apache-2.0 |
| assets/platforms/macos/Info.plist.in | colaboradores do PhotoCraft / SolidCraft | Modelos de empacotamento do PhotoCraft (https://github.com/storytold/photocraft), adaptados para SolidCraft | Apache-2.0 |
| assets/platforms/macos/README.txt | colaboradores do PhotoCraft / SolidCraft | Modelos de empacotamento do PhotoCraft (https://github.com/storytold/photocraft), adaptados para SolidCraft | Apache-2.0 |
| assets/platforms/windows/solidcraft.en-us.wxl | colaboradores do PhotoCraft / SolidCraft | Modelos de empacotamento do PhotoCraft (https://github.com/storytold/photocraft), adaptados para SolidCraft | Apache-2.0 |
| assets/platforms/windows/solidcraft.wxs | colaboradores do PhotoCraft / SolidCraft | Modelos de empacotamento do PhotoCraft (https://github.com/storytold/photocraft), adaptados para SolidCraft | Apache-2.0 |
| assets/shaders/viewport.wgsl | colaboradores do SolidCraft | Dados de recursos ou código de desenho originais do SolidCraft, reunidos sem redesenho | MIT OR Apache-2.0 |
| assets/styles/ui/palettes.rs | colaboradores do SolidCraft | Dados de recursos ou código de desenho originais do SolidCraft, reunidos sem redesenho | MIT OR Apache-2.0 |
| assets/styles/web/loading.css | colaboradores do SolidCraft | Dados de recursos ou código de desenho originais do SolidCraft, reunidos sem redesenho | MIT OR Apache-2.0 |

| docs/book/src/og.png | colaboradores do SolidCraft | cópia de assets/branding/web/source/og.png preparada por cargo xtask book (ignorada pelo git) | MIT OR Apache-2.0 |

## Código adaptado de projetos relacionados

| Arquivo do SolidCraft | Adaptado de | Licença |
|---|---|---|
| xtask/src/layers.rs | CADCraft `xtask/src/layers.rs` (regras e testes de camadas de dependências) | MIT OR Apache-2.0 |
| apps/solidcraft/src/control_server.rs | CADCraft `apps/cadcraft/src/control_server.rs` (servidor de linhas JSON em loopback) | MIT OR Apache-2.0 |
| crates/ui-egui/src/lib.rs (fila de capturas de tela, entrada sintética) | CADCraft `crates/ui-egui/src/lib.rs` | MIT OR Apache-2.0 |
| crates/ui-egui/src/control.rs (despacho de métodos e gravação de capturas de tela) | CADCraft `crates/ui-egui/src/control.rs` | MIT OR Apache-2.0 |
| crates/ui-egui/src/gpu.rs (estrutura de callback de pintura) | CADCraft `crates/ui-egui/src/gpu.rs` | MIT OR Apache-2.0 |
| crates/mcp/src/server.rs, crates/mcp/src/backend.rs (enquadramento JSON-RPC, ciclo de vida MCP, ponte da porta de controle) | CADCraft `crates/mcp/src/{server,backend}.rs` | MIT OR Apache-2.0 |
| crates/mcp/src/tools.rs (auxiliares de esquema de ferramentas e verificação de argumentos) | CADCraft `crates/mcp/src/tools.rs`, GridCraft `crates/mcp/src/tools.rs` | MIT OR Apache-2.0 |
| crates/io/src/threemf.rs (leitura e gravação limitadas de partes zip) | GridCraft `crates/xlsx/src/package.rs` | MIT OR Apache-2.0 |
| crates/ui-egui/src/titlebar.rs (arraste da barra de título, botões da janela, zonas de redimensionamento e seus testes) e a configuração da janela em apps/solidcraft/src/main.rs | VectorCraft `crates/ui-egui/src/titlebar.rs`, `apps/vectorcraft/src/main.rs` | MIT OR Apache-2.0 |
| apps/solidcraft/src/open_documents.rs (macOS: arquivos abertos pelo Finder e pelo Dock, seu teste) | VectorCraft `apps/vectorcraft/src/open_documents.rs` (https://github.com/storytold/vectorcraft) | MIT OR Apache-2.0 |
| LICENSE-MIT, LICENSE-APACHE (texto da licença) | CADCraft | — |
| .github/workflows/release.yml, .github/workflows/packaging-lint.yml e packaging/ (pipeline de versões: Linux AppImage/deb/rpm/Flatpak/tar.gz, FreeBSD, macOS dmg, Windows MSI/zip portátil, zip web; modelos AppRun, nfpm, Flatpak, WiX e Info.plist) | PhotoCraft (https://github.com/storytold/photocraft) `.github/workflows/{release,packaging-lint}.yml`, `packaging/` | Apache-2.0 |

## Adaptador incremental de codec HDR

`crates/render/src/lighting/hdr.rs` adapta as máquinas de estado de decodificação scanline/component/old-RLE
do decodificador Radiance de `image` 0.25.10
([fonte](https://docs.rs/crate/image/0.25.10/source/src/codecs/hdr/decoder.rs)),
MIT OR Apache-2.0. A leitura dos metadados ainda usa essa biblioteca. O adaptador torna cooperativo o trabalho
por linha e adiciona comprimentos de sequências verificados, rejeição de sequências de comprimento zero, validação de largura,
limites de cabeçalho/entrada/radiância e cancelamento. Os textos das licenças originais são mantidos
em THIRD-PARTY-LICENSES.txt; nenhuma imagem ou arte desse projeto é copiada.

## Contribuições recuperadas

| Arquivo do SolidCraft | Contribuição original | Licença |
|---|---|---|
| crates/ui-egui/src/viewport.rs (`nearest_quarter_turn`, `axis_view_orbit`) | [PR #101](https://github.com/bherbruck/solidcraft/pull/101) de N3MI-DG: [rotações mínimas TOP/BOTTOM](https://github.com/bherbruck/solidcraft/commit/c28e780f52490784ef5886f094b9d3f6d10c3e3f) e [arraste para orbitar](https://github.com/bherbruck/solidcraft/commit/f30b30be6ecc4e2e6544a851243137c01a354934), adaptadas ao manipulador de eixos | MIT OR Apache-2.0 |

## Código de terceiros incorporado

| Diretório | Fonte | Licença | Alterações |
|---|---|---|---|
| vendor/truck-meshalgo | truck-meshalgo 0.4.0, https://github.com/ricosjp/truck (RICOS Co. Ltd.) | Apache-2.0 | geração de malha: buscas de parâmetros limitadas, projeção de contornos colapsados, subdivisão e prévias provisórias de grade fixa (vendor/README.md) |
| vendor/truck-stepio | truck-stepio 0.3.0, https://github.com/ricosjp/truck (RICOS Co. Ltd.) | Apache-2.0 | gravador STEP: superfícies de curvas de interseção gravadas com IDs únicos (vendor/README.md) |
| vendor/truck-shapeops | truck-shapeops 0.4.0, https://github.com/ricosjp/truck (RICOS Co. Ltd.) | Apache-2.0 | geração robusta de malha e alternativa de parâmetro mais próximo em operações booleanas; rejeição de curvas de interseção degeneradas; histórico de faces e arestas de operações booleanas; macro de rastreamento; escolha de canto em vértices onde várias curvas de interseção terminam (vendor/README.md) |
| vendor/matext4cgmath | matext4cgmath 0.1.0, https://github.com/ricosjp/matext4cgmath (RICOS Co. Ltd.) | Apache-2.0 | o refinamento de raízes para após 100 etapas de Newton (vendor/README.md) |

## Correção de compatibilidade do renderizador incorporado

`vendor/egui-wgpu` é egui-wgpu 0.36.2 de https://github.com/emilk/egui
(MIT OR Apache-2.0; os dois textos de licença originais estão incluídos). Os códigos-fonte Rust e
seu shader original `egui.wgsl` são mantidos; nenhuma imagem ou outra arte é copiada.
A correção pontual do SolidCraft seleciona um caminho MSAA 1x seguro no adaptador OpenGL escolhido
e compartilha essa contagem efetiva com os anexos de superfície e os callbacks CAD.
Vulkan, DirectX 12, Metal e WebGPU mantêm a multiamostragem solicitada.
