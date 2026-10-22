# Trabalhos de imagem de produto (R4a)

R4a acrescenta imagens CPU limitadas de prévia ao espaço Render. O executor instalado
`builtin` / `cpu-product` suporta Prévia, Imagem e PNG. Não promete refração, HDRI, exposição,
modo final ou EXR; essas solicitações falham explicitamente. O [consumidor PBR opaco](render-opaque-pbr.md)
posterior acrescenta superfícies opcionais com luz direta pelo mesmo host. Aparências escalares
legadas e o snapshot de Projeto preservam seu sombreamento existente.

## Um caminho de comandos

`render.preflight` e `render.start` recebem `setup` (nome salvo) OU `settings` (objeto
RenderSetup; campos omitidos usam os padrões documentados), além dos booleanos opcionais
`transparent_background` e `replace`, ambos false por padrão. Sem setup/settings, usam a
câmera solicitada atual ou ajustam a câmera ao produto. Salvar uma configuração é independente
de iniciar um trabalho. Campos desconhecidos são erros.

Ambos os comandos rejeitam geometria desatualizada sem estabilizar um gesto de modelagem ao
vivo. Conclua o gesto ou use primeiro o limite explícito `render.scene`. A captura usa malhas,
posicionamentos e aparências visíveis definitivos de R3, sem seleção, arestas, eixos, grades,
seções ou análises. Uma captura a frio pode preparar geometria global e malhas de forma síncrona;
não promete latência limitada de avaliação CAD. A verificação valida capacidades e estima a
admissão, mas não reserva fila ou memória do host. Iniciar verifica esses recursos novamente.

Iniciar retorna um `job: {session, serial, generation}` tipado. Mantenha os três campos ao
chamar `render.status`, `render.cancel` ou `render.export`. IDs nunca são aceitos por outra
sessão de documento, cópia temporária ou documento recém-aberto/recuperado. Estado não renderiza
nem espera a conclusão. Informa queued, preparing, running, encoding, cancelling, cancelled,
completed ou failed, com `done/total` por fase, erros estruturados e metadados do quadro
concluído. O progresso reinicia quando muda a fase; não é estimativa de tempo.

A `scene_generation` capturada é a revisão da sessão no início. Edições posteriores, desfazer,
alterações de câmera e troca de abas não modificam a captura. A imagem continua descrevendo
a cena capturada; a interface indica conservadoramente uma revisão diferente (invalidação do
cache de vista também pode avançar a revisão). O estado de execução fica fora do documento,
desfazer, checkpoints e formato de projeto. Fechar/substituir uma sessão cancela seus trabalhos;
o worker não pode publicar um quadro cancelado. Substituir diretamente o documento pelo host
preserva o serviço instalado, mas aloca nova propriedade dos trabalhos; IDs antigos e imagens
concluídas nunca passam para o novo documento. `replace: true` cancela trabalhos ativos e
pendentes retidos antes de enviar a solicitação. Uma geração cancelada não publica tarde.
Uma geração antiga bem-sucedida nunca substitui um resultado bem-sucedido mais novo.

Cada documento retém no máximo dois trabalhos ativos, quatro registros terminais recentes e
uma última imagem bem-sucedida. Cancelamento/falha preserva essa imagem. Uma nova imagem
bem-sucedida libera a anterior; metadados terminais antigos podem informar conclusão, mas não
garantem bytes exportáveis. Exportar exige um ID explícito de trabalho concluído retido;
nunca exporta silenciosamente outro trabalho mais novo.

## Execução nativa e no navegador

Cada aplicativo instala um host CPU compartilhado pelos documentos. A execução nativa usa um
worker e no máximo duas entradas na fila entre documentos, sem depender de consultas da UI.
O navegador usa a mesma tarefa incremental, acionada por quadros da UI, sem threads.
Preparação, ordenação estável de transparência, limpeza, testes de pixels e redução verificam
cancelamento entre pequenas unidades. Uma etapa admite no máximo 16384 unidades; codificar PNG
também cede a execução após quatro linhas. A UI processa 1024 unidades por quadro. São limites
de trabalho, não garantias de tempo real: alocação, compressão e agendamento do SO ainda têm
latência dependente da plataforma.

A admissão limita a saída a 4096 por dimensão e 4 milhões de pixels; memória temporária é
estimada em 48 bytes por pixel de saída (superamostragem 2x). Outros limites: 262144 triângulos/
substituições de face/polilinhas de aresta, 128 MiB de geometria retida, 384 MiB de memória
estimada por trabalho, 512 MiB de admissão compartilhada incluindo PNGs retidos e 128 milhões
de testes de pixels projetados. O limite de trabalho projetado é verificado na preparação
limitada e pode fazer falhar um trabalho admitido. Bytes PNG têm limite adicional
`8 * output_pixels + 65536`. Alocador e codec são estimativas, não uma cota de memória do SO.
Limites falham com erros em vez de alocar buffers sem limite. Quadros concluídos mantêm a
reserva de memória enquanto os bytes são retidos.

O caminho CPU sombreia em luz linear, mistura aproximadamente alfa dos triângulos ordenados
e codifica sRGB de oito bits com alfa direto e chunk sRGB PNG. Arestas transparentes
compartilhadas usam uma regra consistente de preenchimento. Superfícies transparentes que se
cruzam e ordenação de triângulos continuam aproximações raster. Não é um renderizador de
vidro/refração. Prévia GPU nativa e exportação CPU podem diferir em sombreamento e transparência.

PNG usa a crate Rust `png` já transitiva (MIT OR Apache-2.0), agora dependência explícita de
renderização. Não há mecanismo externo ou arte de terceiros nova.

## Exportação e automação

`render.export` recebe `job`, `path` terminado em `.png` e `overwrite` (padrão false).
O destino é verificado contra o escopo existente de arquivos antes de ler o quadro retido
ou chamar qualquer escritor nativo. Pastas não permitidas e travessia são recusadas nos dois
modos. Exportar sem substituir grava/sincroniza um temporário ao lado do destino e publica
com hard link atômico; um destino existente vence mesmo em uma corrida. Sistemas sem essa
operação retornam erro. Substituir explicitamente usa o escritor atômico existente sem ler ou
copiar o destino anterior. Falhas de gravação preservam a imagem concluída e o estado CAD.
No navegador, exportar usa o serviço virtual de arquivos/download; caminhos de sistema ou
conteúdos base64 não são persistidos no projeto.

O painel Render oferece Verificar solicitação, Iniciar renderização, progresso por fase,
Cancelar e Exportar PNG. Substituir arquivo existente é uma escolha explícita. Uma nova
renderização que falha ou é cancelada mantém disponível a exportação anterior bem-sucedida.

A exportação nativa PNG na integração combinada usa o seletor assíncrono #1127 vinculado ao
documento. ID de trabalho e escolha de substituição são capturados antes de abrir: cancelar
não grava nada, substituir o documento de origem descarta a resposta e uma imagem removida
falha em vez de substituir por um resultado novo. Entradas de modelo e troca/fechamento de
abas aguardam a resposta; desenho e redimensionamento continuam.

Controles e mensagens de fase usam IDs estáveis de catálogos offline; os idiomas registrados
são inglês, japonês, chinês, alemão, francês, coreano, russo, ucraniano e português brasileiro, com fallback
nas lacunas de catálogos parciais. IDs de comandos e identificadores de cenários não mudam.

```sh
solidcraft-cli render examples/render-studio.json --out product.png --width 800 --height 600
solidcraft-cli render design.solidcraft --setup Studio --out product.png --transparent
```

A CLI segue verificar/iniciar/esperar nativamente/exportar, com espera padrão de 120 segundos
(`--timeout 1..3600`). Esgotar o prazo solicita cancelamento. `--overwrite` é obrigatório para
substituir um destino existente. Uma sessão MCP persistente sem GUI instala o mesmo host:
use `execute` com os cinco comandos e o ID retornado. Fechar o processo CLI/MCP encerra a
sessão; um `exec render.start` isolado não é serviço durável em segundo plano. No navegador,
mantenha o aplicativo executando para os quadros processarem o trabalho.

Evidências de validação e medições de responsividade estão na issue/PR R4a; testes nativos
passando ou wasm compilado não estabelecem desempenho visual no navegador.
