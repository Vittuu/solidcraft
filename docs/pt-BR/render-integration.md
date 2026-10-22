# Integração em fases de renderização e materiais

Acompanhada em #842, com o projeto principal de renderização #620 e o roteiro de materiais
físicos/visuais #922. A integração combinada preserva a ancestralidade das contribuições;
é um caminho alternativo de revisão, não evidência de integração das PRs individuais. Cada
resultado no GitHub está vinculado à revisão de origem testada. Não reutilize um resultado
verde antigo para uma nova integração nem descreva wasm compilado como interação verificada no navegador.

## Fases de entrega

| Fase | Contribuição / entrega | Critério de aceitação |
|---|---|---|
| 1 | R2 #667, R3 #706, R4a #800: configurações do documento, vista de produto, trabalhos limitados de PNG por CPU | Migração/salvar/reabrir/desfazer de configurações, isolamento de documentos/trabalhos, capacidades estritas, cancelamento, evidência nativa e verificações do repositório |
| 2 | Entrada HDR #837 e referência de superfície #1019 integradas | Testes limitados de codec/BRDF; capacidades CPU existentes inalteradas até haver consumidores reais |
| 3 | PBR opaco de produto #1046; persistência de fichas de engenharia #1060 | Campos ópticos suportados explícitos, preservação de fonte/condições, desfazer em uma etapa, reabertura a frio, geometria inalterada e densidade atribuída independente |
| 4 | Catálogos escalares #863 integrados; transporte mais rico #1086; inspetor/importação #1094 | Revisões exatas selecionadas, definições copiadas, limites para entradas hostis, nenhuma conversão com perda, acesso com escopo a arquivos, importação revisada e evidência nativa real |
| 5 | Consumidor de ambiente #813, depois mapeamento de textura/madeira #1004/#786 com admissão de recursos #909 | Interpretação linear/sRGB e iluminação reais, sistemas persistentes locais ao objeto, limites de cancelamento/memória, instâncias transformadas, evidências nativas e de navegador separadas |
| 6 | Qualidade óptica/de dados específica por família sob #922 | Madeira L/R/T e condições de umidade/classe; classe/processo/temperatura/taxa de polímeros; espessura/IOR/absorção de vidro; liga/têmpera/resposta condutora e anisotropia escovada de metais; revisão de fonte/licença antes de distribuir dados |

O formato de documento lançado é 20 (v0.11.0); configurações de renderização e aparências
PBR opacas são salvas no formato 21. Configurações migram de formatos anteriores com recusa
por leitores antigos. A vista Projeto mantém a tesselação upstream por planos de exibição;
Render usa suas malhas preparadas de produto. Trabalhos de execução e identificadores de
dispositivo não são persistidos. Consulte contratos versionados em vez de reservar outro formato.

## Limites da base atual

O host GPU fornece uma **vista** de produto. O executor CPU fornece **imagens PNG** limitadas
de prévia, com PBR opaco opcional e transparência ordenada aproximada. O [consumidor de estúdio
fixo](render-opaque-pbr.md) oferece apenas iluminação direta; não tem HDRI, sombras projetadas,
transmissão ou reflexões indiretas. O utilitário Radiance prepara RGB linear no espaço de cor
da origem; não é iluminação. O contrato numérico de superfície opaca não comprova que um host
renderize essas propriedades. Recursos importados ou intenção salva nunca acrescentam
silenciosamente suporte a HDRI/refração/final/EXR. Solicitações não suportadas falham e preservam
a última imagem válida e o estado CAD. Veja [backends](render-backends.md),
[configurações](render-settings.md), [espaço de trabalho](render-workspace.md),
[trabalhos](render-jobs.md), [entrada HDR](render-hdr-input.md) e [referência de superfície](render-surface.md).

Ambos os modos de publicação atômica de PNG verificam o escopo existente de arquivos do chamador.
A regressão do canal de controle rejeita pastas externas e travessia e permite um destino válido,
inclusive substituição explícita. Essa proteção e a regressão de adoção de documento pertencem
agora ao R4a independente, cuja segurança não depende da integração combinada. O código de
produção do mecanismo/cena corresponde a essa base. A interface combinada também consome o
seletor assíncrono descrito abaixo; não acrescenta outro renderizador ou serviço de arquivos.

## Integração do seletor antes da versão

O serviço upstream integrado #1127 (`file_picker`) substitui o candidato anterior #1114
nesta branch. Há um serviço de seletor, fornecido pela main. Seu future retorna um caminho
selecionado ou cancelamento; o loop de eventos continua desenhando/redimensionando enquanto
entradas de modelo, troca/fechamento de abas e comandos de controle do mecanismo aguardam.
O indicador de compatibilidade `file_dialog_open` espelha `file_picker_busy` e não expõe
caminhos. Callbacks de Render e biblioteca capturam consentimento de substituição e mantêm
identidade explícita de trabalho/editor/documento. Uma imagem removida falha em vez de ser
substituída por um resultado mais recente. Cancelar não grava nada. Alterações de origem/host
que substituam o documento solicitante recusam a exportação; um editor substituído não recebe
uma resposta antiga de biblioteca. Regressões de futures controlados cobrem esses limites.

Evidências nativas anteriores de responsividade e testes de roteamento entre abas usavam
o candidato #1114 e mantêm suas revisões originais. Não comprovam a nova política de retenção
de entradas. Os testes do serviço upstream cobrem entradas retidas e quadros de redimensionamento;
acrescente evidência nativa de revisão antes de descrever o fluxo atualizado como verificado na GUI.

## Propriedade de materiais, aparências e recursos

Propriedades físicas, densidade atribuída, aparências visuais e iluminação de ambiente são
distintas. Editar apenas uma ficha não substitui densidade atribuída ou estilo visual;
aplicar uma aparência não muda a massa. Valores de engenharia ausentes continuam desconhecidos,
com fontes e condições por propriedade. Consistência numérica não é certificação nem solução
estrutural. Eixos mecânicos não são inferidos de uma textura.

O primeiro catálogo escalar continua utilizável sem extensões ópticas/de engenharia.
O transporte mais rico deve preservar registros PBR/fichas copiados e rejeitar conversões com
perda; reabrir um projeto não pode exigir o catálogo de origem. Mantenha revisões de origem,
conflitos e aplicabilidade explícitos. Inspeção/aplicação usa comandos existentes; chamadas
programáticas nunca abrem seletores. Exportar biblioteca edita o arquivo reutilizável em vez
do projeto CAD. Aplicar uma definição é uma edição de projeto que pode ser desfeita em uma etapa.

Use primeiro exemplos sintéticos originais. Uma string de licença em dados importados é
uma alegação da origem, não permissão de redistribuição. Dados ou texturas distribuídos
exigem revisão real de fonte/licença e atribuição. Arte de referência comercial não é copiada.
Não se presume outro renderizador, banco certificado de materiais ou solucionador de simulação.

## Verificação e colaboração

Execute `cargo xtask ci` antes de cada etapa com commit. Registre resultados locais/remotos,
upstream vinculado, plataforma real e falhas separadamente. Não enfraqueça referências
geométricas, mascare avisos ou infira desempenho geral de tempos de captura/cancelamento em
cache. Anexe evidência real do aplicativo/PNG para comportamentos visíveis; identifique evidências
antigas por sua revisão de origem. Use exemplos originais com escopo limitado.

Os documentos privados de `plan/` não estão disponíveis neste checkout; nenhum conteúdo é
inventado. Contratos públicos e instruções atuais regem este trabalho. Proponha interfaces
limitadas no GitHub quando faltar uma passagem de trabalho, convide comentários e verifique
alegações/PRs existentes antes de implementar. Preserve as branches independentes dos colaboradores.
