# Adaptando um modelo para o SolidCraft

Para um modelo que já existe como código (build123d, CadQuery, OpenSCAD) ou como STEP. A adaptação está
concluída quando a exportação STEP do SolidCraft corresponde à geometria de origem para todos os valores de parâmetros
verificados: `compare_step.py` informa xor de no máximo 1e-3 mm3. Apenas volume, área e centro de massa
não bastam: em uma adaptação, um hexágono com dof 1 alterou o volume em 2e-7 do total e o
centro de massa em 3e-4 mm, enquanto xor mostrou 56 mm3 em 96 pedaços ao longo das paredes externas.

1. **Geometria de referência no sistema de coordenadas do projeto.** A origem do SolidCraft é escolhida pela armadilha 1 de SKILL.md, por isso
   costuma diferir da origem. Translade o modelo de origem para o mesmo sistema de coordenadas e exporte como
   STEP para cada valor de parâmetro verificado. Escolha valores que alcancem todos os ramos da lógica de layout.
2. **Diferenças deliberadas.** Lógica que as expressões do SolidCraft não comportam (condicionais,
   busca) recebe uma fórmula fechada. Verifique primeiro a fórmula com a origem em um amplo intervalo de
   valores em Python simples, liste onde difere e aplique a fórmula também à referência
   (altere a função de origem), para que a comparação continue equivalente. Informe cada diferença
   ao usuário.
3. **Exportar e comparar.** Adicione `parameters.change` e `file.export {path, bodies: [name]}` por
   valor a um script, execute-o e depois
   `uv run --no-project --with build123d python $SKILL/scripts/compare_step.py actual.step reference.step`.
   As caixas dos pedaços mostram onde as formas diferem; um par de pedaços nas duas metades de uma face
   significa que ela está girada, geralmente por um esboço com restrições insuficientes.
4. **Fixar o teste.** Meça a referência (volume, área, centro de massa, caixa delimitadora) e
   coloque os números na cópia de `templates/test_design.py` ao lado do projeto. Peça autorização ao usuário
   antes de remover o modelo de origem; o histórico git o preserva.
