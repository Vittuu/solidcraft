# Primeiros passos

O executável para desktop é `solidcraft`, a CLI é `solidcraft-cli`, e os projetos são salvos como
`.solidcraft`. Projetos, preferências e configurações de versões publicadas com o nome anterior do produto continuam
funcionando (veja as [notas de compatibilidade](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/compatibility.md)).

## Desktop

Baixe o instalador para seu sistema na
[seção Download do README](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/README.md#download) ou na
[página de versões](https://github.com/bherbruck/solidcraft/releases), ou compile por conta própria com
Rust estável (1.95 ou mais recente):

```sh
git clone https://github.com/bherbruck/solidcraft && cd solidcraft
cargo run --release -p solidcraft -- --sample   # opens with a sample part
```

`solidcraft part.step` abre um arquivo STEP, IGES, 3MF, STL ou OBJ como um novo projeto, e
`solidcraft design.solidcraft` abre um projeto salvo. Versões para Windows podem ser compiladas a partir do Linux com
`cargo xwin build --release --target x86_64-pc-windows-msvc -p solidcraft`.

### macOS

Abra o `.dmg` da versão e arraste `SolidCraft.app` para Aplicativos.

Ao atualizar uma versão publicada com o nome anterior do produto, siga as
[notas de atualização para macOS](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/compatibility.md#macos) para substituí-la em vez de instalar uma segunda cópia.

### Linux Flatpak

Instale o Flatpak usando o gerenciador de pacotes da sua distribuição. Para uma instalação que receba
atualizações, adicione o Flathub para o ambiente de execução Freedesktop necessário e o repositório do aplicativo SolidCraft
à mesma instalação de usuário:

```sh
flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak remote-add --user --if-not-exists solidcraft https://bherbruck.github.io/solidcraft/flatpak/solidcraft.flatpakrepo
flatpak install --user solidcraft ai.storyteller.solidcraft
flatpak run ai.storyteller.solidcraft
```

As versões até a 0.7 usavam o ID de aplicativo `ai.storyteller.solvecraft`. Se você instalou uma
delas pelo repositório do SolidCraft, `flatpak update --user` informa que ela chegou ao fim de vida e oferece
a substituição por `ai.storyteller.solidcraft`; aceite, e suas preferências e salvamentos automáticos serão transferidos.
Se você instalou um pacote `.flatpak`, instale o novo pacote, execute-o uma vez (ele copia suas
preferências) e depois execute `flatpak uninstall --user ai.storyteller.solvecraft` (veja as
[notas de compatibilidade](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/compatibility.md#id-do-aplicativo-linux)).

Como alternativa, baixe o arquivo `.flatpak` da versão para sua arquitetura e execute
`flatpak install --user ./solidcraft-VERSION-linux-ARCH.flatpak`. O pacote fornece o
endereço do repositório de ambientes de execução do Flathub; a instalação ainda exige acesso à internet para baixar qualquer
ambiente de execução ausente. Inicie com `flatpak run ai.storyteller.solidcraft`, em vez de procurar
um binário `solidcraft` no sistema hospedeiro. Arquivos escolhidos por Abrir/Salvar usam o portal de seleção de arquivos do desktop;
caminhos de linha de comando fora de Documentos podem exigir uma permissão adicional específica de acesso ao sistema de arquivos.

### Linux Snap

No Ubuntu e em outras distribuições com snapd:

```sh
sudo snap install solidcraft
solidcraft                # or open SolidCraft from the app menu
solidcraft.cli --version  # the CLI, for scripts and `solidcraft.cli mcp`
```

O snap pode abrir projetos na sua pasta pessoal (exceto pastas ocultas) e, pelo seletor de arquivos,
em qualquer outro local. Para unidades USB e outros discos montados, execute
`sudo snap connect solidcraft:removable-media` uma vez. O arquivo `.snap` de uma versão pode ser instalado sem
a loja com `sudo snap install --dangerous ./solidcraft-VERSION-linux-ARCH.snap`; nesse caso,
ele não recebe atualizações automáticas.

## Web

O mesmo aplicativo funciona no navegador: [abra o SolidCraft na web](https://bherbruck.github.io/solidcraft/app/) (`?sample` abre a peça de exemplo,
e `?webgl` força WebGL2 onde WebGPU não está disponível). Os projetos são salvos no armazenamento privado
de arquivos do navegador, que exige HTTPS ou localhost. A integração planejada de workers e armazenamento em camadas
está descrita em [web-runtime.md](https://github.com/bherbruck/solidcraft/blob/main/docs/web-runtime.md)
(proposta; ainda não integrada à compilação wasm distribuída).

Após a primeira visita, o aplicativo web funciona offline: o navegador mantém uma cópia dele. Quando há conexão, cada
inicialização obtém a compilação publicada mais recente (o navegador a baixa uma vez quando ela muda); sem conexão,
quando a rede não responde em alguns segundos ou quando não é possível baixar uma compilação nova
por inteiro, a cópia armazenada é iniciada (e o download continua para a próxima inicialização). Para compilá-lo
por conta própria:

```sh
rustup target add wasm32-unknown-unknown
cd apps/solidcraft-web && trunk serve --release --no-autoreload
```

O uso offline exige `--no-autoreload`: a recarga automática do Trunk altera a página ao servi-la, e por isso ela
deixa de corresponder à compilação que o navegador armazenaria. Compilações de depuração (`trunk serve`) nunca armazenam uma cópia.

## CLI

`solidcraft-cli` executa os mesmos comandos sem uma janela:

```sh
cargo run --release -p solidcraft-cli -- commands                       # every command, as JSON
cargo run --release -p solidcraft-cli -- run examples/bracket.json --out bracket.step
cargo run --release -p solidcraft-cli -- eval examples/bracket.json     # volume, area, faces…
cargo run --release -p solidcraft-cli -- snapshot examples/bracket.json --out bracket.png
cargo run --release -p solidcraft-cli -- exec solid.box '{"length": 20, "width": 10, "height": 5}'
```

Um script é uma lista de comandos:

```json
{"commands": [
  {"command": "sketch.create", "params": {"plane": "XY"}},
  {"command": "sketch.rectangle.two_point", "params": {"p0": [0, 0], "p1": [40, 30]}},
  {"command": "sketch.finish", "params": {}},
  {"command": "solid.extrude", "params": {"distance": 20}},
  {"command": "solid.fillet", "params": {"edges": [[0, 0, 10]], "radius": 3}}
]}
```

![O exemplo de suporte, renderizado por solidcraft-cli snapshot](../generated/bracket.png)

`snapshot` usa a vista isométrica. `--view NAME` escolhe outra vista padrão (`front`, `back`,
`top`, `bottom`, `left`, `right`) ou uma vista nomeada salva no projeto com `view.save`.
`--camera JSON` aceita uma câmera arbitrária, como a ferramenta MCP `screenshot` (veja as
[convenções da câmera](mcp.md#câmera-e-roteiro-para-vistas-de-detalhe)). Um script que execute
`view.apply` também define a câmera. A grade fica em Z 0 e atravessa peças que cruzam esse plano:
`--no-grid` a omite da imagem, e `--no-axes` omite os eixos da origem.
A imagem é gerada na GPU com os mesmos pipelines da área de visualização, por isso mostra o que a janela mostraria
(incluindo uma análise de superfície como `inspect.zebra`). Uma máquina sem GPU usa um adaptador por software
(lavapipe do Mesa no Linux, WARP no Windows); sem ambos (um Mac sem Metal utilizável, ou Linux sem Vulkan do Mesa),
o rasterizador por CPU gera a imagem sem cores de análise de superfície, e a resposta informa `renderer: cpu` com um aviso.
`SOLIDCRAFT_GPU=auto|hardware|software|cpu` escolhe a política (`hardware` e `software` falham em vez de recorrer à CPU),
e `solidcraft-cli gpu` lista os adaptadores e o nível que uma captura usaria.

```sh
solidcraft-cli snapshot part.solidcraft --out photo.png --view Photo
solidcraft-cli snapshot part.solidcraft --out close-up.png --no-grid --no-axes --camera '{"target": [30, -20, 0], "yaw": 0.6, "pitch": 0.25, "distance": 60}'
```

O Windows PowerShell 5.1 remove as aspas duplas internas de um argumento passado a um programa:
nesse ambiente, escreva-as como `\"`.

Um comando que falha interrompe o script. Uma operação também pode falhar depois, quando uma alteração de parâmetro
a reconstrói; nesse caso, `run` termina com status 1, lista as operações com falha e não grava nenhum
dos arquivos de `--out` e `--save`, porque falta nos corpos o que essas operações deveriam ter criado.
`--allow-errors` grava os arquivos mesmo assim.

`run` exibe o resultado de cada comando com `--results` (caso contrário, a medição), e `eval`
exibe a medição como JSON em stdout. `--json-out FILE` grava esse JSON em um arquivo.
O arquivo é UTF-8 simples, algo que redirecionar não garante em todos os ambientes: `>` do
Windows PowerShell recodifica a saída como UTF-16 ou UTF-8 com marca de ordem de bytes,
conforme as configurações; leitores JSON estritos como `json.load` do Python a recusam.

```sh
cargo run --release -p solidcraft-cli -- run examples/bracket.json --results --json-out results.json
cargo run --release -p solidcraft-cli -- eval examples/bracket.json --json-out measures.json
```

O histórico de um projeto pode virar um script que o reconstrói, transformando um bug que só
aparece naquele projeto em uma reprodução de arquivo único:

```sh
cargo run --release -p solidcraft-cli -- script part.solidcraft --out repro.json
cargo run --release -p solidcraft-cli -- eval repro.json               # the same part
```

O script começa com os parâmetros do usuário (`parameters.add`) e as unidades, regras de chapa
metálica e plástico e materiais do projeto; depois adiciona cada operação da linha do tempo com `timeline.add`,
exatamente como o arquivo de projeto a armazena: mesmos IDs, nomes e nomes de parâmetros, para que as etapas seguintes
ainda encontrem os esboços e corpos que usam, e uma operação que falhou no projeto falhe
da mesma forma. Edite os valores de uma etapa para experimentar uma alteração ou exclua as etapas após uma operação para
ver o projeto como estava naquele ponto. Aparências, pastas, vistas nomeadas e configurações são
deixadas de fora (a CLI informa quais), e projetos com componentes ou juntas ainda não são
suportados. No aplicativo e pelo MCP, o mesmo script vem de `timeline.export_script`.

As unidades são milímetros e graus, a menos que uma expressão indique outra unidade (`"1 in"`, `"30 deg"`),
e Z aponta para cima.

## Nome e compatibilidade

O SolidCraft foi renomeado a partir de um nome de trabalho anterior. Desde a versão 0.5.0, tudo usa o novo
nome: os programas `solidcraft` e `solidcraft-cli`, os projetos `.solidcraft`, as variáveis de ambiente `SOLIDCRAFT_*`,
as pastas de preferências e o servidor MCP `solidcraft`. A atualização mantém o funcionamento:
projetos, preferências, arquivos de recuperação, variáveis de ambiente e configurações de CLI/MCP de
versões anteriores são preservados. Cada mecanismo de compatibilidade está listado nas
[notas de compatibilidade](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/compatibility.md).

Use as URLs da documentação e do aplicativo web do SolidCraft. O GitHub redireciona links para o repositório
renomeado; as URLs de Pages e Flatpak com o nome antigo não são mantidas como aliases permanentes.
