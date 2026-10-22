# Fichas de dados físicos de materiais

`material.properties.validate {card}` verifica um registro candidato fornecido pelo chamador,
sem atribuí-lo, salvá-lo no projeto, ler arquivos, buscar fontes ou alterar o histórico,
a seleção, a câmera ou a geometria ativa. Esta é a fase 1 de #923/#922, rumo a #785.
O comportamento existente de `material.create/edit` e de densidade/aparência permanece
inalterado. O formato de projeto não muda.

Validação significa consistência numérica e estrutural. Ela não autentica fontes, aprova
redistribuição, certifica valores para projeto, fornece um solver nem habilita renderização
avançada. O resultado informa `analysis_solver_available: false` e
`rendering_properties_available: false`. Propriedades ausentes permanecem desconhecidas.

## Contrato (esquema 1 de ficha independente)

Uma ficha tem `schema_version: 1`, `designation`, `family` (`wood`, `polymer`, `glass`,
`metal`, `other`), `sources` e `properties`. IDs de fontes são referências locais exatas
que diferenciam maiúsculas de minúsculas. Cada fonte tem `id`, `title`, `kind`
(`user_supplied`, `measurement`, `public_reference`, `illustrative`) e textos opcionais
`license` e `reference`. Esses textos são metadados fornecidos pelo chamador; mesmo uma
licença aberta declarada permanece em `source_review_required`. Nada é baixado.

Cada propriedade tem `data`, `source_id` e `conditions` opcionais. As condições são
`temperature_k`, `moisture_dry_basis_kg_kg`, `strain_rate_per_s` e um `state` em texto livre
(classe, têmpera, processamento ou condição do corpo de prova). As condições pertencem à
propriedade, sem presumir uma temperatura ambiente global. Um bloco elástico representa
constantes medidas sob as mesmas condições e fonte; curvas com condições diferentes e
constantes de fontes independentes precisam de uma representação posterior.

| `data.kind` | Campos SI explícitos |
|---|---|
| `density` | `kg_m3` |
| `elasticity` | `model`, descrito abaixo |
| `tensile_strength`, `compressive_strength`, `yield_strength`, `shear_strength` | `pa` |
| `thermal_conductivity` | `w_m_k: [k1,k2,k3]`, `axes` |
| `thermal_expansion` | `per_k: [a1,a2,a3]`, `axes` |
| `specific_heat` | `j_kg_k` |

Valores direcionais usam `axes: material123` (eixos locais 1/2/3 do material) ou `wood_lrt`
(longitudinal/radial/tangencial). Esses rótulos não atribuem um sistema de eixos a um corpo
nem inferem a orientação mecânica a partir da rotação da textura. Valores direcionais
iguais podem descrever uma propriedade térmica isotrópica. A expansão térmica pode ser
zero ou negativa.

Um modelo elástico isotrópico tem `symmetry: isotropic`, `young_modulus_pa` e
`poisson_ratio`. A estabilidade exige E > 0 e -1 < nu < 0.5. Valores auxéticos válidos
são aceitos. O resultado deriva G = E/[2(1+nu)] e K = E/[3(1-2nu)], em Pa.

Um modelo ortotrópico tem `symmetry: orthotropic`, `axes`,
`young_moduli_pa: [E1,E2,E3]`, `shear_moduli_pa: [G12,G23,G31]` e
`poisson_ratios: [nu12,nu23,nu31]`. Aqui, nu_ij = -eps_j/eps_i sob tensão ao longo de i;
a reciprocidade fornece nu_ij/Ei = nu_ji/Ej. A matriz de flexibilidade normal é simétrica,
com diagonal 1/Ei e termos fora da diagonal -nu_ij/Ei. O escalonamento diagonal produz uma
diagonal unitária; o critério de Sylvester verifica se a matriz é positiva definida.
Os três módulos de cisalhamento devem ser positivos. Coeficientes de Poisson ortotrópicos
gerais não são limitados a 0.5. Módulos cujos inversos não possam ser representados como
valores finitos são rejeitados; a resposta isotrópica derivada e seus inversos também
devem permanecer finitos/positivos.

A umidade é kg de água / kg de material seco e pode exceder um. Ela não é uma fração da
massa úmida. Temperatura/estado, umidade da madeira ou taxa de deformação de polímeros não
especificados são informados em `unspecified_conditions`; não são presumidos nem substituídos
por zero. O rótulo de família não certifica que um modelo constitutivo escolhido seja
adequado a um material. Resistências escalares são medições individuais, não uma envoltória
tridimensional de falha. Direções de resistência tipadas e métodos de ensaio permanecem
uma extensão separada.

## Limites de aceitação

Antes de clonar/desserializar JSON: no máximo 2.048 nós, profundidade 12, 32 entradas de
vetor ou campos de objeto e um limite conservador de contabilização de 256 KiB
(32 bytes por nó mais os bytes de textos/chaves). O registro tipado permite 32 fontes e
16 propriedades; IDs de fontes ou tipos de propriedades duplicados e campos desconhecidos
são rejeitados.

Limites de texto em bytes UTF-8: designação/título 256, ID de fonte 128, licença 512,
referência 2.048 e estado 1.024; caracteres de controle e texto em branco são rejeitados.
Todos os valores numéricos devem ser finitos. Os limites superiores de aceitação para valores
positivos são: densidade 1e6 kg/m³, módulos/resistências 1e15 Pa, condutividade 1e9 W/(m K),
capacidade térmica 1e9 J/(kg K), temperatura 20.000 K e taxa de deformação 1e12 /s.
A expansão está dentro de ±1 /K, a umidade em base seca dentro de [0,1e6]
(também acomodando polímeros altamente hidratados) e os coeficientes de Poisson ortotrópicos
dentro de ±1e6, além do teste de estabilidade da matriz de flexibilidade.
São limites técnicos de aceitação amplos, sem representar faixas de aplicabilidade medidas
ou garantia de condicionamento para um futuro solver.

## Exemplo sintético original

Estas constantes deliberadamente inventadas demonstram a validação, não uma classe real:

```json
{
  "card": {
    "schema_version": 1,
    "designation": "Synthetic elastic example — not design data",
    "family": "other",
    "sources": [{
      "id": "example",
      "title": "Original synthetic example",
      "kind": "illustrative",
      "license": "MIT OR Apache-2.0"
    }],
    "properties": [{
      "data": {
        "kind": "elasticity",
        "model": {"symmetry": "isotropic", "young_modulus_pa": 12000000000, "poisson_ratio": 0.25}
      },
      "source_id": "example"
    }]
  }
}
```

O resultado tem G = 4.8e9 Pa, K = 8e9 Pa, condições não especificadas e revisão de fontes
ainda necessária. Não há alterações na massa atribuída, no material salvo nem na aparência.

## Próximas fases e referências

A persistência/integração com a biblioteca precisa de uma migração de documento revisada
separadamente, edições atômicas, desfazer/refazer e reabertura offline, coordenados com #863.
Curvas, incerteza, escalas de dureza, viscoelasticidade e critérios de falha permanecem para
trabalho posterior. O uso visual de PBR/textura/ambiente segue o contrato de renderização
solicitado em #813, com os utilitários de entrada #909/#837; aceitar dados físicos não
anuncia nenhuma dessas capacidades.

O [USDA Wood Handbook](https://research.fs.usda.gov/fpl/wood-handbook), incluindo
[a definição de umidade em base seca do capítulo 4](https://research.fs.usda.gov/download/treesearch/62243.pdf), é uma
referência científica pública para umidade, comportamento mecânico e a distinção entre
propriedades de corpos de prova e valores de projeto para classes de materiais.
Nenhum dado/tabela do manual é incluído. O [modelo de materiais PBR da Khronos](https://www.khronos.org/gltf/pbr)
é uma referência para o trabalho óptico separado. Nenhum código de terceiros, conjunto de
dados de engenharia ou recurso é adicionado por esta implementação.

## Consulta de elasticidade direcional

`material.properties.directional {card, direction: [x,y,z]}` informa o módulo de Young
em Pa para um ensaio constitutivo uniaxial de pequenas deformações ao longo de uma direção
local do material. Ele retorna a direção normalizada de forma segura, os eixos declarados
(`null` para isotropia) e o relatório completo de validação/fontes/condições. Ele não
atribui eixos a um corpo nem prevê deslocamento, falha, resistência ou fator de segurança
de uma peça. Nenhuma capacidade de solver ou renderização é habilitada. A ausência de
elasticidade é recusada, em vez de gerar dados inventados.

Para ortotropia, seja n=(x,y,z) a direção unitária. Com a flexibilidade de cisalhamento de
engenharia 1/G e a convenção de reciprocidade de Poisson acima:

```
1/E(n) = x^4/E1 + y^4/E2 + z^4/E3
       + x²y²(1/G12 - 2 nu12/E1)
       + y²z²(1/G23 - 2 nu23/E2)
       + z²x²(1/G31 - 2 nu31/E3).
```

Isso resulta de aplicar a tensão sigma = n tensor n, calcular a deformação com a matriz
simétrica de flexibilidade e então contrair n com a deformação e n. A implementação usa
uma soma de quadrados de Cholesky para a contribuição normal, evitando cancelamento entre
os termos diagonais positivos e os termos negativos de acoplamento. As contribuições de
cisalhamento também são não negativas. A isotropia retorna E diretamente, independente da orientação.

Direções de entrada devem conter exatamente três números finitos e ser não nulas;
qualquer magnitude é permitida. O escalonamento pelo maior componente evita overflow ou
underflow ao normalizar vetores muito grandes ou muito pequenos. Valores de
flexibilidade/módulo que não podem ser representados são recusados; essa verificação de
precisão finita não certifica o número de condicionamento. As condições de ensaio da ficha
continuam se aplicando; mudar a direção não extrapola temperatura, umidade, processo,
taxa, dano ou comportamento não linear. Testes analíticos sintéticos cobrem eixos
principais, o limite isotrópico e termos de cisalhamento/acoplamento fora dos eixos.
Nenhum conjunto de dados físicos nem implementação externa é incluído.
