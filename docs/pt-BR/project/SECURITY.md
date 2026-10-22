# Política de segurança

## Versões suportadas

O SolidCraft é recente e muda rapidamente, por isso somente a versão mais nova recebe correções de segurança. Atualize
para a [versão mais recente](https://github.com/bherbruck/solidcraft/releases/latest) antes de
relatar um problema e verifique se ele ainda ocorre em `main`.

| Versão | Suportada |
| --- | --- |
| Versão mais recente | ✅ |
| Compilações noturnas | ✅ (as correções chegam aqui primeiro) |
| Versões anteriores | ❌ |

## Relatando uma vulnerabilidade

**Não abra uma issue pública** para um problema de segurança. Relate-o de forma privada pelo GitHub:
abra a [aba Segurança](https://github.com/bherbruck/solidcraft/security) do repositório e escolha
**Relatar uma vulnerabilidade**
([link direto](https://github.com/bherbruck/solidcraft/security/advisories/new)).

Inclua o que encontrou, como reproduzir (um arquivo de projeto, comando ou solicitação MCP), a
versão e a plataforma, e o impacto esperado. Confirmarei o recebimento assim que puder,
manterei você informado no comunicado e darei crédito quando a correção for publicada, a menos que prefira
permanecer anônimo.

Áreas em que relatos são especialmente úteis:

- abrir arquivos de projeto, STEP, STL ou 3MF não confiáveis (falhas, excesso de memória ou CPU, manipulações de caminhos)
- o servidor MCP e a API de controle (acesso a arquivos fora das pastas permitidas, injeção de comandos)
- o aplicativo web (qualquer acesso além do armazenamento da própria página)
- pacotes de versão e caminhos de atualização (AppImage, Flatpak, MSI, deb, rpm, snap)
