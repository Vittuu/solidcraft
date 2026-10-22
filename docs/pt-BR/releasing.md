# Publicando versões do SolidCraft

Publicações de versões são operações deliberadas que um agente pode concluir de ponta a ponta. O desenvolvimento entra
em `main`; enviar o commit selecionado para `release` compila todos os pacotes e cria um **rascunho**
de GitHub Release. Publicar esse rascunho verificado é uma operação separada. Enviar para `main` ou
enviar uma tag não inicia o fluxo de publicação. Não há incremento automático de versão baseado em mensagens
de commit nem uma versão de correção para cada merge.

## Nomes e implantação

Todos os arquivos de versão, binários, nomes de produto de instaladores e pacotes usam o nome SolidCraft
(`solidcraft-<version>-linux-<arch>.AppImage`, `SolidCraft-<version>-windows-x64.exe`,
arquivos de `solidcraft-cli`, …). O nome antigo permanece apenas nos mecanismos de compatibilidade listados em
[compatibility.md](compatibility.md) (as referências Flatpak em fim de vida do ID antigo do aplicativo Linux, as cópias legadas de `.zsync` e `.flatpakrepo`,
o alias obsoleto da CLI com o nome antigo); mantenha-os até que esse documento permita removê-los. Não
reescreva arquivos publicados, somas de verificação, tags nem notas históricas de versões anteriores.

Os instaladores principais (`.msi`, `.zip` portátil, `.dmg`, `.AppImage`, `.deb`, `.rpm`, `.flatpak`, `.snap`) também são
anexados sem a versão nos nomes, por exemplo `solidcraft-windows-x64.msi` e
`solidcraft-linux-x86_64.AppImage`, e listados em `SHA256SUMS.txt`. A seção Download do README
aponta para `https://github.com/bherbruck/solidcraft/releases/latest/download/<name>`, que segue
a versão marcada como mais recente; versões noturnas nunca recebem essa marca. Mantenha esses nomes estáveis: renomear um quebra os
links do README e a verificação de atualizações do aplicativo, que os escolhe pelo nome (`crates/ui-egui/src/update.rs`;
[atualizações e privacidade](book/src/guide/updates.md)).

O Pages do projeto publica a documentação, o aplicativo de navegador e o repositório Flatpak de versões
publicadas em `/solidcraft/`. Não adicione um publicador separado para o site do proprietário nem aliases permanentes
de Pages/Flatpak com o nome antigo. Redirecionamentos de repositório do GitHub são separados da hospedagem Pages;
verifique as novas URLs do projeto após a implantação.

O aplicativo de navegador armazena uma cópia de si para uso offline (`apps/solidcraft-web/sw.js`). Navegadores mantêm esse
service worker depois que o código que o registrou deixa de existir; portanto, para desativá-lo, implante
`apps/solidcraft-web/sw.js` sem compilar (como fazem as compilações `trunk serve`): ele exclui o que armazenou e
remove seu registro. Um site sem `build.json` também é seguro: as inicializações passam a usar a rede.

## Pré-lançamentos noturnos

`release.yml` também é executado à meia-noite em `America/New_York` (horário do leste dos EUA, acompanhando o horário
de verão). O GitHub pode iniciar execuções agendadas mais tarde quando seus executores estão ocupados.
Antes de compilar, ele compara o commit exato de `main` agendado com a tag da versão
publicada mais recentemente, incluindo pré-lançamentos. Se os commits forem iguais, todos os jobs de empacotamento e
publicação são pulados: nenhuma versão, tag ou release nova é criada. Rascunhos não contam
como versões publicadas, então uma compilação ou publicação que falhou pode ser repetida na noite seguinte.

Commits diferentes usam os jobs existentes de compilação e validação de pacotes. Somente depois que todos os jobs
passam, o fluxo prepara os arquivos em um rascunho e publica um pré-lançamento chamado
`v<workspace-version>-nightly.<YYYYMMDD>.<run-number>.<run-attempt>` com somas de verificação e notas
geradas. Qualquer sufixo existente de pré-lançamento do workspace é substituído pelo sufixo noturno. As datas
usam o horário do leste; versões dos pacotes são substituídas sem mudar Cargo.toml ou Cargo.lock,
de modo que a versão Cargo embutida nos binários continua sendo a do workspace. Versões noturnas não
substituem a versão estável mais recente nem avançam a branch `release`. O processo deliberado
normal abaixo continua criando rascunhos para verificação e publicação separadas.

A publicação noturna usa `GITHUB_TOKEN`, por isso não dispara a implantação subsequente Flatpak Pages.
O repositório Flatpak estável não é atualizado automaticamente com compilações noturnas.

## Escolher e preparar a versão

1. Busque `origin` e suas tags. Inspecione a versão publicada anterior e as mudanças desde
   ela. Escolha uma versão para o conjunto completo, com base no comportamento visível ao usuário:
   - Patch: correções compatíveis, melhorias de confiabilidade e correções de empacotamento.
   - Minor: novas capacidades compatíveis. Durante `0.x`, use também incremento minor para mudanças
     incompatíveis e explique a incompatibilidade e qualquer migração nas notas.
   - Major: comportamento público incompatível ou mudanças de formato de arquivo/API após `1.0`. Passar para
     `1.0` é uma decisão explícita de produto, não algo inferido de um prefixo de commit.
   - Use `-rc.1`, `-rc.2`, etc. quando uma versão candidata for solicitada.
   Mudanças apenas de documentação normalmente esperam a próxima versão. Informe nas notas o motivo
   do incremento escolhido. Siga qualquer versão ou escopo de publicação definido pelo proprietário.
2. Atualize `[workspace.package] version` no `Cargo.toml` da raiz. Depois execute
   `cargo update --workspace --offline` para atualizar as versões dos pacotes do workspace em
   `Cargo.lock` (isso altera apenas nossas próprias crates, nunca versões de dependências, e não exige
   rede). Inspecione o diff: somente linhas `version = ` das crates do workspace devem mudar.
   O SolidCraft atualmente não implementa `cargo xtask version set`.
3. Fixe o número do formato do projeto: execute `scripts/fold-format-pending.sh`. Ele move as alterações de
   formato pendentes (`crates/doc/src/format/pending/*.rs`) para a próxima pasta `crates/doc/src/format/v<N>/`,
   de modo que a versão publicada grave e leia esse número permanentemente, e não faz nada quando não há alterações
   pendentes. Faça commit dessa movimentação com os arquivos de versão. (CONTRIBUTING.md, "Alterar o formato dos arquivos de projeto".)
4. Execute `cargo xtask ci` e as verificações de empacotamento em `.github/workflows/packaging-lint.yml`.
   Faça commit dos arquivos de versão pelo fluxo normal de revisão e integre-os em `main`.
   Não use `[skip ci]`. Registre o SHA exato do commit a publicar. Se mudar, valide-o novamente.

## Compilar o rascunho

Use uma conta autenticada de mantenedor ou GitHub App com as permissões normais de publicação
do repositório. Siga as regras de branch; nunca force o push nem contorne verificações obrigatórias.

```sh
git fetch origin --tags
git switch release
git merge --ff-only origin/release
git merge --ff-only origin/main
git push origin release
```

Para a primeira versão, depois que este fluxo tiver sido integrado em `main`, crie `release` a partir do
commit validado de `origin/main` (`git switch -c release origin/main`) e envie-o. Se a branch
já existir, use os comandos acima. Não crie a branch a partir de trabalho sem relação. Se um
fast-forward falhar, inspecione e resolva a divergência pelo fluxo normal de revisão.

Localize a execução Actions de `release.yml` para o SHA registrado e a branch `release`, e acompanhe essa
execução específica. O fluxo compila pacotes Linux (ambas as arquiteturas, incluindo Flatpak), macOS,
Windows, FreeBSD e web. Quando todos os jobs obrigatórios passam, ele cria um rascunho
`SolidCraft v<version>` com os arquivos, `SHA256SUMS.txt`, notas de instalação e mudanças geradas.

Um segundo push com a mesma versão ainda sem tag pode recompilar e substituir o conjunto completo
de arquivos desse rascunho. Notas de versão existentes são preservadas na recompilação; atualize sua seção de mudanças se
novos commits forem adicionados. Uma tag existente ou versão publicada nunca é sobrescrita. Use uma nova
versão após publicar. Não envie a tag de versão por conta própria: publicar o rascunho a cria
no commit de destino do rascunho.

## Notas para a próxima versão

Itens a incluir nas notas da próxima versão (etapa 3 abaixo) e depois remover daqui:

- Nenhum por enquanto.

## Verificar e publicar

Quando a tarefa autoriza publicar uma versão, agentes devem concluir estas etapas sem
exigir que o proprietário clique na interface do GitHub. Um pedido apenas para preparar uma versão ou
alterar a infraestrutura de publicação não solicita, por si só, a publicação do aplicativo.

1. Confirme que a execução do fluxo passou para o SHA pretendido e que a versão correspondente
   ainda é um rascunho destinado a esse SHA. Evite publicar enquanto outra compilação de versão estiver ativa.
2. Baixe todos os arquivos para um diretório novo. Execute `sha256sum --check SHA256SUMS.txt` e verifique
   a presença de cada plataforma/arquitetura esperada. Faça um teste básico de `--version` da CLI compilada
   e de um modelo de exemplo em uma plataforma disponível. Inspecione a saída do job de verificação de pacotes para
   outras plataformas; informe qualquer plataforma que você não conseguiu testar por conta própria.
3. Revise as notas geradas. Adicione uma descrição concisa das capacidades, correções, mudanças incompatíveis
   e limitações conhecidas. Mantenha as instruções de instalação. Os pacotes atualmente não são assinados;
   não afirme o contrário. Resolva verificações com falha ou arquivos ausentes antes de publicar.
4. Publique o rascunho verificado usando `gh` ou a API equivalente de versões do GitHub. Por exemplo:

   ```sh
   # Substitute the version that was actually built and verified.
   gh release edit v0.2.0 --draft=false --latest
   # For a release candidate instead:
   gh release edit v0.2.0-rc.1 --draft=false --prerelease --latest=false
   ```

5. Confirme que a versão é pública e que sua tag resolve para o SHA registrado. Informe a
   URL da versão, a versão, as verificações feitas e quaisquer limitações restantes.

## Simulações e novas tentativas

`gh workflow run release.yml --ref <branch> -f draft=false` compila pacotes e envia o
artefato Actions `release-assets` sem criar nem publicar uma versão. Para uma nova tentativa explícita
que cria/atualiza um rascunho, use `--ref release -f draft=true`. A criação de rascunho a partir
de outra branch é rejeitada. Uma nova execução ainda deve usar uma versão não publicada e sem tag.

## Coordenação do repositório Flatpak

O repositório Flatpak proposto na PR #39 consome versões **publicadas**, não estes rascunhos.
Mantenha essa implantação separada da compilação do aplicativo. Uma publicação feita com
`GITHUB_TOKEN` do Actions não dispara fluxos subsequentes de `release: published`; nesse
caso, dispare explicitamente o fluxo Flatpak com a tag publicada ou use um token de mantenedor/App
adequadamente autorizado para publicar. Não presuma que o repositório foi atualizado: verifique
a execução da implantação. O fluxo Flatpak solicita o fluxo Pages compartilhado com essa tag;
Pages reúne a documentação, o aplicativo de navegador e o repositório Flatpak. Atualizações
comuns de documentação também incluem a versão Flatpak estável mais recente. Verifique a execução Pages resultante,
não apenas o job de disparo, para que uma versão não apague a documentação ou o aplicativo e atualizações de documentação não
apaguem o repositório de pacotes.

## Pacote Snap e Snap Store

Cada versão e compilação noturna anexa `solidcraft-<version>-linux-<arch>.snap` para x86_64 e
aarch64 (`packaging/linux/snap-pack.sh`, manifesto `packaging/linux/snap/snapcraft.yaml`). Como o
Flatpak, ele reempacota o tarball do job Linux. O job `snap` o instala com `--dangerous`,
inicia o aplicativo confinado em X11 e Wayland sem monitor (`packaging/linux/snap-smoke.py`), modela uma
caixa pelo canal de controle e mantém os quadros capturados como o artefato `snap-smoke-<arch>`.

As etapas de Snap Store só são executadas quando o segredo de repositório `SNAPCRAFT_STORE_CREDENTIALS` existe;
sem ele, são puladas e os arquivos `.snap` ainda são anexados à versão:

- compilações noturnas são enviadas para `latest/edge`;
- rascunhos de versão (pushes para `release`) são enviados para `latest/candidate`, depois que todos os pacotes
  foram compilados e o rascunho foi preparado;
- publicar uma versão GitHub que não seja pré-lançamento executa `snap-store.yml`, que promove
  `latest/candidate` para `latest/stable`. Como no repositório Flatpak, uma publicação feita com
  `GITHUB_TOKEN` não o dispara: nesse caso, execute `gh workflow run snap-store.yml`.

Para habilitar a Snap Store uma vez (proprietário):

1. Entre em <https://snapcraft.io> com uma conta Ubuntu One e registre o nome em
   <https://snapcraft.io/register-snap> (ou `snapcraft login` e depois `snapcraft register solidcraft`).
2. Em uma máquina com snapcraft (`sudo snap install snapcraft --classic`), exporte credenciais
   apenas de envio para este snap:

   ```sh
   snapcraft export-login --snaps=solidcraft \
     --channels=edge,candidate,stable \
     --acls=package_access,package_push,package_update,package_release \
     --expires=2027-10-01 solidcraft-store-credentials.txt
   ```

3. Armazene-as como segredo do repositório e exclua o arquivo:

   ```sh
   gh secret set SNAPCRAFT_STORE_CREDENTIALS -R bherbruck/solidcraft < solidcraft-store-credentials.txt
   rm solidcraft-store-credentials.txt
   ```

4. Opcional: preencha a página da loja em <https://snapcraft.io/solidcraft/listing> (ícone,
   resumo e descrição vêm do snap; adicione capturas de tela e a categoria "Art and design").
5. A próxima compilação noturna preenche `edge` e a próxima versão preenche `candidate`, depois `stable` na
   publicação. Para preencher `stable` imediatamente a partir de uma versão já publicada, baixe seus
   arquivos `.snap` e execute `snapcraft upload --release=stable <file>` para cada um.

Renove as credenciais antes de `--expires` repetindo as etapas 2 e 3. O snap usa confinamento
estrito e apenas conectores que se conectam automaticamente, exceto `removable-media`, que os usuários
conectam com `sudo snap connect solidcraft:removable-media` (getting-started.md).

## WinGet (Gerenciador de Pacotes do Windows)

SolidCraft está em [winget-pkgs](https://github.com/microsoft/winget-pkgs) como
`bherbruck.SolidCraft` (`winget install bherbruck.SolidCraft`). Publicar uma versão GitHub
que não seja prévia executa `winget.yml`, que usa [Komac](https://github.com/russellbanks/Komac)
(binário de versão fixa com checksum verificado) para abrir uma pull request em
microsoft/winget-pkgs pelo fork do proprietário `bherbruck/winget-pkgs`. Envia os três MSIs
versionados (`solidcraft-<version>-windows-{x64,x86,arm64}.msi`, nunca as cópias de nome fixo),
lê seus códigos de produto, vincula as notas e mantém os outros campos do manifesto anterior.
Os bots winget-pkgs validam e um moderador integra, geralmente em um ou dois dias.

O job termina com sucesso sem fazer nada enquanto o segredo `WINGET_TOKEN` estiver ausente
ou antes de a primeira versão do pacote ser integrada. Como em `snap-store.yml`, publicar com
`GITHUB_TOKEN` não o aciona; nesse caso execute
`gh workflow run winget.yml -R bherbruck/solidcraft -f tag=v<version>`.

Para habilitar uma vez (proprietário):

1. Crie um token de acesso pessoal **clássico** em <https://github.com/settings/tokens/new>
   (Settings > Developer settings > Personal access tokens > Tokens (classic)): nota
   "solidcraft winget", validade como um ano e apenas o escopo **`public_repo`**.
   Tokens de granularidade fina não abrem PRs em microsoft/winget-pkgs.
2. Guarde como segredo do repositório (cole o token quando solicitado):

   ```sh
   gh secret set WINGET_TOKEN -R bherbruck/solidcraft
   ```

3. Mantenha o fork `bherbruck/winget-pkgs` (Komac envia suas branches para ele). A branch
   padrão não precisa de sincronização.

Quando expirar, crie um token da mesma forma e repita a etapa 2. Se uma versão foi publicada
com o segredo ausente ou expirado, envie depois com
`gh workflow run winget.yml -R bherbruck/solidcraft -f tag=v<version>`.

<a id="update-signing"></a>
## Assinatura de atualizações

A opção **Atualizar agora** do aplicativo (docs/book/src/guide/updates.md) só instala um pacote cujo SHA-256
corresponde ao `SHA256SUMS.txt` da versão, obtido por HTTPS. Isso protege contra downloads corrompidos e
adulterados em trânsito, mas não contra alguém que possa enviar arquivos da versão. Assinar
a lista de somas de verificação fecha essa lacuna: quando `PUBLIC_KEY` em `crates/ui-egui/src/update/install.rs`
é definido, o aplicativo também exige `SHA256SUMS.txt.minisig`, uma assinatura [minisign](https://jedisct1.github.io/minisign/)
feita com essa chave cujo comentário confiável identifica a versão (`SolidCraft v<version>`), e
recusa atualizar a instalação caso contrário. O workflow de publicação assina a lista quando o
secret `UPDATE_SIGNING_KEY` existe e falha quando o aplicativo exige assinaturas, mas o secret está
ausente.

Para ativá-la uma vez (proprietário):

1. Em uma máquina confiável com minisign (`sudo apt install minisign`, `brew install minisign` ou
   `winget install minisign`), crie um par de chaves sem senha (o workflow não pode digitá-la):

   ```sh
   minisign -G -W -p solidcraft-update.pub -s solidcraft-update.key
   ```

2. Armazene a chave secreta como secret do repositório e mantenha um backup offline dela (perder uma chave
   exige uma nova chave e uma versão do aplicativo que a inclua; vazar uma chave exige o mesmo, rapidamente):

   ```sh
   gh secret set UPDATE_SIGNING_KEY -R bherbruck/solidcraft < solidcraft-update.key
   ```

3. Coloque a chave pública (a segunda linha de `solidcraft-update.pub`, que começa com `RW`) em
   `crates/ui-egui/src/update/install.rs`:

   ```rust
   pub const PUBLIC_KEY: Option<&str> = Some("RW…");
   ```

   e integre a alteração por uma PR. As compilações seguintes recusam atualizações sem assinatura; as versões compiladas
   a partir desse ponto incluem `SHA256SUMS.txt.minisig`.
4. Exclua `solidcraft-update.key` da máquina quando o backup estiver seguro.

Compilações antigas sem a chave continuam atualizando somente com a verificação SHA-256.

## Configuração do repositório

A branch `release` avança apenas para conjuntos deliberados de publicação. Restrinja seus escritores a
mantenedores/agentes de publicação por regras de repositório se a política do proprietário exigir. O
fluxo precisa de `contents: write` apenas no job de criação de rascunho. Ele não envia commits,
calcula versões nem exige um bot semantic-release que contorne proteções de `main`.

Este fluxo substitui a proposta de semantic-release a cada push para main da PR #38; não
integre essa automação junto com este modelo de publicação sem redesenhar seus gatilhos.
