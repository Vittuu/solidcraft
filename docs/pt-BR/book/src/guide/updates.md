# Atualizações e privacidade

O SolidCraft informa quando há uma versão mais recente. Ele nunca se atualiza sem perguntar, e
a verificação não envia nada sobre você ou seus projetos.

## O que aparece

Quando há uma versão estável mais recente, um botão **Atualizar** aparece na barra de título ao lado de
**Relatar um bug**. Ele abre uma pequena janela com a nova versão, suas notas de versão e três
opções:

- **Baixar** abre no navegador o arquivo adequado à sua instalação (veja a tabela abaixo),
  ou a página da versão quando não há um arquivo correspondente.
- **Ignorar esta versão** oculta o botão até que saia uma versão ainda mais recente.
- **Lembrar mais tarde** oculta o botão por três dias.

**Preferências ▸ Geral ▸ Atualizações** tem **Verificar atualizações automaticamente** (ativado por padrão) e
**Verificar agora**, que verifica imediatamente e mostra a janela mesmo para uma versão ignorada.

## Atualização com um clique

Quando o SolidCraft pode substituir a si mesmo com segurança, a janela também oferece **Atualizar agora**:

| Instalado como | O que Atualizar agora faz |
|---|---|
| AppImage | Baixa o novo AppImage ao lado do antigo, verifica-o, renomeia-o sobre o arquivo antigo e oferece **Reiniciar agora** |
| Instalador Windows (`.msi`) | **Instalar (fecha o SolidCraft)**: baixa e verifica o `.msi`, fecha o SolidCraft (perguntando primeiro sobre projetos não salvos) e inicia o instalador |
| Windows portátil `.zip` ou `.exe` | Baixa e verifica o pacote, substitui `solidcraft.exe` (e `solidcraft-cli.exe`) e oferece **Reiniciar agora**; o arquivo antigo é removido na próxima inicialização |
| macOS | **Baixar e abrir**: baixa e verifica o `.dmg` e o abre; arraste o SolidCraft para Aplicativos para substituir a cópia antiga |
| Flatpak, Snap, `.deb`, `.rpm`, `.tar.gz` | Nada: seu gerenciador de pacotes os atualiza (a janela mostra o comando), ou você descompacta o novo arquivo |

Todo download é verificado antes de qualquer alteração: seu SHA-256 deve corresponder ao
`SHA256SUMS.txt` da versão, obtido por HTTPS da mesma versão, e, quando o projeto assinar suas
versões, essa lista deve conter uma assinatura válida da chave de publicação do SolidCraft que identifica a
versão (veja [publicação](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/releasing.md#update-signing)).
Um download que não passa na verificação é excluído e a cópia instalada é mantida; **Baixar**
ainda fornece o pacote para instalação manual. No macOS, o SolidCraft abre a imagem de disco em vez de
substituir o próprio aplicativo, pois ele tem assinatura ad-hoc e não é notarizado: substituí-lo em
segundo plano instalaria código que o macOS não pode garantir sem que você veja.

Atualizar agora exige acesso de gravação ao arquivo instalado: um AppImage em uma pasta em que você não pode gravar
falha com uma mensagem, e **Baixar** é a forma de atualizá-lo.

## O que é enviado

Uma requisição HTTPS, na inicialização e no máximo uma vez por dia enquanto o SolidCraft está em execução:

```text
GET https://api.github.com/repos/bherbruck/solidcraft/releases/latest
User-Agent: SolidCraft-update-check
Accept: application/vnd.github+json
X-GitHub-Api-Version: 2022-11-28
```

**Atualizar agora** então baixa `SHA256SUMS.txt` (e sua assinatura) e o único pacote pelos
links `github.com/bherbruck/solidcraft/releases/download/…` da própria versão, com o mesmo
User-Agent. É tudo: sem número de versão, identificador de máquina ou usuário, dados de uso ou cookies. O GitHub
vê a requisição como qualquer outra visita à API (seu endereço IP, o horário); veja a
[declaração de privacidade do GitHub](https://docs.github.com/site-policy/privacy-policies/github-general-privacy-statement).
O SolidCraft compara a resposta com sua própria versão no seu computador.

- Offline, ou se o GitHub não responder em 15 segundos: nada aparece, e a verificação é repetida
  uma hora depois.
- O limite de requisições anônimas do GitHub (60 por hora por endereço) é respeitado: quando o GitHub
  manda esperar, o SolidCraft espera esse tempo, inclusive ao usar Verificar agora.
- Desativar **Verificar atualizações automaticamente** interrompe as verificações; só **Verificar agora** consulta.
- Definir a variável de ambiente `SOLIDCRAFT_UPDATE_CHECK=0` desativa todas as verificações, por
  exemplo em máquinas gerenciadas.

Compilações feitas por você, o Flatpak e o aplicativo de navegador (que se atualiza sozinho) nunca verificam.

## Qual arquivo Baixar fornece

As versões anexam os pacotes principais com nomes fixos (veja [publicação](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/releasing.md)),
e o SolidCraft escolhe aquele que corresponde à forma como foi instalado:

| Instalado como | Detectado por | Download | Como atualizar |
|---|---|---|---|
| AppImage | `APPIMAGE` definido | `solidcraft-linux-<arch>.AppImage` | Substitua o AppImage antigo pelo novo |
| Flatpak | `FLATPAK_ID` definido | não verifica (veja abaixo) | `flatpak update ai.storyteller.solidcraft` |
| Snap da Snap Store | `SNAP` definido | página da versão | `sudo snap refresh solidcraft` (o snapd também atualiza sozinho) |
| Snap de um arquivo baixado | `SNAP_REVISION` começa com `x` | `solidcraft-linux-<arch>.snap` | `sudo snap install --dangerous ./solidcraft-linux-<arch>.snap` |
| `.deb` | em `/usr`, dpkg conhece `solidcraft` | `solidcraft-linux-<arch>.deb` | `sudo apt install ./solidcraft-linux-<arch>.deb` |
| `.rpm` | em `/usr`, existe um banco de dados rpm | `solidcraft-linux-<arch>.rpm` | `sudo dnf install ./solidcraft-linux-<arch>.rpm` |
| `.tar.gz` | qualquer outro local no Linux | `solidcraft-<version>-linux-<arch>.tar.gz` | Descompacte sobre a cópia antiga |
| Instalador Windows | em Program Files | `solidcraft-windows-<arch>.msi` | Execute o instalador |
| Windows portátil | `solidcraft-cli.exe` ao lado | `solidcraft-windows-<arch>-portable.zip` | Descompacte sobre a cópia antiga |
| Windows `.exe` | qualquer outro local no Windows | `SolidCraft-<version>-windows-x64.exe` | Substitua o `.exe` antigo |
| macOS | dentro de `SolidCraft.app` | `solidcraft-macos-universal.dmg` | Arraste o novo aplicativo para Aplicativos |

A janela mostra o comando para instalações Snap, `.deb` e `.rpm`, com um botão Copiar. O
Flatpak não tem permissão de rede e o próprio Flatpak (ou seu centro de aplicativos) oferece suas
atualizações, por isso nunca verifica. Compilações de publicação comparam versões por [semver](https://semver.org): uma compilação noturna conta
como a versão após a qual foi criada, portanto só recebe a oferta de uma versão estável mais recente, e versões
candidatas recebem a oferta de sua versão final.
