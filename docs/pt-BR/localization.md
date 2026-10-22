# Localização da interface

SolidCraft incorpora suas traduções e funciona offline. Inglês, japonês, alemão, francês, coreano, russo, ucraniano,
turco, chinês e português brasileiro (`pt-BR`) estão registrados atualmente em `crates/engine/src/i18n.rs`. Adicionar um idioma exige um
catálogo incorporado e um registro nesse arquivo; o seletor de preferências e a validação usam o mesmo
registro.

A interface reexporta os catálogos imutáveis compartilhados e fornece seus escopos de renderização. Os textos
de apresentação da CLI e do MCP usam o mesmo registro de idiomas sem depender da interface gráfica.
`solidcraft-cli --language pt-BR --help` seleciona português brasileiro; `SOLIDCRAFT_LANGUAGE`
é a configuração equivalente por variável de ambiente. IDs de comandos, nomes de propriedades e valores de enums
legíveis por máquinas mantêm sua grafia canônica. Descrições de protocolo com exemplos JSON literais
usam o dicionário de texto-fonte simples em `assets/locales/protocol/pt-BR.json`, independentemente dos
modelos da interface com placeholders nomeados.

O aplicativo desktop também aceita `solidcraft --language pt-BR` (ou `SOLIDCRAFT_LANGUAGE`),
inclusive na ajuda do terminal e no resumo de relatórios de falha. O idioma indicado na
inicialização prevalece sobre as preferências salvas. O navegador aceita `?language=pt-BR`;
a mensagem de carregamento, o idioma da página, os metadados e a preferência do aplicativo
seguem essa escolha explícita válida. O preloader HTML usa o pequeno cache
`solidcraft-language` apenas para o texto inicial enquanto o aplicativo Rust carrega.
Após a inicialização do aplicativo, suas preferências RON salvas são a fonte de verdade;
uma query de idioma válida prevalece sobre a preferência carregada. O cache é então
atualizado a partir do idioma selecionado no aplicativo.

A ajuda dos parâmetros é identificada pelo ID canônico de comando em `assets/locales/commands/pt-BR.json`.
Tanto `solidcraft-cli commands` quanto os catálogos de comandos MCP traduzem apenas campos de apresentação.
Pesquisas MCP aceitam títulos localizados e os rótulos/IDs originais. A referência de comandos
gerada do livro em português usa o mesmo dicionário e falha se faltar ajuda de parâmetros.

O manual em português também traduz a navegação, a ajuda de teclado, o seletor de temas,
as mensagens de busca e área de transferência, os redirecionamentos e a página 404 do mdBook
por `assets/locales/book/pt-BR.json`. O gerador Rust preserva o conteúdo e o código dos capítulos,
atualiza os hashes de conteúdo dos scripts traduzidos e rejeita entradas de catálogo ausentes
dos controles gerados pelo mdBook.

## Mensagens

`assets/locales/en.json` define IDs estáveis de mensagens e o texto de fallback em inglês.
Mantenha os IDs ao mudar a redação. Outros catálogos usam os mesmos IDs; mensagens ausentes recorrem
ao inglês. Use `app.tr("ui.preferences")` para mensagens estáticas e
`Locale::format` para frases completas com placeholders nomeados. Traduza a frase inteira,
incluindo a pontuação, para que o tradutor possa reordenar seus argumentos. Os valores de placeholders são
inseridos literalmente, mesmo que um nome de documento contenha chaves.

O adaptador de rótulos em inglês (`app.tr_label`) suporta tabelas existentes do mecanismo e da interface. Ele mapeia
o texto exibido para um ID de mensagem e então usa a mesma busca no catálogo. Novas mensagens da interface devem usar
IDs estáveis diretamente. O adaptador não altera IDs de comandos, nomes de modelos ou identificadores de cenários.
Quando a redação em inglês de uma tabela mudar, atualize também a entrada correspondente do catálogo inglês.
Rótulos gerados da árvore do modelo usam modelos nomeados, enquanto os nomes de modelos subjacentes permanecem intactos.

Os catálogos são analisados uma vez e consultados por mapas ordenados. Isso é localização offline,
não uma afirmação de custo zero em tempo de execução. Os testes de catálogo verificam IDs de idiomas registrados,
chaves de tradução e consistência de placeholders. Execute `cargo xtask ci` antes de enviar alterações.

## Cobertura

Uma mensagem que um idioma nunca recebeu recorre silenciosamente ao inglês, portanto uma lacuna aparece apenas como uma linha
que parece não traduzida. Duas proteções detectam isso na compilação:

- `every_complete_catalog_carries_every_english_message` — adicionar uma chave a `en.json`
  sem fornecê-la a um catálogo completo faz os testes falharem. Inglês é o catálogo de origem;
  japonês, chinês e português brasileiro são entregues completos; alemão, coreano, russo, francês e ucraniano são parciais — suas
  lacunas recorrem ao inglês e são informadas por `cargo xtask i18n` sem reprovar a compilação. Um
  novo rótulo exige en, ja, zh, pt-BR, de e ko; russo, francês e ucraniano podem vir depois.
- `every_toolbar_tab_and_panel_is_in_the_english_catalog` — abas e painéis chegam à interface como strings
  simples de origem, resolvidas apenas pelo adaptador de rótulos em inglês, portanto um título ausente em `en.json`
  nunca pode ser traduzido.

Rótulos de comandos são o mesmo caso em uma escala maior: cada rótulo no registro do mecanismo agora tem
uma entrada em `cmd.<command id>`, e `every_command_label_is_in_the_english_catalog` falha se um
comando for adicionado sem uma.

O menu de contexto e a árvore do modelo são novamente o mesmo caso e contêm a maioria dos rótulos:
`every_menu_label_is_in_the_english_catalog` abre todos os menus acessíveis ao aplicativo, alterna o estado
de cada condição de rótulo (origem mostrada, corpos bloqueados, corpos ocultos) para que ambos os ramos
sejam vistos e falha em qualquer rótulo nunca visto pelo catálogo. Símbolos de unidades ficam fora da
varredura: `mm` é `mm` em todos os idiomas, portanto não são texto a traduzir. A varredura também verifica
que continua alcançando um conjunto amplo de rótulos, para não passar por não encontrar nada.

`cargo xtask i18n` exibe o mesmo panorama em todos os idiomas — quais mensagens estão
ausentes em cada um e quais rótulos de comandos o inglês não consegue traduzir. `--check` transforma isso em um código de saída diferente de zero
para a CI. `--add-missing` grava cada rótulo descoberto em `en.json` sob `cmd.<command id>`, com
o rótulo em inglês como texto, para que o adaptador de rótulos o encontre e faltem apenas as traduções.
Uma chave por rótulo distinto: dois comandos que compartilham um rótulo (`Trim`, `Extend`) não devem ambos
reivindicá-lo, ou a busca pelo texto inglês escolheria um silenciosamente.

## Renderização e cobertura

Um único tipo imutável `Locale` cuida de todas as traduções. Widgets de tabela sem referência ao aplicativo
usam um escopo de renderização que restaura o idioma anterior ao sair, incluindo renderizações aninhadas.
Painéis traduzidos independentes entram no mesmo escopo. Meça o texto renderizado para dimensionar widgets
personalizados; o comprimento em bytes UTF-8 não descreve sua largura exibida.

O host de desktop tenta fontes CJK do sistema em Windows, macOS e Linux, além de um pacote opcional
de fontes `CRAFT_FONTS_DIR`, e ignora arquivos que não podem ser lidos. Chinês usa sua própria cadeia
de fontes alternativas após a fonte CJK geral: muitas fontes CJK japonesas (Hiragino Gothic, Meiryo, Yu Gothic)
correspondem à busca por fontes CJK, mas não têm caracteres simplificados, portanto uma segunda alternativa encontrada pela
lista de nomes de fontes chinesas (`PingFang`, `Microsoft YaHei`, `Noto Sans CJK SC`, …) os cobre por
glifo. Nenhuma fonte é redistribuída nos pacotes nativos de distribuição para desktop; compilações nativas dependem
inteiramente das fontes existentes do sistema operacional ou de craft-fonts.

No navegador, `solidcraft-web` inclui uma fonte japonesa hospedada pelo próprio projeto (`BIZUDPGothic-Regular.ttf`, licenciada
sob SIL OFL 1.1) apenas nos artefatos de distribuição web. A fonte é carregada de forma assíncrona da mesma
origem (`./fonts/BIZUDPGothic-Regular.ttf`), sob demanda, quando o idioma da interface é CJK ou, com
qualquer outro idioma de interface, na primeira vez em que texto japonês (kana, kanji ou formas de largura completa em um nome de projeto,
nome de arquivo, nome de corpo ou texto digitado) é desenhado, sem contatar CDNs externas ou servidores de terceiros.
Uma sessão que nunca exibe esse texto não baixa fontes. Se o carregamento de fonte falhar com uma interface CJK, a interface
web mostra inglês apenas naquela sessão para evitar glifos ausentes (tofu); a preferência salva de idioma
é mantida, para que a próxima visita tente novamente. A fonte incluída é voltada especificamente ao japonês; também
renderiza os kanji compartilhados pelo chinês e japonês, enquanto coreano (Hangul) e caracteres chineses exclusivamente
simplificados não são cobertos, portanto um pacote de fontes SC/coreano hospedado pelo projeto continua sendo trabalho futuro para cobertura web completa.

As mensagens atuais suportam substituição nomeada, não seleção de plurais específica por idioma,
formatação de números/datas ou layout e composição da direita para a esquerda. Adicione essas capacidades ao
introduzir mensagens ou idiomas que precisem delas; use um formatador de mensagens estabelecido para
regras de plural, em vez de adicionar ramos de idioma a painéis. Nomes de modelos definidos pelo usuário,
IDs de comandos, expressões, unidades e dados de modelos legíveis por máquina mantêm a grafia canônica.
O português brasileiro traduz a prosa de diagnóstico escrita pelo projeto ao apresentar erros
na interface, na CLI e no MCP. Nomes de objetos, IDs, expressões, caminhos de arquivos e dados
fornecidos não são traduzidos. Detalhes opacos
do sistema operacional, de terceiros e de depuração também permanecem literais; a explicação
escrita pelo projeto ao redor deles é traduzida.

Os modelos de diagnóstico próprios do projeto ficam em `assets/locales/diagnostics/pt-BR.json`.
`assets/locales/diagnostics/arguments.json` identifica apenas argumentos cuja origem em
prosa de diagnóstico própria do projeto foi comprovada. Esses argumentos podem ser
traduzidos recursivamente dentro dos limites de bytes, campos, passos e profundidade do
matcher. Os demais argumentos permanecem literais; mensagens desconhecidas ou ambíguas
também mantêm o texto original.

`cargo xtask --language pt-BR <command>` seleciona relatórios humanos em português, e
`SOLIDCRAFT_LANGUAGE` também se aplica. Argumentos após o comando continuam a chegar
aos subprocessos sem reescrita de opções de idioma.
