# Compatibilidade com o nome SolveCraft

SolidCraft inicialmente se chamava SolveCraft, e compilações até 0.4 mantiveram esse nome em seus
identificadores após a mudança do nome exibido. A versão 0.5.0 renomeou os binários, arquivos de versão e
pacotes; a versão seguinte renomeia todo o restante: crates `solidcraft-*`, binários `solidcraft` e `solidcraft-cli`, projetos
`.solidcraft`, a etiqueta `"format": "solidcraft/N"`, variáveis de ambiente `SOLIDCRAFT_*`, pastas de configurações
e recuperação, o nome do servidor MCP, documentação, CI e empacotamento.

O nome antigo permanece **somente** nas camadas de compatibilidade abaixo (além de seus testes, deste arquivo e do histórico /
notas de versão). `git grep -i solvecraft` não deve listar mais nada. Ao adicionar ou remover uma
camada de compatibilidade, atualize este arquivo na mesma PR.

## 1. Arquivos de projeto `.solvecraft`

Novos projetos são salvos como `.solidcraft`. Arquivos com a extensão antiga continuam abrindo em todos os lugares:

| Caminho de entrada | Onde |
|---|---|
| Extensão → formato de projeto | `crates/io/src/lib.rs` (`LEGACY_DESIGN_EXT`, detecção de formato) |
| Filtros do diálogo Abrir (desktop) | `apps/solidcraft/src/main.rs` (`services`) |
| Arrastar e soltar, arquivos recentes, `doc.open`, MCP `open`, CLI `eval`/`run` | passam pela mesma detecção de formato em `solidcraft_io` |
| Seletor de arquivos do navegador | `apps/solidcraft-web/src/files.rs` (`accept`) |
| Windows | `assets/platforms/windows/solidcraft.wxs`: a extensão `.solvecraft` no ProgId do projeto e em `SupportedTypes` |
| macOS | `assets/platforms/macos/Info.plist.in`: `solvecraft` nas extensões do tipo de documento de projeto |
| Linux / FreeBSD / Flatpak | `assets/platforms/linux/ai.storyteller.solidcraft.mime.xml`: padrão glob `*.solvecraft` e o alias `application/x-solvecraft` |

Entradas de arquivos recentes que apontam para um arquivo `.solvecraft` continuam funcionando: são caminhos simples e
abrem pela mesma detecção. O aplicativo de navegador também lista projetos que armazenou com a extensão
antiga, e arquivos de atalhos de teclado exportados como `solvecraft-shortcuts/1` continuam sendo importados
(`crates/ui-egui/src/toolbar_custom.rs`). Testes: `crates/io` (`Format::from_name`,
`is_design_path`), `legacy_extension_and_format_tag_still_open` (CLI, `run_results.rs`) e
`legacy_design_files_and_resource_uris` (MCP `open`).

## 2. A etiqueta de formato `solvecraft/N`

Os projetos registram seu formato como `"format": "solidcraft/N"`. Arquivos salvos por compilações até 0.5 usam
`"format": "solvecraft/N"`; `crates/doc/src/format.rs` (`version_of`) lê ambos os prefixos e
os atualiza normalmente (o arquivo recebe novamente a etiqueta `solidcraft/N` ao ser salvo).

Não há compatibilidade com versões anteriores: compilações até 0.5 não conhecem a nova etiqueta e rejeitam arquivos salvos por
compilações posteriores. Teste: `version_of` aceita ambos os prefixos; projetos com etiqueta legada carregam e recebem a
nova etiqueta ao serem salvos.

## 3. Variáveis de ambiente `SOLVECRAFT_*`

Todas as variáveis de execução são lidas por um único auxiliar, `solidcraft_geom::env` (`var`, `var_os`):
ele lê `SOLIDCRAFT_<NAME>` e usa `SOLVECRAFT_<NAME>` como alternativa quando a nova variável não está definida.
Os pontos de entrada do shell fazem o mesmo: `packaging/linux/AppRun`
(`SOLIDCRAFT_NO_DESKTOP_INTEGRATION`) e `packaging/env.sh` (`SOLIDCRAFT_VERSION`,
`SOLIDCRAFT_BUILD_DATE`). Testes: testes unitários do auxiliar (a nova tem prioridade, a antiga é a alternativa,
nenhuma → não definida).

## 4. Pastas de configurações, preferências, salvamento automático e recuperação

Novas pastas usam o nome SolidCraft:

| | Linux | macOS | Windows |
|---|---|---|---|
| Preferências, configurações gráficas | `~/.local/share/solidcraft` | `~/Library/Application Support/SolidCraft` | `%APPDATA%\SolidCraft\data` |
| Recuperação (salvamento automático) | `…/solidcraft/recovery` | `…/SolidCraft/recovery` | `%LOCALAPPDATA%\SolidCraft\recovery` |

Na primeira inicialização, se a nova pasta não existir, mas a antiga (`solvecraft` / `SolveCraft`)
existir, ela é **copiada** para o novo local; a pasta antiga nunca é excluída ou alterada. O
estado da janela e da interface que o eframe mantinha em `ai.storyteller.solvecraft` é copiado da mesma forma.
Salvamentos automáticos copiados da pasta antiga (`<id>.solvecraft`) são listados e recuperados como os novos.
A chave de preferências no armazenamento do eframe é `solidcraft.prefs`; `solvecraft.prefs` é lida quando
ela está ausente (desktop e navegador, `solidcraft_ui_egui::prefs::stored`). O aplicativo de navegador também
lê arquivos e o salvamento automático mantidos com os nomes antigos no armazenamento local e no OPFS
(`apps/solidcraft-web/src/files.rs`). Um perfil 3DxWare que o driver criou para o nome antigo do
executável é lido até que ele crie um novo (`apps/solidcraft/src/space_mouse.rs`).
No sandbox do Flatpak, essas pastas ficam em `~/.var/app/<app id>/data`; para dados mantidos com
o ID do aplicativo de versões anteriores a 0.8, veja [ID do aplicativo Linux](compatibility.md#id-do-aplicativo-linux).
O cache de miniaturas não é migrado; ele é reconstruído. O auxiliar de cópia é
`solidcraft_io::adopt_legacy_dir`, usado pelo aplicativo para desktop (`adopt_legacy_settings`) e
por `solidcraft_engine::recovery::default_dir`. Testes: `legacy_dir` (copiar uma vez, nunca sobrescrever,
pasta antiga intacta), `first_launch_adopts_the_previous_release_settings`,
`autosave_from_a_legacy_folder_is_listed_and_loads`, `recovery_dirs_and_their_legacy_names`,
`stored_prefs_fall_back_to_the_legacy_key`.

## 5. Alias `solvecraft-cli`

Um pequeno binário `solvecraft-cli` (`apps/solidcraft-cli/src/alias.rs`) é distribuído em todos os pacotes por
pelo menos uma versão (desde 0.5.0). Ele exibe uma nota de descontinuação de uma linha em stderr e executa o
`solidcraft-cli` ao seu lado com os mesmos argumentos, stdin/stdout e código de saída, para que scripts
e configurações de clientes MCP que chamam `solvecraft-cli` continuem funcionando. Teste: `apps/solidcraft-cli/tests/legacy_alias.rs` (encaminha `--version` e o estado de saída;
a nota vai apenas para stderr, mantendo stdout limpo para o MCP).

Não há alias para o binário de desktop: lançadores, associações de arquivos e entradas no menu Iniciar
instalados pelos pacotes apontam para `solidcraft`.

## 6. MCP

`serverInfo.name` agora é `solidcraft`. O nome exibido por um cliente é escolhido pelo usuário quando o
servidor é adicionado, portanto uma configuração existente, como

```sh
claude mcp add solvecraft -- solvecraft-cli mcp
```

continua funcionando (pelo alias acima). Novas configurações usam:

```sh
claude mcp add solidcraft -- solidcraft-cli mcp
```

Faça a mudança antes de o alias ser removido. Os recursos são listados como `solidcraft://design` e
`solidcraft://commands`; os URIs antigos `solvecraft://` continuam legíveis (`crates/mcp/src/server.rs`,
teste `legacy_design_files_and_resource_uris`).

## IDs de pacotes

| Pacote | Decisão | Motivo |
|---|---|---|
| ID de aplicativo Flatpak / ID de desktop Linux | alterado em 0.8 de `ai.storyteller.solvecraft` para `ai.storyteller.solidcraft` | O ID que o lançador ArtCraft procura (`ai.storyteller.<name>`). Os nomes dos arquivos `.desktop`, metainfo e MIME, o nome do ícone e o `app_id` Wayland / classe X11 o acompanham. Instalações existentes são migradas; veja [ID do aplicativo Linux](compatibility.md#id-do-aplicativo-linux). |
| Arquivo de repositório remoto Flatpak | `solidcraft.flatpakrepo`, além de uma cópia legada `solvecraft.flatpakrepo` | Links antigos continuam funcionando; repositórios remotos existentes apontam para a URL do repositório, que não mudou. |
| ID de bundle macOS | alterado para `ai.storyteller.solidcraft` | Nenhum atualizador depende dele; o aplicativo é instalado por arraste. O macOS pode perguntar novamente na primeira inicialização (compilações não assinadas). |
| `UpgradeCode` MSI | **mantido** | Para que o novo MSI atualize a instalação antiga no mesmo local. O nome do produto é SolidCraft; a pasta padrão passa a ser `Program Files\SolidCraft` (a atualização principal remove a pasta antiga). Uma pasta de instalação personalizada não é lembrada nesta atualização. |
| Nome de pacote deb / rpm | renomeado para `solidcraft`; deb `Replaces`/`Conflicts`/`Provides: solvecraft`, rpm `Obsoletes`/`Provides: solvecraft` | Instalar o novo pacote substitui o antigo, em vez de instalar lado a lado. |
| AppImage | `solidcraft-<version>-linux-<arch>.AppImage`; a versão também contém uma cópia de seu `.zsync` com o nome antigo `solvecraft-…` | AppImages já instalados procuram o `.zsync` com o nome antigo, portanto o AppImageUpdate continua encontrando atualizações. |

## ID do aplicativo Linux

Versões até 0.7 usavam `ai.storyteller.solvecraft`; 0.8 usa `ai.storyteller.solidcraft`
em todo lugar (Flatpak, deb, rpm, AppImage, Snap, FreeBSD, Nix). As camadas de compatibilidade:

| Camada de compatibilidade | Onde | O que faz |
|---|---|---|
| Rebase de fim de vida no repositório Flatpak | `packaging/flatpak-site.sh` | A implantação do Pages de cada versão mantém as referências antigas `app/ai.storyteller.solvecraft/<arch>/stable` (espelhadas da implantação anterior) e as marca uma vez com `flatpak build-commit-from --end-of-life-rebase=ai.storyteller.solvecraft=ai.storyteller.solidcraft`. `flatpak update` então informa que o aplicativo antigo chegou ao fim de vida e oferece substituí-lo pelo novo; aceitar instala `ai.storyteller.solidcraft`, desinstala o aplicativo antigo, e o Flatpak move `~/.var/app/ai.storyteller.solvecraft` para a pasta do novo ID (deixando um link simbólico no caminho antigo) na primeira inicialização, mantendo configurações e salvamentos automáticos inalterados. |
| AppStream `<replaces>` / `<provides>` | `assets/platforms/linux/ai.storyteller.solidcraft.metainfo.xml.in` | As centrais de software tratam o novo ID como sucessor do antigo e de `ai.storyteller.solvecraft.desktop`. |
| `X-Flatpak-RenamedFrom` | `assets/platforms/linux/ai.storyteller.solidcraft.desktop` | Ambientes de desktop que o respeitam (GNOME Shell) mantêm favoritos e lançadores fixados apontando para a entrada renomeada. |
| Acesso somente de leitura à pasta antiga de dados do Flatpak | `--filesystem=~/.var/app/ai.storyteller.solvecraft:ro` em `packaging/linux/flatpak/ai.storyteller.solidcraft.bundle.yml`; `solidcraft_io::legacy_flatpak_path`, usado por `adopt_legacy_settings` e `recovery::default_dir` | Quando o novo aplicativo foi instalado ao lado do antigo (por exemplo, a partir de um bundle `.flatpak`, que não tem canal de atualização), em vez de pelo rebase, sua primeira inicialização copia configurações e salvamentos automáticos da pasta do aplicativo antigo, como na seção 4. A pasta antiga não é alterada. |
| O `app_id` antigo da janela como chave legada de armazenamento do eframe | `adopt_legacy_settings` (`LEGACY_APP_ID`) | O estado da janela que versões anteriores à renomeação das configurações mantinham em `~/.local/share/ai.storyteller.solvecraft` ainda é copiado (seção 4). |
| Entradas antigas de menu AppImage | `packaging/linux/AppRun` | A primeira execução de um AppImage remove a entrada `ai.storyteller.solvecraft.desktop` e os ícones que um AppImage até 0.7 instalou em `~/.local/share`, para que o menu não mostre o aplicativo duas vezes. |

deb e rpm não precisam de uma camada de compatibilidade: o pacote (`solidcraft`) não mudou, e uma atualização remove os antigos
`/usr/share/applications/ai.storyteller.solvecraft.desktop`, metainfo, MIME e arquivos de ícone
porque a nova versão não os distribui; o script de pós-instalação atualiza os caches de desktop, MIME
e ícones, sem deixar uma entrada de menu desatualizada. O Snap não é afetado, além de seu
`common-id`: snapd nomeia sua entrada no menu a partir do snap.

Flatpak instalado a partir de um bundle `.flatpak` baixado (sem canal de atualização): instale o novo bundle,
inicie-o uma vez (configurações e salvamentos automáticos são copiados) e então execute
`flatpak uninstall --user ai.storyteller.solvecraft`.

**Remoção:** mantenha as referências de fim de vida no repositório Flatpak por pelo menos seis meses após
0.8.0 (até 2027-04) e por pelo menos duas versões secundárias; removê-las deixa sem atualização qualquer instalação que
não tenha executado `flatpak update` desde então. A permissão de sistema de arquivos somente de leitura e `legacy_flatpak_path` podem ser removidas
ao mesmo tempo. `<replaces>`, `<provides>` e `X-Flatpak-RenamedFrom` são inofensivos e podem permanecer.
Testes: `flatpak_twin_maps_our_sandbox_data_to_the_old_app_id`,
`first_launch_adopts_the_previous_release_settings`.

## macOS

Versões anteriores a 0.4 eram instaladas como `SolveCraft.app`. Para atualizar, feche o aplicativo, arraste
`SolidCraft.app` do DMG para Aplicativos, mova `SolveCraft.app` para o Lixo e
substitua qualquer atalho do Dock com o nome antigo (abra `SolidCraft.app` e escolha
**Opções > Manter no Dock**). Projetos e configurações são mantidos (veja 4).

## Repositórios remotos Flatpak adicionados pelo endereço antigo do site

`flatpak remote-modify --user --url=https://bherbruck.github.io/solidcraft/flatpak/repo/ solvecraft`
(use o nome de seu repositório remoto), depois `flatpak update --user`.

## Contêineres de projeto de rascunho

O rascunho #303 usava `solvecraft-container/1`. `crates/io/src/design.rs` aceita esse
marcador junto com `solidcraft-container/1` e grava apenas o novo marcador. Prévias antigas
opcionais são descartadas; a validação obrigatória de documento/blobs não mudou. A
regressão `draft_container_marker_remains_readable` cobre essa compatibilidade.
