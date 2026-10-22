O SolidCraft é CAD 3D paramétrico de código aberto em Rust: crie um esboço, aplique restrições, extrude e altere um
parâmetro para ver a linha do tempo reconstruída. [Documentação](../../book/src/introduction.md)
· [experimente no navegador](https://bherbruck.github.io/solidcraft/app/) ·
[como contribuir](../../CONTRIBUTING.md) ·
[issues](https://github.com/bherbruck/solidcraft/issues). Agentes de IA podem modelar por MCP:
`solidcraft-cli mcp` ([como](../../book/src/guide/mcp.md)).

**Novidades** do SolidCraft {version} são listadas abaixo, e o
[ROADMAP](../../ROADMAP.md#limitações-conhecidas) apresenta as limitações conhecidas.
Projetos de todas as versões anteriores abrem nesta versão.

**Atualizando de 0.7 ou anterior.** A versão 0.8 mudou o ID do aplicativo Linux para `ai.storyteller.solidcraft`:
`flatpak update` oferece substituir o aplicativo antigo, e `.deb`, `.rpm` e AppImage removem a entrada antiga
do menu.

**Atualizando de 0.5 ou anterior.** A versão 0.6 renomeou SolveCraft para SolidCraft e tornou o contêiner zip
o formato padrão de salvamento. Projetos `.solvecraft`, pastas antigas de preferências e
variáveis `SOLVECRAFT_*` ainda são encontrados, e `solvecraft-cli` ainda encaminha para
`solidcraft-cli`. Quer JSON simples? Use **Arquivo ▸ Salvar como JSON…** ou `--save design.json`. No
macOS, substitua `SolveCraft.app` por `SolidCraft.app`. Detalhes:
[notas de compatibilidade](../../compatibility.md).

A documentação, o aplicativo de navegador e o repositório Flatpak usam as URLs Pages do projeto SolidCraft.

**Qual arquivo?**
- **Windows:** `SolidCraft-{version}-windows-x64.exe` executa diretamente. Ou instale `.msi`, ou extraia o
  `-portable.zip` (x64, x86 e arm64).
- **macOS 11+** (Apple silicon e Intel): abra `.dmg` e arraste `SolidCraft.app` para Aplicativos.
  A CLI está em `solidcraft-cli-{version}-macos-universal.zip`.
- **Linux** (x86_64 e aarch64, glibc 2.35+): `.AppImage` (`chmod +x`, depois execute),
  `.deb` ou `.rpm`, `.flatpak` (`flatpak install --user <file>`), `.snap`
  (`sudo snap install --dangerous <file>`, ou `sudo snap install solidcraft` pela Snap Store) ou `.tar.gz`.
- **FreeBSD:** `tar -xzf solidcraft-{version}-freebsd-x86_64.tar.gz --strip-components 1 -C /usr/local`.
- **Web:** `solidcraft-web-{version}.zip` é um site estático para hospedar por conta própria.
- `SHA256SUMS.txt`: `sha256sum -c SHA256SUMS.txt --ignore-missing`.
- Arquivos sem versão no nome (`solidcraft-windows-x64.msi`, `solidcraft-linux-x86_64.AppImage`, …)
  são os mesmos pacotes com nomes fixos, para links de download que sempre obtêm a versão mais recente.

Para instalar pelo repositório Flatpak e receber atualizações, adicione o Flathub para o ambiente de execução necessário antes de
instalar o SolidCraft; mantenha os dois repositórios remotos na mesma instalação de usuário:

```sh
flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak remote-add --user --if-not-exists solidcraft https://bherbruck.github.io/solidcraft/flatpak/solidcraft.flatpakrepo
flatpak install --user solidcraft ai.storyteller.solidcraft
flatpak run ai.storyteller.solidcraft
```

Instalações Flatpak de 0.7 ou anteriores usam o ID antigo `ai.storyteller.solvecraft`: `flatpak update --user`
oferece substituí-lo por `ai.storyteller.solidcraft`, e as preferências são transferidas.

Pacotes `.flatpak` de arquivo único incluem o endereço do repositório do ambiente de execução; ambientes ausentes
ainda exigem conexão à internet. [Detalhes de instalação Linux](../../book/src/guide/getting-started.md#linux-flatpak).

As compilações ainda não são assinadas. No macOS, clique com o botão direito em `SolidCraft.app` em Aplicativos e escolha
**Abrir** na primeira vez (ou execute `xattr -dr com.apple.quarantine /Applications/SolidCraft.app`).
No Windows, o SmartScreen pode dizer "O Windows protegeu o computador": **Mais informações**, depois **Executar assim mesmo**.
Licenças são integradas (Ajuda ▸ Sobre ▸ Licenças, `solidcraft-cli licences`); os pacotes também
instalam `THIRD-PARTY-LICENSES.txt`.
