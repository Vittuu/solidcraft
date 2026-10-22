# Parâmetros e expressões

Cada cota e entrada de operação é um **parâmetro do modelo** (`d1`, `d2`, …). **Parâmetros do usuário**
são aqueles que você adiciona, com uma unidade, uma expressão e um comentário opcional. Altere-os em
**Alterar parâmetros** (`parameters.change`). A linha do tempo é reconstruída a partir da primeira operação que
os utiliza.

No painel CONFIGURAR da barra de ferramentas SÓLIDO, clique em **Parâmetros** para abrir o editor de
parâmetros do usuário e do modelo. O botão separado **Configurações** abre a tabela de configurações;
**Configurar** permanece na lista suspensa para configurar colunas.

Pressione **Enter** no campo de nome, expressão ou comentário do novo parâmetro para adicioná-lo.
Entradas válidas devolvem o foco ao campo de nome para o próximo parâmetro; entradas inválidas permanecem
editáveis. Nos campos de expressão, **Cima/Baixo** seleciona uma sugestão de preenchimento automático,
**Enter/Tab** a aceita e **Escape** fecha as sugestões. Aceitar uma sugestão
não envia também o parâmetro.

As expressões reconhecem unidades: `2 * width + 5 mm`, `angle / 2`, `sqrt(area)`, `1 in + 3 mm`, usando
`+ - * / ^`, parênteses, `sin cos tan asin acos atan atan2 sqrt abs min max floor ceil round mod
trunc` e conversão de unidades. Um comprimento onde se espera um ângulo (ou o inverso) é um
erro, não uma conversão silenciosa. Ciclos são informados com seu caminho (`a → b → a`), renomear um
parâmetro atualiza todas as expressões, cotas e entradas de operação que o utilizam, e um parâmetro
em uso não pode ser excluído (o erro lista quem o utiliza).

As unidades do projeto (Configurações do documento no navegador: mm, cm, m, in ou ft) definem como um número sem unidade
digitado em uma operação é interpretado. Em um projeto em polegadas, `2` significa 2 in. Os diálogos Caixa, Cilindro, Esfera, Toro
e Hélice começam com suas dimensões nas unidades do projeto, exibindo a unidade (`20 mm`, `20 in`).
Alterar as unidades nunca move a geometria existente. Uma dimensão calculada a partir de um número sem unidade mantém
a unidade atribuída ao número: uma esfera com diâmetro sem unidade de 20 tem o raio
`(20 mm) / 2` em um projeto em milímetros. Ao lado de uma expressão sem unidade, como `2 * 5`, ou de um
parâmetro sem unidade, que é interpretado nas unidades do projeto, o número permanece sem unidade e é interpretado
junto com ela.

Os parâmetros podem ser exportados e importados em CSV ou JSON (`parameters.export`, `parameters.import`).
**Configurações** (`config.*`) mantêm uma tabela de linhas, cada uma com seus próprios valores de parâmetros,
operações suprimidas, materiais físicos e aparências dos corpos, alternadas por `config.activate`.
O seletor Biblioteca da tabela seleciona valores de material ou aparência. Escolha **Seguir material**
para remover uma substituição de aparência. Aparências personalizadas preservam a cor e a opacidade ao
ativar, desfazer e salvar/reabrir. Veja
[comandos e exemplos de configurações](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/configurations.md).
