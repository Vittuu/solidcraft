# SolidCraft com Nix

A flake suporta `x86_64-linux`, `aarch64-linux` e `aarch64-darwin`. Habilite os recursos experimentais `nix-command`
e `flakes` do Nix. O `flake.lock` versionado fixa nixpkgs e
sua cadeia de ferramentas Rust; a avaliação do pacote verifica esse compilador em relação à versão mínima
do Rust em `Cargo.toml`.

## Executar, compilar ou instalar

Execute o aplicativo atual sem instalá-lo:

```sh
nix run github:bherbruck/solidcraft#solidcraft -- --sample
```

Dentro de uma cópia do repositório, `nix run .` e `nix run .#solidcraft` iniciam o aplicativo para desktop.
`nix run .#cli -- --version` inicia a CLI. O nome legado de aplicativo `.#solidcraft`
também funciona. Esses comandos usam o binário exato encapsulado da flake, em vez de um
executável antigo já disponível em `PATH` no shell.

Compile sem instalar:

```sh
nix build github:bherbruck/solidcraft#solidcraft
./result/bin/solidcraft --version
./result/bin/solidcraft --sample
```

O resultado contém `solidcraft`, `solidcraft-cli` e o alias de CLI descontinuado com o nome antigo
(veja [compatibility.md](compatibility.md)). **Compilar não instala uma entrada no menu do desktop.** Para uma instalação
no perfil do usuário, execute:

```sh
nix profile install github:bherbruck/solidcraft#solidcraft
solidcraft --sample
```

No Linux, isso instala a entrada de desktop do SolidCraft, a definição MIME e os tamanhos de ícones gerados
junto com os comandos. No macOS, o pacote contém apenas os binários; para um
`SolidCraft.app` empacotado, use o `.dmg` da versão. NixOS/Home Manager podem, em vez disso, colocar
`packages.${system}.solidcraft` da flake em `environment.systemPackages`/`home.packages`.
Se um desktop que não seja NixOS não descobrir lançadores instalados pelo perfil, certifique-se de que
sua sessão inclua `~/.nix-profile/share` em `XDG_DATA_DIRS`, depois saia e
entre novamente. O ID de desktop/aplicativo não mudou (veja
[compatibility.md](compatibility.md#ids-de-pacotes)).

Confira qual executável um comando simples no terminal executaria:

```sh
type -a solidcraft
readlink -f "$(command -v solidcraft)"
solidcraft --version
nix run github:bherbruck/solidcraft#solidcraft -- --version
```

Um executável antigo não relacionado em `PATH` não é atualizado por `nix build`.
Use o aplicativo da flake ou `./result/bin/solidcraft` para verificar o pacote que acabou de compilar.

## Gráficos e diálogos de arquivos

O wrapper do aplicativo inclui bibliotecas Wayland, X11, xkbcommon, EGL, Vulkan-loader e D-Bus.
Ele expõe `/run/opengl-driver/lib` e
`/run/opengl-driver/share` para que os drivers gráficos e manifestos Vulkan do NixOS possam ser
descobertos. Habilite `hardware.graphics.enable = true` no NixOS e mantenha a
pilha de drivers do sistema consistente com o compositor. Seletores de arquivos também precisam de um
D-Bus de sessão funcional e do serviço `xdg-desktop-portal` do desktop.

No macOS (`aarch64-darwin`), nada disso se aplica: winit comunica-se com AppKit, wgpu renderiza por
Metal, e os drivers de GPU vêm com o sistema operacional, portanto o pacote darwin não contém
wrapper de bibliotecas nem caminhos adicionais de busca.

Em outra distribuição, drivers gráficos fora do armazenamento Nix podem precisar de um
[wrapper nixGL](https://github.com/nix-community/nixGL) compatível. Siga suas
instruções específicas para a GPU; um wrapper para outro driver não o substitui.
Para Intel/Mesa, após compilar:

```sh
nix run github:nix-community/nixGL#nixGLIntel -- ./result/bin/solidcraft --sample
```

Se uma janela estiver preta, primeiro confira se iniciou o pacote recém-encapsulado
e se seu sistema expõe um driver de GPU funcional. `WGPU_BACKEND=vulkan`
seleciona Vulkan para diagnóstico; um carregador sem driver utilizável ainda não consegue renderizar.
A renderização OpenGL usa suavização de serrilhado 1x para evitar a falha de quadros pretos com multiamostragem.
Vulkan e outros backends mantêm a contagem de amostras solicitada; preferências salvas são preservadas.
Para compilações antigas, incluindo 0.3.0, defina Suavização de serrilhado como 1x e reinicie ao usar OpenGL.
Se essa janela não conseguir mostrar o diálogo de configurações, edite somente o campo `msaa` para `1` em
`~/.local/share/solidcraft/graphics.json`; preserve todas as outras configurações.

## Wayland, XWayland e niri

No Linux, o aplicativo usa XWayland sempre que uma tela X está disponível, porque
winit só suporta arrastar arquivos de um gerenciador de arquivos para a janela em X11. No
niri (detectado por `XDG_CURRENT_DESKTOP` com um socket Wayland presente), ele permanece em
Wayland nativo, pois ações de gerenciamento de janela, como maximizar/restaurar, funcionam incorretamente
no XWayland do niri. A contrapartida: **no Wayland nativo, soltar arquivos na
janela não funciona**; use Arquivo > Abrir ou forneça o arquivo pela linha de comando.
Isso se aplica a todas as compilações, não apenas ao pacote Nix.

Substitua a escolha com `SOLIDCRAFT_WAYLAND=1` (sempre Wayland nativo) ou
`SOLIDCRAFT_WAYLAND=0` (XWayland sempre que uma tela X estiver disponível).

## Shells de desenvolvimento

`nix develop` fornece bibliotecas de execução e usa a cadeia de ferramentas Rust/C já disponível em
seu `PATH`. `nix develop .#rust` fornece a cadeia de ferramentas fixada, o vinculador C e
o diretório de compilação `target/nix`. Ambos mantêm os nomes existentes dos pacotes Cargo da interface gráfica
e da CLI (`solidcraft`, `solidcraft-cli`).
