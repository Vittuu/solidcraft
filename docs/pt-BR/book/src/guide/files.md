# Importação e exportação

| Formato | Leitura | Gravação |
|---|---|---|
| `.solidcraft` (contêiner versionado ou projeto JSON) | sim | sim |
| `.json` (projeto JSON simples e autocontido) | sim | sim |
| STEP (AP203, AP214, AP242) | sólidos, montagens, unidades, nomes, cores | AP242 com nomes, cores e a árvore de componentes |
| IGES 5.3 | sólidos manifold, ou superfícies recortadas costuradas em sólidos | sólidos manifold com nomes e cores |
| 3MF, STL, OBJ | corpos de malha | sim |
| 3MF com identificadores de face | como 3MF | cada triângulo identificado com sua face B-rep, coordenadas exatas |
| URDF | | a montagem como um robô: elos com malhas, massa e inércia, juntas a partir das juntas da montagem |
| DXF, SVG | em um esboço | esboços e planificações como DXF |
| PNG, JPEG | imagens de referência e decalques | |

Abrir um arquivo STEP ou IGES cria um projeto com os corpos do arquivo como uma operação base, e
inserir um adiciona esses corpos ao projeto atual. Operações posteriores são construídas sobre eles. Corpos de malha podem ser renderizados,
medidos, movidos e exportados, mas operações de sólidos exigem corpos B-rep.

**Arquivo → Salvar como malha…** (também na aba Malha, em Modificar, e no menu de contexto de um
corpo) salva os corpos selecionados como STL, 3MF ou 3MF com identificadores de face (abaixo).
Com **Enviar ao fatiador** marcado, grava o arquivo em uma pasta temporária e o abre no fatiador
escolhido, sem pedir um nome de arquivo. A lista mostra os fatiadores encontrados no computador,
com seus ícones; **Personalizado…** adiciona outro programa ao início da lista e o mantém entre
execuções. Para remover um programa adicionado, passe o cursor sobre ele e pressione Delete.

Malhas mecânicas fechadas com seção transversal constante podem ser reconstruídas como sólidos
pelo comando `mesh.convert` (na paleta de comandos, em scripts e pelo canal de controle).

**Arquivo → Salvar como malha…**, com o formato **3MF com identificadores de face** (`file.export`
ou `file.save_mesh` com `format: "3mf-tagged"`), grava um
3MF comum em que cada triângulo também nomeia a face B-rep sobre a qual está e cada coordenada
é o valor de precisão dupla da própria tesselação. Ferramentas de geração de malha e análise podem usar os identificadores para
preservar as faces de um modelo (para cargas, condições de contorno ou arestas de operações); outros leitores 3MF os ignoram.
O formato está especificado em [`docs/formats/3mf-face-tags.md`](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/formats/3mf-face-tags.md).

**Arquivo → Exportar URDF…** (`file.export` com um caminho `.urdf`) grava a montagem para simuladores
de robôs e ROS: um elo por posicionamento de componente com sua malha, massa e inércia, e as
juntas da montagem, para que as peças se movam no simulador como no SolidCraft. As malhas são
gravadas como arquivos STL ao lado do `.urdf`. Veja [`docs/urdf.md`](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/urdf.md).

Os salvamentos são atômicos: uma falha a qualquer momento deixa o arquivo antigo ou o novo, e a
versão substituída é mantida como `<file>.bak`. Um salvamento preserva as permissões do arquivo, e um projeto
aberto por um link simbólico (uma pasta sincronizada vinculada a um projeto) é salvo no arquivo para o qual
o link aponta, mantendo o link. Alterações não salvas são salvas automaticamente em uma pasta de recuperação,
e a próxima inicialização oferece recuperá-las. Arquivos de projeto antigos são atualizados ao abrir, e arquivos
de uma versão mais nova são recusados com uma mensagem.

**Salvar** grava um contêiner por padrão; arquivos com nome `.json` mantêm JSON nos salvamentos seguintes.
**Salvar como** oferece contêiner ou JSON simples no
desktop; **Salvar como JSON** também está disponível no menu Arquivo no desktop e na web. Escolha
a extensão `.json` para JSON simples. **Exportar projeto como JSON** grava uma cópia independente
e autocontida sem alterar o arquivo atual. Scripts podem passar explicitamente
`"format": "json"` para `file.save` ou `file.save_as` mesmo com um nome `.solidcraft`.
A abertura detecta o formato pelo conteúdo, de modo que projetos JSON existentes continuam funcionando.
Contêineres priorizam leitura e salvamento rápidos por padrão. Scripts podem solicitar
`"compression": "compact"` quando o tamanho do arquivo importa mais que o tempo de CPU.

Contêineres eliminam duplicatas de conteúdos STEP, de malha e de imagem idênticos. Coordenadas de malha preservam
a precisão dupla original; isso não é uma conversão para STL. A geometria ainda é reconstruída
pela linha do tempo ao abrir. Quando uma prévia visual salva está disponível, Abrir no desktop
a exibe durante a verificação da geometria; a edição começa depois da reconstrução completa. Cancelar
mantém o projeto atual. Prévias não podem ser editadas ou medidas. Scripts podem optar por um cache descartável de geometria
com `cache: true` em salvamentos com a geometria já carregada; salvar nunca reconstrói geometria não carregada apenas para
criar esse cache. Salvamentos comuns e downloads no navegador usam `cache: false` por padrão. Um cache adicional de malha de exibição continua opcional
com `preview: true`. Caches não suportados ou incompatíveis são ignorados. A abertura no navegador
ainda reconstrói de forma síncrona.
Salvamentos com geometria já carregada e `cache: true` explícito também podem incluir uma pequena miniatura PNG. Ela acompanha o arquivo, para que
a página Inicial possa mostrar o projeto recente sem reconstruí-lo quando não há uma imagem local
disponível. Imagens ausentes ou incompatíveis usam a geração normal de miniaturas.
O padrão `cache: false` também omite a miniatura portátil.
`docs/design-container.md` no repositório descreve os limites e os detalhes de codificação.

Os projetos são salvos no formato mais recente que esta versão do SolidCraft conhece (`"format": "solidcraft/N"`
no arquivo). Ela abre projetos de todos os formatos anteriores e os atualiza durante a abertura; um projeto
feito por uma versão mais nova pede uma atualização em vez de ter seu conteúdo interpretado por suposição. Compilações de desenvolvimento (noturnas)
salvam um número de formato que nenhuma versão publicada lê ainda.

A atualização preserva o significado do projeto. Os projetos mantêm transferências registradas de corpos, espaços de coordenadas explícitos
de curvas 3D e vinculações certificadas de face, junto com estilos semânticos de configuração, deslocamentos associativos
de forma livre e seleções de relevo cilíndrico. Mapas de propriedade, geometrias e
expressões antigos preservam seu significado salvo. Curvas 3D legadas com associações mantidas recebem
o marcador do espaço de coordenadas de seu produtor sem alterar pontos salvos; curvas desvinculadas
preservam coordenadas globais. Uma associação perdida entre componentes, atualizada por uma versão antiga, pode
exigir um novo vínculo ou atualização explícita quando sua fonte voltar. O carregamento nunca tenta adivinhar uma correção
numérica. Registros de associação, matrizes de coordenadas e posicionamentos têm limites e são verificados.

<a id="design-properties"></a>
## Propriedades do projeto

**Arquivo → Propriedades do projeto…** (também em **Configurações do documento** na árvore
do modelo e no menu de contexto do componente superior) registra autoria e identificação:
projeto, número da peça, revisão, autor, organização, descrição e licença. **OK** aplica as
mudanças em uma etapa de desfazer. Scripts e agentes usam os mesmos comandos:
`document.metadata` lê, `document.metadata.set` (`key` e `value`, ou `values: {key: value, …}`)
e `document.metadata.remove` (`key` ou `keys`) alteram. `document.inspect` e a ferramenta
MCP `inspect_design` incluem essas propriedades.

Exportações as carregam onde o formato oferece um lugar:

| Formato | Gravado |
|---|---|
| STEP | `author` e `organization` em `FILE_NAME`, `description` em `FILE_DESCRIPTION` (em uma linha) |
| IGES | `author` e `organization` na seção global |
| 3MF | `project` como `Title`, `author` como `Designer`, `description` como `Description`, `license` como `LicenseTerms` |

Número da peça e revisão não têm lugar padrão nesses formatos e ficam no projeto.

No arquivo, são um objeto opcional `metadata` no nível superior; um projeto sem propriedades
não o contém e é salvo como antes:

```json
"metadata": {
  "project": "ACME enclosures",
  "part_number": "ACME-1042",
  "revision": "B",
  "author": "Jane Doe",
  "organization": "ACME",
  "description": "Vented case for Raspberry Pi 4",
  "license": "CC-BY-4.0",
  "com.acme.description": { "shop_id": 88123 },
  "org.printfarm": { "slicer_profile": "PETG-0.2", "internal_id": 4 }
}
```

- **Chaves do SolidCraft não têm prefixo** e guardam texto. Outras chaves sem prefixo ficam
  reservadas para versões futuras.
- **Outras ferramentas guardam dados sob uma chave DNS reversa própria** (`com.example.tool`:
  dois ou mais rótulos de letras, dígitos, `-` e `_` separados por pontos), contendo um objeto.
  Cada ferramenta grava apenas em sua chave e preserva as demais.
- **SolidCraft preserva cada chave** ao abrir, editar e salvar, no contêiner e JSON simples,
  sem validar dados de outras ferramentas: uma chave conhecida com valor que não é texto
  (`"author": 42`) é mantida, lida como ausente e nunca impede abrir o projeto. Digitar sobre
  ela em Propriedades do projeto a substitui.
- **Chaves de outras ferramentas nunca são exportadas**; Propriedades do projeto as lista somente para leitura.
- **Mantenha valores pequenos**: `document.metadata.set` recusa texto acima de 4 KiB, objeto
  acima de 64 KiB ou com mais de 32 níveis, mais de 256 chaves ou 256 KiB no total. Dados
  binários devem ficar em um arquivo ao lado do projeto.

Propriedades são salvas pela primeira vez no formato 21. Inserir ou vincular outro projeto
não copia suas propriedades; Salvar como as mantém. Um script do projeto
(`timeline.export_script`) as define em uma etapa.

## Salvando no aplicativo web

No aplicativo web, **Salvar** e **Salvar como** mantêm projetos no armazenamento privado desse navegador;
eles não abrem um diálogo para escolher uma pasta no computador. Limpar os dados do site
pode remover esses projetos. Baixe uma cópia para mantê-la fora do navegador ou transferi-la para
outro dispositivo.

1. Abra o menu Arquivo e escolha **Abrir** para mostrar **Arquivos neste navegador**.
2. Escolha **Baixar projeto aberto** para o projeto atual, incluindo alterações não salvas,
   ou **Baixar** ao lado de um arquivo armazenado para obter seu último conteúdo salvo.
3. Localize o arquivo `.solidcraft` nos downloads do navegador. A exibição de um seletor de pasta
   depende das configurações de download do navegador.

Use **Abrir do computador…** no mesmo painel para reabrir um projeto baixado. O aplicativo web
trabalha em uma cópia armazenada no navegador; salvamentos posteriores não atualizam o arquivo baixado original.
Use **Renomear** ao lado de um arquivo armazenado para mudar seu nome. Exportações como STEP e STL
também são baixadas.
