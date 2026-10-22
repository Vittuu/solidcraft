# Conhecendo a janela

![A janela do SolidCraft](../generated/sample-plate.png)

- **Barra de título.** No Windows e no Linux, a barra do aplicativo é a barra de título da janela. À esquerda:
  o menu Arquivo (Novo, Abrir, Salvar, Salvar como, Exportar…), desfazer e refazer, a aba Inicial (a página inicial)
  e uma aba para cada projeto aberto (cada um com seu próprio histórico de desfazer, seleção e câmera; um ponto indica
  alterações não salvas). À direita: Relatar um erro, Ajuda (?), Preferências (que contêm o tema) e os
  botões da janela.
- **Barra de ferramentas.** O seletor de espaço de trabalho (PROJETO) fica à esquerda. As abas (SÓLIDO, SUPERFÍCIE,
  CHAPA METÁLICA, PLÁSTICO, UTILITÁRIOS e ESBOÇO enquanto você cria um esboço) contêm painéis de comandos. Clique no
  nome de um painel para ver todos os seus comandos. Arraste um comando da lista suspensa para o nome
  ou uma área vazia de um painel para acrescentá-lo; a linha de inserção mostra seu destino. Solte sobre um botão
  existente para inserir antes dele. Os botões também podem ser movidos entre painéis, e o layout é salvo.
- **Navegador** (à esquerda). O projeto como uma árvore: configurações do documento e unidades, vistas nomeadas,
  origem, corpos, esboços, planos de construção, imagens de referência, juntas e componentes. O olho mostra ou
  oculta um item, o botão de opção de um componente o torna o componente ativo, e clicar com o botão direito em qualquer
  linha abre o menu desse item. As linhas podem ser arrastadas para grupos e outros componentes.
- **Área de visualização.** Clique para selecionar, use Shift ou Ctrl para adicionar à seleção e arraste para uma seleção por janela ou cruzamento.
  Para alcançar algo sob outro elemento (duas linhas desenhadas uma sobre a outra, pontos coincidentes,
  uma aresta à frente de uma face, corpos sobrepostos), mantenha o botão esquerdo pressionado, sem mover, por meio segundo
  (toque prolongado em uma tela sensível ao toque) ou clique com o botão direito › **Selecionar outro…**: uma lista de tudo sob
  o cursor se abre ali, com o esboço em edição primeiro. Passar o cursor sobre uma linha realça o elemento; clicar nela
  o seleciona (Shift ou Ctrl adiciona à seleção), e em um diálogo de comando ele é enviado à caixa de seleção ativa.
  Esc ou um clique em outro local fecha a lista. Clicar novamente no mesmo ponto, sem nenhuma mudança
  entre os cliques, seleciona o próximo elemento da lista. A
  roda aplica zoom no cursor, o botão do meio desloca a vista e arrastar com o botão direito orbita livremente (o perfil de mouse
  nas Preferências pode mudar para o estilo SolidWorks, Inventor, Alias, Tinkercad, OpenSCAD ou Touchpad). Em um
  trackpad, o gesto de pinça aplica zoom e a rolagem com dois dedos desloca a vista (Navegação › Rolagem com dois dedos desloca a vista;
  desativado: a rolagem aplica zoom como a roda); a roda do mouse aplica zoom e a roda lateral desloca a vista horizontalmente.
  Shift+botão do meio orbita livremente
  nos perfis padrão e Inventor. Em todos os perfis, Ctrl+arrastar com o botão esquerdo (Cmd no macOS), começando
  em um espaço vazio, orbita; Ctrl+clique continua adicionando à seleção. Shift+arrastar com o botão direito ajusta para a vista padrão ou
  diagonal mais próxima; Shift não solicita esse ajuste quando o perfil já o exige para orbitar.
  Com **Touchpad** selecionado em Preferências › Navegação, segure **Shift** e mova o ponteiro para
  deslocar a vista, ou segure **Alt** (**Option** no macOS) e mova-o para orbitar — sem pressionar um botão do mouse.
  A rolagem vertical com dois dedos aplica zoom nesse perfil, independentemente da configuração salva de
  deslocamento com dois dedos; trocar de perfil restaura essa configuração. Solte o modificador para parar de navegar.
  Shift+clique e Shift+arrastar com o botão esquerdo continuam selecionando. Combinações de Shift+Alt ou modificadores Ctrl/Cmd não
  iniciam navegação apenas com o ponteiro.
  A vista dos eixos mostra as direções X/Y/Z globais nas cores do SolidCraft. Clique na ponta
  positiva ou negativa de um eixo para olhar ao longo dele; clique novamente na ponta alinhada para olhar pelo lado oposto.
  Arraste a vista dos eixos para orbitar em torno do centro da vista, usando Shift para ajustar a vistas predefinidas. As vistas superior e inferior
  mantêm a orientação em quartos de volta mais próxima. O ícone de casa, o botão Inicial da barra de navegação e um clique duplo com o botão do meio na vista
  retornam à vista inicial, enquadrando o que está visível naquele momento (Navegação › **Clique duplo do meio** pode definir a ação como Ajustar, que
  mantém a orientação, ou desativá-la).
  Clique com o botão direito para abrir o menu radial: oito comandos ao redor do cursor, além de um
  menu para o elemento sob ele.
  Vistas sombreadas omitem uniões suaves certificadas entre trechos de spline nas paredes
  extrudadas. Vincos reais, contornos das tampas e divisões de face criadas explicitamente permanecem visíveis.
  A vista em arame mantém sua apresentação das arestas, e ocultar uma união não remove sua identidade CAD
  nem altera o sólido exportado. Geometrias de parede não suportadas ou incertas
  mantêm a apresentação de arestas existente.
- **Diálogos** ficam encaixados à direita com uma prévia em tempo real. A maioria das operações também mostra alças de arraste e uma
  caixa de valor na área de visualização.
- **Linha do tempo** (na parte inferior). Todas as operações em ordem. Clique duas vezes para editar, arraste o marcador para retornar a
  um estado anterior e arraste operações para reordená-las.
- **Caixa S.** Pressione S em qualquer lugar para pesquisar comandos no cursor. Fixe os comandos mais usados
  no topo.

As Preferências (Arquivo › Preferências) contêm o tema, as unidades de novos projetos, o perfil do mouse, o
estilo visual e o salvamento automático. Gráficos › Visão de cores atribui às cores semânticas
(estados de esboço, eixos, dobras, análises, ícones) valores para percepção reduzida de vermelho,
verde ou azul: Protan, Deutan ou Tritan. Nesses modos, faces reprovadas por Análise de ângulo
de saída e de acessibilidade também recebem listras.
Ajuda › Atalhos de teclado lista todas as teclas, e Arquivo › Atalhos de
teclado permite alterá-las.

Gráficos › **Qualidade de exibição** oferece **Baixa**, **Normal**, **Alta** e **Ultra**.
Níveis mais finos suavizam faces curvas e arestas, mas levam mais tempo para gerar a malha da
vista. A preferência altera apenas a visualização, sem mudar o projeto ou as exportações.
**Superamostragem (SSAA)** desenha a vista em resolução maior e a reduz para suavizar as bordas:
**Desativada (1×)** é o padrão; 1.5× e 2× usam mais recursos da GPU. Com **Somente quando a vista
estiver parada** marcado, a amostragem extra espera a órbita ou o zoom parar. A resolução é
limitada à dimensão máxima de textura do dispositivo.

As análises de ângulo de saída, acessibilidade e mapa de curvatura mostram uma legenda para
interpretar as cores exibidas na vista.

Navegação › **Zoom suave com a roda** fica ativado por padrão. Desative para aplicar imediatamente cada evento da roda ou do
trackpad, sem continuação de suavização. Ele é independente de **Animar mudanças de vista**,
que controla as vistas de eixo, Olhar para, Inicial e Ajustar. Reativar a suavização durante
um gesto da roda tem efeito depois que o gesto termina, evitando reaplicar zoom pendente.

Navegação › **Alias** usa as teclas de câmera do Alias: Shift+Alt+arrastar com o botão esquerdo orbita, Shift+Alt+arrastar com o botão do meio
desloca a vista e Shift+Alt+arrastar com o botão direito aplica zoom (arraste para baixo para afastar, para cima para aproximar). Sem
Shift+Alt, arrastar com o botão esquerdo continua selecionando por caixa e arrastando geometria do esboço.

Alguns ambientes de desktop Linux capturam Alt+arrastar para mover ou redimensionar janelas, e por isso a área de visualização
nunca recebe esse gesto. Para liberá-lo:

- **KDE Plasma:** em Configurações do sistema › Gerenciamento de janelas › Comportamento das janelas › Ações da janela, defina
  **Tecla modificadora** como Meta (ou Nenhuma).
- **GNOME:** execute `gsettings set org.gnome.desktop.wm.preferences mouse-button-modifier '<Super>'`
  (ou defina Ajustes › Janelas › Tecla de ação da janela como Super).

O ajuste durante a órbita com Shift inclui vistas de aresta a 45 graus e vistas isométricas de canto, com uma
transição de mola levemente amortecida e uma pequena margem nos limites para impedir oscilações.

No Linux, sem um perfil 3DxWare, o SpaceMouse usa os eixos padrão do driver em modo de objeto:
inclinar o controlador para os lados rola a vista, e torcê-lo gira a vista.

No desktop, Ctrl/Cmd+Mais (ou Igual) amplia a interface, Ctrl/Cmd+Menos a reduz,
e Ctrl/Cmd+0 restaura 100%. Esses atalhos atualizam a preferência de escala da interface. O controle deslizante de escala
nas Preferências aplica o novo layout ao ser solto, mantendo a interface imóvel durante o arraste.
