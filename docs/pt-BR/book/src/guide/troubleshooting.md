# Solução de problemas: SolidCraft não inicia

## 0.10.0 fecha alguns segundos após iniciar (Windows)

SolidCraft 0.10.0 falha na inicialização em PCs Windows que usam Direct3D 12 e o compilador
de shaders do sistema. Isso inclui gráficos Intel antigos, PCs sem driver gráfico (o
"Microsoft Basic Display Adapter"), máquinas virtuais e sessões de Área de Trabalho Remota.
**Instale 0.10.1 ou posterior**, que corrige o problema. Versões anteriores à 0.10.0 não o apresentam.

## Iniciar em modo seguro

O modo seguro inicia com configurações padrão (suas configurações são preservadas para a próxima
inicialização normal), ignora a oferta "Recuperar projeto não salvo?" e usa o renderizador por software.

- Windows: execute `solidcraft.exe --safe-mode` em um terminal na pasta de instalação, ou adicione
  ` --safe-mode` ao final do *Destino* de um atalho.
- macOS: `open -a SolidCraft --args --safe-mode`
- Linux: `solidcraft --safe-mode` (Flatpak: `flatpak run ai.storyteller.solidcraft --safe-mode`)

Definir `SOLIDCRAFT_SAFE_MODE=1` tem o mesmo efeito. A partir da 0.10.2, se duas inicializações
seguidas pararem antes de abrir a janela, a próxima inicia automaticamente em modo seguro.
`SOLIDCRAFT_RENDERER=cpu` usa apenas o renderizador por software e mantém suas configurações.

## Redefinir as configurações

SolidCraft mantém suas configurações e seus dados nas pastas abaixo. Todas as versões instaladas
as compartilham. Nenhuma versão altera algo fora delas: nem configurações do sistema, drivers ou
entradas do registro. Feche o SolidCraft e **renomeie** (não exclua) as pastas. O aplicativo inicia
com os padrões e cria pastas novas. Restaure os nomes para recuperar suas configurações, ou envie
as pastas com um relatório de defeito.

| Sistema | Configurações (`app.ron`, `graphics.json`) | Dados (`recovery`, `crashes`, `control`, miniaturas) |
|---|---|---|
| Windows | `%APPDATA%\SolidCraft` | `%LOCALAPPDATA%\SolidCraft` |
| macOS | `~/Library/Application Support/SolidCraft` | (a mesma pasta) |
| Linux | `~/.local/share/solidcraft` | (a mesma pasta); miniaturas em `$XDG_CACHE_HOME/SolidCraft` ou `/tmp/SolidCraft` |
| Linux (Flatpak) | `~/.var/app/ai.storyteller.solidcraft/data/solidcraft` | (a mesma pasta) |

No Windows, cole `%APPDATA%` ou `%LOCALAPPDATA%` na barra de endereço do Explorador para abrir
a pasta. Versões anteriores à mudança de nome usavam pastas `SolveCraft` ao lado destas;
SolidCraft apenas as lê, portanto você pode mantê-las.

- A pasta **`recovery`** guarda salvamentos automáticos de projetos que nunca foram salvos.
  Renomeie a pasta de dados em vez de excluí-la, para ainda abrir esses arquivos `.solidcraft`
  por Arquivo > Abrir.
- A pasta **`crashes`** guarda relatórios de falha. Anexe o mais recente ao relatório de defeito.

A partir da 0.10.2, SolidCraft também verifica os arquivos de configuração ao iniciar. Um arquivo
que não consiga ler é renomeado para `app.ron.bad-<time>` ou `graphics.json.bad-<time>`, e o
aplicativo inicia com os padrões.

## "No graphics adapter" ou nada acontece

SolidCraft precisa de Direct3D 12 ou Vulkan no Windows, Metal no macOS e Vulkan ou OpenGL no
Linux. Sem driver gráfico, usa o renderizador por software do sistema (WARP no Windows,
llvmpipe ou lavapipe no Linux), que é lento, mas funciona. A partir da 0.10.2, se nenhuma janela
puder ser aberta, tenta o renderizador por software uma vez e, se necessário, mostra uma
mensagem explicando a falha (versões anteriores fechavam sem mensagem). Experimente:

- Instalar o driver do fabricante da GPU (Intel, AMD, NVIDIA) em vez do Windows Update.
- Em uma máquina virtual, ativar a aceleração 3D. Em Área de Trabalho Remota, testar na própria máquina.
- Executar `solidcraft --crash-log` para listar relatórios recentes e anexar o mais novo a uma issue.

## O desktop engasga ou congela enquanto SolidCraft constrói corpos

Alguns drivers travam no shader que SolidCraft usa para construir corpos na GPU, e o sistema
reinicia a GPU. SolidCraft já constrói corpos na CPU na AMD FirePro D300/D500/D700 (Mac Pro 2013).
Em outra GPU com esse problema, inicie com `SOLIDCRAFT_CPU_TESS=1` para construir corpos na CPU
e abra uma issue com o nome da GPU.

## SolidCraft fecha imediatamente no Windows

Um driver gráfico pode derrubar o aplicativo durante a inicialização, antes que ele mostre
algo (um driver Intel Vulkan antigo, `igvk64.dll`, causava isso). O Visualizador de Eventos
do Windows identifica o driver em *Logs do Windows › Aplicativo* ("Nome do módulo com falha").

A partir da 0.11.1, SolidCraft no Windows inicia Direct3D 12 primeiro e não carrega outra API
gráfica quando Direct3D 12 encontra uma placa; Vulkan e depois OpenGL são tentados apenas em
um computador sem uma. Versões anteriores iniciavam Vulkan, Direct3D 12 e OpenGL juntos e
preferiam Vulkan. Para escolher a API em qualquer versão, abra um Prompt de Comando e inicie
SolidCraft nele:

```
set WGPU_BACKEND=dx12
"C:\Program Files\SolidCraft\solidcraft.exe"
```

`vulkan` e `gl` são as outras opções. `setx WGPU_BACKEND dx12` mantém a escolha para aplicativos
abertos pelo menu Iniciar após entrar novamente na sessão. A partir da 0.11.1, a configuração
também pode ficar em `graphics.json`, na pasta de configurações, como
`{"msaa": 4, "backend": "dx12"}` (`auto` usa a ordem acima). O modo seguro a ignora e sempre
usa o renderizador por software.

## Um projeto salvo pela 0.10 não abre em uma versão anterior

Projetos salvos por um SolidCraft mais novo usam um formato de arquivo mais novo. Versões
anteriores os recusam com "este projeto foi salvo por um SolidCraft mais novo" e não alteram
o arquivo. Abra-os com a versão atual.
