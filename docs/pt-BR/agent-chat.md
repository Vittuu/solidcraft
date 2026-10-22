# Chat com agente

O chat com agente é um painel lateral em que você conversa com um agente de programação que já usa
(Claude Code, Codex, Gemini CLI, OpenCode, Grok Build e outros) ou com um modelo em um endpoint compatível com a
OpenAI de sua escolha (Ollama ou LM Studio no seu computador, OpenAI ou qualquer outra URL), para que
trabalhe no projeto aberto. Um agente de programação é executado no seu computador usando **sua própria
conta** nesse agente: o SolidCraft não vende modelos nem intermedeia o acesso a eles, e não precisa de
uma chave para isso. A chave de um endpoint, caso ele exija uma, fica no chaveiro do sistema, nunca
nas configurações do SolidCraft.

Disponível apenas no aplicativo desktop.

## Ativar e desativar

Tudo é configurado em uma única página: **Preferências → IA**.

- **Mostrar recursos de IA** (ativado por padrão) é a opção principal. Quando desativada, o
  SolidCraft não mostra nada relacionado: nenhum botão de IA, atalho, item de menu ou painel, e nada
  é procurado ou iniciado. A página continua disponível para você ativar a opção novamente.
- **Mostrar botão na barra de ferramentas** (ativado por padrão): o botão **IA** na extremidade
  direita da linha da barra de ferramentas permanece ali independentemente da largura da janela.
  Desative a opção para manter apenas o atalho e o item de menu.

Com os recursos de IA ativados, você pode abrir o painel por:

- o botão **IA** na extremidade direita da barra de ferramentas;
- **Ctrl+Shift+A** (Cmd+Shift+A no Mac), que também funciona na própria caixa de texto do chat;
- **Configurações da vista** (o último botão da barra de navegação abaixo da vista) → **Chat com agente**.

Nada é executado antes de ser necessário: o SolidCraft procura agentes instalados (uma chamada rápida
com `--version` para cada um, em segundo plano) quando o painel ou a página de IA é aberto pela
primeira vez, e só inicia um agente quando você envia uma mensagem ou pressiona **Conectar**.

## O painel

- **Posição:** arraste o cabeçalho do painel (a alça pontilhada e o título) para a outra metade da
  janela e solte. O lado em que ele será colocado é destacado durante o arrasto. Você também pode
  escolher **Esquerda** ou **Direita** em Preferências → IA → Painel. O SolidCraft memoriza o lado.
- **Largura:** arraste a borda interna do painel. O SolidCraft memoriza a largura.
- **Minimizar** (a linha no cabeçalho) recolhe o painel em uma faixa estreita que mantém o ícone
  de chat, o estado do agente e a quantidade de itens que aguardam sua resposta. Clique no ícone
  para expandir. **Fechar** (×) oculta o painel. Nenhuma dessas ações para o agente; **Parar** faz isso.
- **O seletor de agente** abaixo do título escolhe o agente desta conversa; trocar de agente
  interrompe o que está em execução. Ao lado dele, quando o agente tem modos próprios (o Claude
  Code tem *Default*, *Accept Edits*, *Plan*…), uma segunda lista define o modo do agente.

## Agentes

Preferências → IA → **Agentes** lista todos os agentes que o chat pode usar, cada um com seu tipo
(programa ou endpoint), versão ou modelo, indicação de conexão à conta, **Testar**, **Padrão** (o
agente usado no início de uma nova conversa), sua própria configuração de permissão e **Remover**
para aqueles que você adicionou.

### Detectados automaticamente

O SolidCraft procura agentes que usam o [Agent Client Protocol](https://agentclientprotocol.com)
no seu `PATH` e nas pastas de instalação usuais que o `PATH` de um aplicativo desktop muitas vezes
não inclui (a pasta global do npm, nvm, Volta, bun, `~/.local/bin`, `~/.grok/bin`, Homebrew, `%APPDATA%\npm`).
Para cada um, mostra a **versão** (obtida com `--version`, com limite de três segundos) e se parece
estar **conectado à conta**. As verificações de versão e as sessões dos agentes também usam essas pastas
no `PATH`, para que os adaptadores encontrem runtimes como o Node quando o SolidCraft é aberto pelo
Finder ou pelo Dock. As entradas absolutas existentes no `PATH` mantêm sua ordem; nenhum arquivo
de inicialização do shell é executado.
O SolidCraft verifica apenas se o arquivo de entrada do próprio agente
(`~/.claude/.credentials.json`, `~/.codex/auth.json`, `~/.gemini/oauth_creds.json`, …) ou uma de suas
variáveis de chave existe. Ele nunca lê o arquivo nem inicia o agente para descobrir isso. No macOS,
o Claude Code mantém sua entrada no chaveiro, por isso o SolidCraft não tenta deduzir esse estado.

| Agente | O que instalar |
|---|---|
| Claude Code | `npm install -g @agentclientprotocol/claude-agent-acp` (o adaptador ACP para o Claude Code) |
| Codex | `npm install -g @agentclientprotocol/codex-acp` |
| Gemini CLI | `npm install -g @google/gemini-cli` (usa ACP diretamente: `gemini --acp`) |
| OpenCode | `npm install -g opencode-ai` (`opencode acp`) |
| XAI Grok | [Instalador do Grok Build](https://docs.x.ai/build/overview), depois `grok login` (`grok agent stdio` usa ACP) |
| GitHub Copilot CLI, Qwen Code, goose | listados com seus comandos de instalação quando ausentes |

O painel lista os agentes que não estão instalados com seu comando de instalação e um botão
**Copiar**. **Atualizar** procura novamente depois que você instala um deles. **Testar** inicia um
agente brevemente, solicita uma sessão e o encerra.

### XAI Grok

Instale o **Grok Build** e execute `grok login` em um terminal para entrar na sua conta Grok.
No SolidCraft, abra **IA → Atualizar**, selecione **XAI Grok** e pressione **Conectar**.
Você pode defini-lo como padrão em **Preferências → IA → Agentes**.

O SolidCraft reutiliza a autenticação do Grok Build em `~/.grok/auth.json`; verifica apenas se
esse arquivo existe, sem ler nem copiar seus tokens. Se for necessário entrar na conta, o painel
oferece as formas de entrada do próprio Grok. Você também pode executar `grok login` novamente
e reconectar. O Grok controla o acesso aos modelos e os limites de uso da sua conta. Essa conexão
não exige uma chave de API no SolidCraft. O Grok Build também aceita `XAI_API_KEY` para usuários
que optarem pela autenticação por API.

### Adicionar agente…

- **Agente instalado:** uma entrada própria para um agente instalado, por exemplo, com outros argumentos.
- **Comando personalizado:** qualquer outro programa que use ACP pela entrada e saída padrão, como
  `my-agent --acp`. O SolidCraft o inicia diretamente, sem passar por um shell.
- **API compatível com a OpenAI:** uma predefinição (Ollama `http://localhost:11434/v1`, LM Studio
  `http://localhost:1234/v1`, OpenAI) ou uma URL personalizada, o nome do modelo e uma chave opcional.
  A chave fica no chaveiro do sistema (Keychain no macOS, Gerenciador de Credenciais no Windows e
  Secret Service no Linux) e só é enviada por HTTPS ou para este computador. **Testar** solicita ao
  endpoint os modelos disponíveis. Para esses agentes, o próprio SolidCraft conduz a conversa,
  com as mesmas ferramentas, limites e perguntas de um agente de programação, e no máximo
  24 solicitações ao modelo por mensagem.

## Entrar na conta

Se um agente de programação não estiver conectado à conta, ele informa isso e o painel mostra
**Entrar no agente** com as formas de entrada oferecidas pelo próprio agente:

- **Um comando para um terminal**, por exemplo, `claude-agent-acp --cli auth login --claudeai`, com
  um botão **Copiar**. Execute-o em um terminal, entre na conta ali e pressione
  **Já entrei: conectar novamente**.
- **Um botão** para agentes que fazem a entrada por conta própria (a entrada do Codex com ChatGPT
  abre seu navegador).
- **Uma variável a definir** para agentes que recebem uma chave pelo ambiente. O painel não tem
  um campo para a chave de um agente de programação, por decisão de projeto.

O que você digitou antes de entrar na conta é enviado assim que o agente estiver pronto.

## O que o agente pode fazer

O agente acessa o projeto aberto apenas pelas ferramentas do próprio SolidCraft (o
[servidor MCP](mcp.md)), que o SolidCraft fornece quando o chat começa:

- **Ler o projeto:** listar comandos, inspecionar operações, esboços e parâmetros, medir e
  capturar imagens da vista.
- **Alterar o projeto:** executar qualquer comando de modelagem, isoladamente ou em lote. As
  alterações aparecem na janela enquanto são feitas.
- **Uma etapa de desfazer por mensagem:** tudo o que o agente altera ao responder a uma mensagem
  torna-se uma única etapa de desfazer, com o nome da sua mensagem. A conversa mostra
  *Alterações: N, uma etapa de desfazer* com **Desfazer estas alterações** enquanto nenhuma outra
  alteração tiver sido feita depois.
- **Cartões de ferramentas:** cada chamada de ferramenta aparece como um cartão com seu estado;
  clique nele para ver o resultado.
- **Anexar vista:** marque a opção para enviar uma imagem da vista atual com sua mensagem, para
  agentes que aceitam imagens.

## Permissões

Preferências → IA → **Permissões** define o que o agente pode fazer sem perguntar, para todos os
agentes; cada agente pode ter uma configuração própria em sua linha.

- **Perguntar sempre:** cada alteração no projeto e cada ação de arquivo aguarda **Permitir**,
  **Permitir sempre** ou **Negar** no painel.
- **Permitir edições do modelo automaticamente** (o padrão): comandos de modelagem são executados
  imediatamente; salvar, abrir, exportar, importar, iniciar um novo projeto, qualquer comando que
  receba um nome de arquivo e desfazer alterações anteriores ao início da sua mensagem ainda pedem confirmação.
- **Permitir tudo:** nada pede confirmação. A página avisa que o agente poderá salvar, sobrescrever
  e substituir seu projeto.

**Permitir sempre** memoriza esse comando para esse agente. A página lista o que você permitiu
assim, por agente, com a opção **Redefinir**.

Essas configurações abrangem as ferramentas do SolidCraft. As ferramentas do próprio agente de
programação (shell e edição de arquivos) seguem o modo do agente, escolhido no cabeçalho do painel.
Quando um agente pede confirmação antes de usar uma ferramenta do SolidCraft, o SolidCraft responde
por você, pois sua própria verificação de permissões controla a chamada, e registra isso no chat
como *Permitido automaticamente*. Com **Permitir tudo**, ou após **Permitir sempre**, as outras
perguntas do agente são respondidas e registradas da mesma forma.

O que é considerado "um comando que recebe um nome de arquivo": um parâmetro que contém um arquivo
ou pasta (`file`, `dir`, `output`, …), ou `path` e `source` quando o valor parece um nome de arquivo
(um separador, `~`, um ponto inicial ou uma extensão conhecida), ou qualquer valor desses parâmetros
em um comando que exporta, importa, salva, abre ou insere. O `path` de uma varredura ou padrão
(ids de curvas) e o `path` de uma ocorrência (ids) são geometria e são executados sem uma pergunta.

## O que ele não pode fazer

- **Arquivos aguardam sua decisão**, a menos que você tenha escolhido **Permitir tudo**. **Cancelar**
  ou **Parar** recusa tudo o que está aguardando, e uma pergunta sem resposta é recusada após quatro
  minutos. Os [limites de pasta do servidor MCP](mcp.md) continuam valendo depois da sua permissão:
  a pasta do projeto, a pasta de recuperação e a própria pasta temporária do servidor.
- **Um limite por mensagem.** Por padrão, o agente pode fazer 24 alterações por mensagem; você pode
  definir esse número na página de IA. Ler o projeto não conta. Quando atinge o limite, ele recebe
  a instrução de parar e informar o que falta; envie outra mensagem para deixá-lo continuar.
- **Nenhum relatório de falha**, independentemente das permissões. Relatórios de falha podem conter
  outros projetos e caminhos de arquivos.
- **Sem acesso ao restante do aplicativo.** Ele não pode clicar na interface, alterar configurações,
  sair do aplicativo ou controlar o próprio chat.
- **Suas próprias ferramentas ficam desativadas quando podemos desativá-las.** O SolidCraft declara
  ao agente que não há sistema de arquivos nem terminal, executa-o em uma pasta de trabalho vazia
  própria e solicita ao adaptador do Claude Code que desative suas ferramentas de shell, arquivos
  e web para a sessão.

Quando o agente do painel é parado, outro agente é escolhido ou o SolidCraft é encerrado, o agente
e todos os programas que ele iniciou (a CLI que utiliza e seu servidor MCP) são parados.

## Privacidade

Nada é enviado a lugar algum, exceto ao agente que você executa ou ao endpoint que adicionou.
O SolidCraft inicia um agente de programação como um programa no seu computador e se comunica com
ele pela entrada e saída padrão. O que o agente envia ao próprio serviço, usando sua conta e sob
os termos desse serviço, é entre você e o serviço, exatamente como quando você usa o agente em um
terminal. O SolidCraft nunca tem acesso à senha, ao token ou à chave de um agente de programação,
nem os armazena ou repassa. A chave de um endpoint é mantida no chaveiro do sistema e enviada apenas
a esse endpoint. A própria conexão do SolidCraft com as ferramentas do agente usa um token aleatório
por inicialização, armazenado em um arquivo que só você pode ler. Esse token nunca passa pelo agente.

## Experimentar sem uma conta

Há um agente de teste com comportamento programado na árvore de código-fonte. Ele cria um bloco,
arredonda suas arestas e o mede, sem usar um modelo:

```sh
cargo build --release -p solidcraft-ui-egui --example acp_test_agent
```

Depois, adicione-o em Preferências → IA → **Adicionar agente… → Comando personalizado** com o caminho
`target/release/examples/acp_test_agent`. Escreva "save" na mensagem para ver um cartão Allow / Deny,
ou "ask" para ver o agente pedir sua própria permissão.
