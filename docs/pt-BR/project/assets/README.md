# Recursos do aplicativo

`assets/` é o local dos recursos de design do SolidCraft.
[ATTRIBUTION.md](../../ATTRIBUTION.md) é o inventário de arquivos e registro de proveniência.
Mantenha um original editável por recurso e coloque os derivados distribuídos em `exports/`.

```text
assets/
  icons/
    app/
      shared/source/                 # original Rust mark and executable ICO encoder
      macos/source/SolidCraft.icon/   # approved Composer project and linked SVGs
      macos/exports/                 # paired Assets.car and icon.icns
      windows/exports/               # executable DIB ICO and installer PNG ICO
      linux/exports/hicolor/         # Linux/FreeBSD package images, 16–512 px
    ui/source/                       # original drawing library and other UI glyphs
  branding/web/source/og.png          # shared social preview card
  styles/ui/palettes.rs               # light/dark defaults
  styles/web/loading.css             # web loading/canvas styles
  appearances/library.rs             # built-in visual presets
  materials/library.rs               # physical defaults and colours
  shaders/viewport.wgsl               # original viewport shader
  fonts/{liberation-sans,biz-udp-gothic}/ # unmodified fonts and OFL licences
  locales/{en,de,fr,ja,ko,ru,uk,zh,pt-BR}.json # shared offline catalogs
  platforms/{macos,linux,windows}/    # bundle, freedesktop and WiX templates
  archive/icons/macos-v1/
    source/icon.vectorcraft          # earlier editable work, retained
    exports/                         # all historical SVG, PNG and iconset exports
```

## Edição e regeneração

A captura de janela usada como fonte no manual em português fica em
`documentation/screenshots/source/sample-plate-pt-BR.png`. Capture-a com
`solidcraft --sample --language pt-BR`, enquadre a vista em uma janela de 1200×800
e mantenha os nomes definidos no exemplo. `cargo xtask book` copia essa fonte
com atribuição para o manual em português; capturas temporárias de revisão da PR
ficam em `plan/`.

[Arte da interface](icons/ui/README.md) identifica os originais de desenho e quem os utiliza.
Edite geometria e IDs juntos na fonte Rust original e revise as mudanças nos
dois temas do aplicativo. Paletas e predefinições do modelo também são dados Rust editáveis.

O original macOS atual é `icons/app/macos/source/SolidCraft.icon/`, incluindo
suas camadas SVG vinculadas e configurações de aparência. O documento VectorCraft anterior
é preservado no arquivo histórico. Nenhum outro projeto nativo de design estava disponível para
o conjunto atual da interface ou cartão de prévia. Mantenha futuros arquivos `.vectorcraft`, `.ai` ou outros
trabalhos nativos e entradas vinculadas em `source/work/` desse recurso, com uma
linha de atribuição; preserve o documento editável junto com suas exportações.

Gere novamente todas as exportações da marca compartilhada Windows/Linux/FreeBSD juntas:

```sh
cargo xtask app-icons
```

Para macOS, use Xcode com suporte a Icon Composer:

```sh
mkdir -p target/resources/macos-icon
xcrun actool assets/icons/app/macos/source/SolidCraft.icon \
  --compile target/resources/macos-icon --platform macosx \
  --minimum-deployment-target 11.0 --app-icon SolidCraft \
  --output-partial-info-plist target/resources/macos-icon/icon-info.plist \
  --output-format human-readable-text
cp target/resources/macos-icon/Assets.car assets/icons/app/macos/exports/Assets.car
cp target/resources/macos-icon/SolidCraft.icns assets/icons/app/macos/exports/icon.icns
```

Revise e faça commit das alterações de fontes e das exportações necessárias juntas. Renderizações temporárias,
caches de ferramentas e preparação de pacotes pertencem a `target/` ou `dist/` ignorados.
Compilações normais usam as exportações versionadas e não exigem aplicativos de design.
A CLI ainda aceita exportações arbitrárias PNG/ICO com `solidcraft-cli icon`.

## Vinculações de plataforma

| Plataforma | Recursos e caminhos de saída |
| --- | --- |
| macOS | Original Composer e exportações correspondentes; o pacote instala `Assets.car` e `SolidCraft.icns` |
| Executável Windows | `solidcraft-executable.ico`: representações originais DIB/máscara em 16/24/32/48/64/128/256 px |
| Instalador Windows | `solidcraft.ico`: representações PNG em 16/20/24/32/40/48/64/128/256 px; substituição `SOLIDCRAFT_ICONS` preservada |
| Linux, Nix, deb/rpm, tar, Flatpak, Snap | Copiar imagens hicolor canônicas; o nome do ícone permanece `ai.storyteller.solidcraft` |
| AppImage | Usa a imagem Linux de 256 px e o alias `.DirIcon` |
| FreeBSD | Compartilha imagens hicolor do Linux e metadados freedesktop |
| Web | HTML/Trunk permanece em `apps/solidcraft-web/`; copia fonte/licença japonesa para `fonts/` e usa o CSS reunido; atualmente não há favicon/ícone PWA |
| Documentação e prévias de links | `xtask book` copia o cartão compartilhado para o caminho público existente `/solidcraft/og.png` |

## Código e proveniência

Recursos continuam compilados por sua camada responsável pelas APIs existentes:
`ui-egui::icons::paint` e `Tokens` em L5, predefinições de modelo em L2 `doc` e fontes de
esboço em L1. Estado, projeção, interação, comandos e vinculações GPU permanecem
nos módulos atuais. Dados incorporados usam `CARGO_MANIFEST_DIR`; o código do aplicativo
não precisa de registro de recursos em tempo de execução nem consulta ao repositório.

`cargo xtask assets` exige uma linha exata de atribuição para cada arquivo reunido,
exceto documentação Markdown, e continua verificando arquivos de recursos em outros locais do
repositório. Isso inclui trabalhos nativos, originais Rust, catálogos, shaders e
modelos de plataforma. O empacotamento seleciona arquivos atuais explícitos e falha quando
entradas necessárias estão ausentes. A arte arquivada não é usada em produção.

Fontes do egui e shaders do renderizador pertencentes a dependências permanecem com suas
bibliotecas; as licenças continuam em `THIRD-PARTY-LICENSES.txt` e `ATTRIBUTION.md`.
Fontes do sistema, pacotes opcionais, fontes/imagens de referência/decalques/materiais do usuário e controles nativos
do sistema operacional mantêm propriedade externa. Textos de licença/aviso da raiz permanecem canônicos.
