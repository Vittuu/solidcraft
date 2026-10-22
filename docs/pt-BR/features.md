# Operações

Os comandos do SolidCraft por área, gerados por `cargo xtask features` a partir do registro de comandos (`solidcraft-cli commands`). **378 comandos.** Planejados (registrados, ainda não implementados): 3. Todos os comandos são executados da mesma forma pela barra de ferramentas, um script, `solidcraft-cli exec <id> '<json>'`, o canal de controle e o MCP; os parâmetros estão na [referência de comandos](book/src/reference/commands.md) (`cargo xtask book`). Estado e planos: [ROADMAP](ROADMAP.md).

## Esboço (103)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Criar esboço | `sketch.create` | SÓLIDO › CRIAR |
| Editar esboço | `sketch.edit` | – |
| Finalizar esboço | `sketch.finish` | ESBOÇO › FINALIZAR ESBOÇO |
| Linha (L) | `sketch.line` | ESBOÇO › CRIAR |
| Desenhar geometria cotada no esboço | `sketch.draw.dimensioned` | – |
| Retângulo por 2 pontos (R) | `sketch.rectangle.two_point` | ESBOÇO › CRIAR |
| Retângulo por 3 pontos | `sketch.rectangle.three_point` | ESBOÇO › CRIAR |
| Retângulo arredondado | `sketch.rectangle.rounded` | ESBOÇO › CRIAR |
| Retângulo por centro | `sketch.rectangle.center` | ESBOÇO › CRIAR |
| Círculo por centro e diâmetro (C) | `sketch.circle.center` | ESBOÇO › CRIAR |
| Círculo por 2 pontos | `sketch.circle.two_point` | ESBOÇO › CRIAR |
| Círculo por 3 pontos | `sketch.circle.three_point` | ESBOÇO › CRIAR |
| Arco por 3 pontos | `sketch.arc.three_point` | ESBOÇO › CRIAR |
| Arco por ponto central | `sketch.arc.center_point` | ESBOÇO › CRIAR |
| Polígono inscrito | `sketch.polygon.inscribed` | ESBOÇO › CRIAR |
| Polígono circunscrito | `sketch.polygon.circumscribed` | ESBOÇO › CRIAR |
| Polígono por aresta | `sketch.polygon.edge` | ESBOÇO › CRIAR |
| Ranhura de centro a centro | `sketch.slot.center_to_center` | ESBOÇO › CRIAR |
| Ranhura por comprimento total | `sketch.slot.overall` | ESBOÇO › CRIAR |
| Ponto | `sketch.point` | ESBOÇO › CRIAR |
| Cota do esboço (D) | `sketch.dimension` | ESBOÇO › CRIAR |
| Horizontal/Vertical | `sketch.constraint.horizontal_vertical` | ESBOÇO › RESTRIÇÕES |
| Coincidente | `sketch.constraint.coincident` | ESBOÇO › RESTRIÇÕES |
| Tangente | `sketch.constraint.tangent` | ESBOÇO › RESTRIÇÕES |
| Igual | `sketch.constraint.equal` | ESBOÇO › RESTRIÇÕES |
| Paralelo | `sketch.constraint.parallel` | ESBOÇO › RESTRIÇÕES |
| Perpendicular | `sketch.constraint.perpendicular` | ESBOÇO › RESTRIÇÕES |
| Fixar/liberar | `sketch.constraint.fix` | ESBOÇO › RESTRIÇÕES |
| Ponto médio | `sketch.constraint.midpoint` | ESBOÇO › RESTRIÇÕES |
| Concêntrico | `sketch.constraint.concentric` | ESBOÇO › RESTRIÇÕES |
| Colinear | `sketch.constraint.collinear` | ESBOÇO › RESTRIÇÕES |
| Simetria | `sketch.constraint.symmetry` | ESBOÇO › RESTRIÇÕES |
| Normal/construção (X) | `sketch.construction` | ESBOÇO › CRIAR |
| Excluir entidades do esboço | `sketch.delete` | – |
| Arrastar ponto do esboço | `sketch.move_point` | – |
| Arrastar geometria do esboço | `sketch.drag` | – |
| Encerrar arraste do esboço | `sketch.drag_end` | – |
| Mover texto da cota | `sketch.dimension_text` | – |
| Opções da paleta de esboço | `sketch.options` | – |
| Cadeia | `sketch.chain` | – |
| Resolver esboço | `sketch.solve` | – |
| Linha por ponto médio | `sketch.line.midpoint` | ESBOÇO › CRIAR |
| Arco tangente | `sketch.arc.tangent` | ESBOÇO › CRIAR |
| Círculo por 2 tangentes | `sketch.circle.two_tangent` | ESBOÇO › CRIAR |
| Círculo por 3 tangentes | `sketch.circle.three_tangent` | ESBOÇO › CRIAR |
| Ranhura por ponto central | `sketch.slot.center_point` | ESBOÇO › CRIAR |
| Ranhura em arco por três pontos | `sketch.slot.arc_three_point` | ESBOÇO › CRIAR |
| Ranhura em arco por ponto central | `sketch.slot.arc_center` | ESBOÇO › CRIAR |
| Elipse | `sketch.ellipse` | ESBOÇO › CRIAR |
| Spline por pontos de ajuste | `sketch.spline.fit_point` | ESBOÇO › CRIAR |
| Spline por pontos de controle | `sketch.spline.control_point` | ESBOÇO › CRIAR |
| Spline por pontos de controle (grau 5) | `sketch.spline.control_point_5` | ESBOÇO › CRIAR |
| Curva cônica | `sketch.conic` | ESBOÇO › CRIAR |
| Curva de transição | `sketch.blend_curve` | ESBOÇO › MODIFICAR |
| Linha de centro | `sketch.centerline` | ESBOÇO › CRIAR |
| Linha infinita | `sketch.infinite` | – |
| Texto | `sketch.text` | ESBOÇO › CRIAR |
| Editar texto | `sketch.edit_text` | – |
| Curvatura | `sketch.constraint.curvature` | ESBOÇO › RESTRIÇÕES |
| Polígono | `sketch.constraint.polygon` | ESBOÇO › RESTRIÇÕES |
| Restrições automáticas | `sketch.auto_constrain` | ESBOÇO › RESTRIÇÕES |
| Restrições automáticas a partir da referência | `sketch.auto_constrain_from_datum` | ESBOÇO › RESTRIÇÕES |
| Concluir com restrições automáticas | `sketch.finish_auto_constrain` | ESBOÇO › FINALIZAR ESBOÇO |
| Aplicador de restrições do esboço | `sketch.constrainer` | ESBOÇO › CRIAR |
| Alternar cota de referência | `sketch.toggle_driven` | – |
| Símbolos de restrição | `sketch.glyphs` | – |
| Inferência de encaixe | `sketch.snap` | – |
| Inserir DXF | `sketch.insert_dxf` | ESBOÇO › INSERIR |
| Inserir SVG | `sketch.insert_svg` | ESBOÇO › INSERIR |
| Salvar como DXF | `sketch.export_dxf` | – |
| Linha 3D | `sketch.line3d` | – |
| Spline 3D | `sketch.spline3d` | – |
| Ponto 3D | `sketch.point3d` | – |
| Mover ponto 3D | `sketch.move3d` | – |
| Arredondamento | `sketch.fillet` | ESBOÇO › MODIFICAR |
| Chanfro com distâncias iguais | `sketch.chamfer.equal_distance` | ESBOÇO › MODIFICAR |
| Chanfro por distância e ângulo | `sketch.chamfer.distance_angle` | ESBOÇO › MODIFICAR |
| Chanfro com duas distâncias | `sketch.chamfer.two_distance` | ESBOÇO › MODIFICAR |
| Deslocar (O) | `sketch.offset` | ESBOÇO › MODIFICAR |
| Desvincular deslocamento | `sketch.offset.break` | – |
| Aparar (T) | `sketch.trim` | ESBOÇO › MODIFICAR |
| Estender | `sketch.extend` | ESBOÇO › MODIFICAR |
| Quebrar | `sketch.break` | ESBOÇO › MODIFICAR |
| Escala do esboço | `sketch.scale` | ESBOÇO › MODIFICAR |
| Mover/copiar entidades do esboço | `sketch.move` | ESBOÇO › MODIFICAR |
| Espelhar | `sketch.mirror` | ESBOÇO › CRIAR |
| Padrão circular | `sketch.pattern.circular` | ESBOÇO › CRIAR |
| Padrão retangular | `sketch.pattern.rectangular` | ESBOÇO › CRIAR |
| Projetar (P) | `sketch.project` | ESBOÇO › CRIAR |
| Interseção | `sketch.intersect` | ESBOÇO › CRIAR |
| Incluir geometria 3D | `sketch.include_3d` | ESBOÇO › CRIAR |
| Projetar na superfície | `sketch.project_to_surface` | ESBOÇO › CRIAR |
| Curva de interseção | `sketch.intersection_curve` | ESBOÇO › CRIAR |
| Perfil de revolução | `sketch.spun_profile` | ESBOÇO › CRIAR |
| Curva isoparamétrica | `sketch.isoparametric_curve` | ESBOÇO › CRIAR |
| Ajustar curvas à seção da malha | `sketch.fit_curves_to_section` | ESBOÇO › CRIAR |
| Desvincular | `sketch.break_link` | – |
| Projetar arestas automaticamente na referência | `sketch.auto_project` | – |
| Redefinir plano do esboço | `sketch.redefine` | – |
| Copiar entidades do esboço | `sketch.copy` | – |
| Recortar entidades do esboço | `sketch.cut` | – |
| Colar entidades do esboço | `sketch.paste` | – |
| Inspecionar esboço | `sketch.inspect` | – |

## Modelagem de sólidos (35)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Extrusão (E) | `solid.extrude` | SÓLIDO › CRIAR |
| Revolução | `solid.revolve` | SÓLIDO › CRIAR |
| Varredura | `solid.sweep` | SÓLIDO › CRIAR |
| Loft | `solid.loft` | SÓLIDO › CRIAR |
| Caixa | `solid.box` | SÓLIDO › CRIAR |
| Cilindro | `solid.cylinder` | SÓLIDO › CRIAR |
| Esfera | `solid.sphere` | SÓLIDO › CRIAR |
| Toro | `solid.torus` | SÓLIDO › CRIAR |
| Arredondamento (F) | `solid.fillet` | SÓLIDO › MODIFICAR |
| Chanfro | `solid.chamfer` | SÓLIDO › MODIFICAR |
| Combinar | `solid.combine` | SÓLIDO › MODIFICAR |
| Padrão retangular | `solid.pattern.rectangular` | SÓLIDO › CRIAR |
| Padrão circular | `solid.pattern.circular` | SÓLIDO › CRIAR |
| Padrão no caminho | `solid.pattern.path` | SÓLIDO › CRIAR |
| Espelhar | `solid.mirror` | SÓLIDO › CRIAR |
| Furo (H) | `solid.hole` | SÓLIDO › CRIAR |
| Tubo | `solid.pipe` | SÓLIDO › CRIAR |
| Sólido envolvente | `solid.bounding_solid` | SÓLIDO › CRIAR |
| Escala | `solid.scale` | SÓLIDO › MODIFICAR |
| Deslocar face | `solid.offset_face` | SÓLIDO › MODIFICAR |
| Rosca | `solid.thread` | SÓLIDO › CRIAR |
| Casca | `solid.shell` | SÓLIDO › MODIFICAR |
| Ângulo de saída | `solid.draft` | SÓLIDO › MODIFICAR |
| Dividir face | `solid.split_face` | SÓLIDO › MODIFICAR |
| Dividir corpo | `solid.split_body` | SÓLIDO › MODIFICAR |
| Pressionar/puxar (Q) | `solid.press_pull` | SÓLIDO › MODIFICAR |
| Mover/copiar (M) | `solid.move` | SÓLIDO › MODIFICAR |
| Preenchimento de limites | `solid.boundary_fill` | SÓLIDO › CRIAR |
| Hélice | `solid.coil` | SÓLIDO › CRIAR |
| Nervura | `solid.rib` | SÓLIDO › CRIAR |
| Rede de nervuras | `solid.web` | SÓLIDO › CRIAR |
| Relevo | `solid.emboss` | SÓLIDO › CRIAR |
| Substituir face | `solid.replace_face` | SÓLIDO › MODIFICAR |
| Alinhar | `solid.align` | SÓLIDO › MODIFICAR |
| Remover | `solid.remove` | SÓLIDO › MODIFICAR |

## Superfícies (15, 2 planejados)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Remendo | `surface.patch` | SUPERFÍCIE › CRIAR |
| Costurar | `surface.stitch` | SUPERFÍCIE › MODIFICAR |
| Espessar | `surface.thicken` | SUPERFÍCIE › CRIAR |
| Estender | `surface.extend` | SUPERFÍCIE › MODIFICAR |
| Aparar | `surface.trim` | SUPERFÍCIE › MODIFICAR |
| Extrusão de superfície | `surface.extrude` | SUPERFÍCIE › CRIAR |
| Revolução de superfície | `surface.revolve` | SUPERFÍCIE › CRIAR |
| Varredura de superfície | `surface.sweep` | SUPERFÍCIE › CRIAR |
| Loft de superfície | `surface.loft` | SUPERFÍCIE › CRIAR |
| Deslocar superfície | `surface.offset` | SUPERFÍCIE › CRIAR |
| Superfície regrada | `surface.ruled` | SUPERFÍCIE › CRIAR |
| Superfície por limites *(planejado: [#763](https://github.com/bherbruck/solidcraft/issues/763))* | `surface.boundary` | SUPERFÍCIE › CRIAR |
| Superfície de transição | `surface.blend` | SUPERFÍCIE › CRIAR |
| Desfazer recorte | `surface.untrim` | SUPERFÍCIE › MODIFICAR |
| Descosturar | `surface.unstitch` | SUPERFÍCIE › MODIFICAR |
| Inverter normal | `surface.reverse_normal` | SUPERFÍCIE › MODIFICAR |
| Arredondamento de superfície *(planejado: [#769](https://github.com/bherbruck/solidcraft/issues/769))* | `surface.fillet` | SUPERFÍCIE › MODIFICAR |

## Geometria de construção (19)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Plano deslocado | `construct.plane.offset` | SÓLIDO › CONSTRUÇÃO |
| Plano em ângulo | `construct.plane.at_angle` | SÓLIDO › CONSTRUÇÃO |
| Eixo através de cilindro/cone/toro | `construct.axis.through_cylinder` | SÓLIDO › CONSTRUÇÃO |
| Eixo perpendicular à face | `construct.axis.normal_to_face` | SÓLIDO › CONSTRUÇÃO |
| Eixo através de dois planos | `construct.axis.two_planes` | SÓLIDO › CONSTRUÇÃO |
| Eixo através de dois pontos | `construct.axis.two_points` | SÓLIDO › CONSTRUÇÃO |
| Eixo através de aresta | `construct.axis.edge` | SÓLIDO › CONSTRUÇÃO |
| Ponto no vértice | `construct.point.vertex` | SÓLIDO › CONSTRUÇÃO |
| Ponto na interseção de duas arestas | `construct.point.two_edges` | SÓLIDO › CONSTRUÇÃO |
| Ponto na interseção de três planos | `construct.point.three_planes` | SÓLIDO › CONSTRUÇÃO |
| Ponto no centro de círculo/esfera/toro | `construct.point.center` | SÓLIDO › CONSTRUÇÃO |
| Ponto na interseção de aresta e plano | `construct.point.edge_plane` | SÓLIDO › CONSTRUÇÃO |
| Ponto ao longo de caminho | `construct.point.along_path` | SÓLIDO › CONSTRUÇÃO |
| Plano intermediário | `construct.plane.midplane` | SÓLIDO › CONSTRUÇÃO |
| Plano através de duas arestas | `construct.plane.two_edges` | SÓLIDO › CONSTRUÇÃO |
| Plano através de três pontos | `construct.plane.three_points` | SÓLIDO › CONSTRUÇÃO |
| Plano tangente | `construct.plane.tangent` | SÓLIDO › CONSTRUÇÃO |
| Plano ao longo de caminho | `construct.plane.along_path` | SÓLIDO › CONSTRUÇÃO |
| Plano perpendicular | `construct.plane.perpendicular` | SÓLIDO › CONSTRUÇÃO |

## Chapas metálicas (12)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Flange | `sheet.flange` | CHAPA METÁLICA › CRIAR |
| Perfurar | `sheet.perforate` | CHAPA METÁLICA › CRIAR |
| Bainha | `sheet.hem` | CHAPA METÁLICA › CRIAR |
| Dobrar | `sheet.fold` | CHAPA METÁLICA › CRIAR |
| Desdobrar | `sheet.unfold` | CHAPA METÁLICA › MODIFICAR |
| Dobrar novamente | `sheet.refold` | CHAPA METÁLICA › MODIFICAR |
| Converter para chapa metálica | `sheet.convert` | CHAPA METÁLICA › CRIAR |
| Regras de chapa metálica | `sheet.manage_rules` | CHAPA METÁLICA › MODIFICAR |
| Listar regras de chapa metálica | `sheet.rules` | – |
| Criar planificação | `sheet.flat_pattern` | CHAPA METÁLICA › CRIAR |
| Exportar planificação como DXF | `sheet.export_dxf` | – |
| Exportar tabela de dobras | `sheet.export_bend_table` | – |

## Peças plásticas (7)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Ressalto | `plastic.boss` | PLÁSTICO › CRIAR |
| Rebordo | `plastic.lip` | PLÁSTICO › CRIAR |
| Encaixe por pressão | `plastic.snap_fit` | PLÁSTICO › CRIAR |
| Suporte | `plastic.rest` | PLÁSTICO › CRIAR |
| Gerenciar regras de plástico | `plastic.manage_rules` | PLÁSTICO › PREPARAÇÃO |
| Atribuir regra de plástico | `plastic.assign_rule` | PLÁSTICO › PREPARAÇÃO |
| Listar regras de plástico | `plastic.rules` | – |

## Componentes (17)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Novo componente | `component.create` | SÓLIDO › MONTAGEM |
| Criar componentes a partir de corpos | `component.from_bodies` | – |
| Ativar componente | `component.activate` | – |
| Listar componentes | `component.list` | – |
| Propriedades físicas do componente | `component.properties` | – |
| Renomear componente | `component.rename` | – |
| Excluir componente | `component.delete` | – |
| Desbloquear conteúdo | `component.unlock` | – |
| Mover corpos para componente | `component.move_bodies` | – |
| Mover esboços para componente | `component.move_sketches` | – |
| Colar como novo | `component.paste_new` | – |
| Registrar posição | `component.capture_position` | SÓLIDO › POSIÇÃO |
| Reverter posição | `component.revert_position` | SÓLIDO › POSIÇÃO |
| Inserir projeto | `component.insert_design` | SÓLIDO › MONTAGEM |
| Atualizar vínculos | `component.update_links` | – |
| Desvincular | `component.break_link` | – |
| Listar vínculos | `component.links` | – |

## Juntas de montagem (14)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Origem da junta | `joint.origin` | SÓLIDO › CRIAR |
| Junta (J) | `joint.create` | SÓLIDO › MONTAGEM |
| Junta na posição atual | `joint.as_built` | SÓLIDO › MONTAGEM |
| Grupo rígido | `joint.rigid_group` | SÓLIDO › MONTAGEM |
| Acionar juntas | `joint.drive` | SÓLIDO › MONTAGEM |
| Vínculo de movimento | `joint.motion_link` | SÓLIDO › MONTAGEM |
| Engrenagem | `joint.gear` | SÓLIDO › MONTAGEM |
| Pinhão e cremalheira | `joint.rack_pinion` | SÓLIDO › MONTAGEM |
| Fuso | `joint.screw` | SÓLIDO › MONTAGEM |
| Limites da junta | `joint.limits` | – |
| Editar junta | `joint.edit` | – |
| Excluir junta | `joint.delete` | – |
| Listar juntas | `joint.list` | – |
| Resolver juntas | `joint.solve` | – |

## Configurações (8)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Configurar | `config.configure` | SÓLIDO › CONFIGURAR |
| Exibir tabela de configurações | `config.table` | SÓLIDO › CONFIGURAR |
| Coluna de configuração | `config.column` | – |
| Linha de configuração | `config.row` | – |
| Ativar configuração | `config.activate` | – |
| Prévia da configuração | `config.preview` | – |
| Regra de configuração | `config.rule` | – |
| Regras de configuração | `config.rules` | – |

## Linha do tempo (21)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Calcular tudo | `timeline.compute_all` | SÓLIDO › MODIFICAR |
| Excluir (Delete) | `timeline.delete` | SÓLIDO › MODIFICAR |
| Renomear | `timeline.rename` | – |
| Nova pasta de operações | `timeline.group` | – |
| Remover pastas de operações | `timeline.ungroup` | – |
| Renomear pasta de operações | `timeline.rename_group` | – |
| Mover operações para pasta | `timeline.move_to_group` | – |
| Mover itens da linha do tempo | `timeline.move_many` | – |
| Mover marcador da linha do tempo | `timeline.rollback` | – |
| Posicionar marcador do histórico | `timeline.roll_to` | – |
| Reordenar operação | `timeline.reorder` | – |
| Reordenar operações | `timeline.reorder_many` | – |
| Editar operação | `timeline.redefine` | – |
| Dependentes da operação | `timeline.dependents` | – |
| Suprimir operação | `timeline.suppress` | – |
| Editar operação | `timeline.edit` | – |
| Inspecionar operação | `timeline.inspect` | – |
| Copiar operações | `timeline.copy` | – |
| Colar operações | `timeline.paste` | – |
| Adicionar operação registrada | `timeline.add` | – |
| Exportar script de comandos | `timeline.export_script` | – |

## Inspeção (13, 1 planejado)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Análise de pente de curvatura | `inspect.curvature_comb` | ESBOÇO › INSPECIONAR |
| Análise de raio mínimo | `inspect.minimum_radius` | ESBOÇO › INSPECIONAR |
| Análise de isocurvas | `inspect.isocurve` | ESBOÇO › INSPECIONAR |
| Centro de massa | `inspect.center_of_mass` | ESBOÇO › INSPECIONAR |
| Análise de listras zebra | `inspect.zebra` | ESBOÇO › INSPECIONAR |
| Análise de ângulo de saída | `inspect.draft` | ESBOÇO › INSPECIONAR |
| Análise de mapa de ambiente | `inspect.environment_map` | ESBOÇO › INSPECIONAR |
| Análise de acessibilidade | `inspect.accessibility` | ESBOÇO › INSPECIONAR |
| Análise de mapa de curvatura | `inspect.curvature_map` | ESBOÇO › INSPECIONAR |
| Análise de continuidade *(planejado: [#770](https://github.com/bherbruck/solidcraft/issues/770))* | `inspect.continuity` | SUPERFÍCIE › INSPECIONAR |
| Interferência | `inspect.interference` | SÓLIDO › INSPECIONAR |
| Medir (I) | `inspect.measure` | SÓLIDO › INSPECIONAR |
| Comparar corpos | `inspect.compare` | – |
| Análise de seção | `inspect.section` | SÓLIDO › INSPECIONAR |

## Arquivos (9)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Novo projeto (Ctrl+N) | `file.new` | – |
| Inserir STEP | `file.insert_step` | SÓLIDO › INSERIR |
| Inserir malha | `file.insert_mesh` | SÓLIDO › INSERIR |
| Salvar (Ctrl+S) | `file.save` | – |
| Salvar como | `file.save_as` | – |
| Exportar | `file.export` | – |
| Salvar como malha | `file.save_mesh` | MALHA › MODIFICAR |
| Fatiadores instalados | `file.slicers` | – |
| Exportar vista atual como DXF | `file.export_view_dxf` | – |

## Documentos (4)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Abrir (Ctrl+O) | `doc.open` | – |
| Projetos recuperáveis | `doc.recovery_list` | – |
| Recuperar projeto | `doc.recover` | – |
| Descartar projeto recuperado | `doc.recovery_discard` | – |

## Edição (4)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Desfazer (Ctrl+Z) | `edit.undo` | – |
| Refazer (Ctrl+Y) | `edit.redo` | – |
| Ponto de restauração | `edit.checkpoint` | – |
| Restaurar ponto de restauração | `edit.restore_checkpoint` | – |

## `appearance.*` (3)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Aparência (A) | `appearance.assign` | SÓLIDO › MODIFICAR |
| Biblioteca de aparências | `appearance.library` | – |
| Listar aparências | `appearance.list` | – |

## `body.*` (1)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Renomear corpo | `body.rename` | – |

## `browser.*` (6)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Novo grupo | `browser.group` | – |
| Desagrupar | `browser.ungroup` | – |
| Renomear grupo | `browser.rename_group` | – |
| Mover itens da árvore do modelo | `browser.move` | – |
| Ordenar itens da árvore do modelo | `browser.order` | – |
| Mostrar/ocultar | `browser.visibility` | – |

## `canvas.*` (5)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Imagem de referência | `canvas.insert` | ESBOÇO › INSERIR |
| Decalque | `canvas.decal` | ESBOÇO › INSERIR |
| Editar imagem de referência | `canvas.edit` | ESBOÇO › INSERIR |
| Calibrar imagem de referência | `canvas.calibrate` | – |
| Excluir imagem de referência | `canvas.delete` | – |

## `contact.*` (7)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Ativar conjuntos de contato | `contact.enable_sets` | SÓLIDO › MONTAGEM |
| Ativar todos os contatos | `contact.enable_all` | SÓLIDO › MONTAGEM |
| Desativar contato | `contact.disable_all` | SÓLIDO › MONTAGEM |
| Novo conjunto de contato | `contact.create` | SÓLIDO › MONTAGEM |
| Editar conjunto de contato | `contact.edit` | – |
| Listar conjuntos de contato | `contact.list` | – |
| Verificar contato | `contact.check` | – |

## `diagnostics.*` (4)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Relatórios de falha | `diagnostics.crash_reports` | – |
| Relatório de falha | `diagnostics.crash_report` | – |
| Excluir relatórios de falha | `diagnostics.crash_reports.clear` | – |
| Teste de falha | `diagnostics.crash_test` | – |

## `document.*` (5)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Alterar unidades do projeto | `document.units` | – |
| Propriedades do projeto | `document.metadata` | – |
| Alterar propriedades do projeto | `document.metadata.set` | – |
| Remover propriedade do projeto | `document.metadata.remove` | – |
| Inspecionar projeto | `document.inspect` | – |

## `engine.*` (1)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Listar comandos | `engine.commands` | – |

## `explode.*` (4)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Vista explodida | `explode.create` | SÓLIDO › MONTAGEM |
| Mostrar vista explodida | `explode.show` | – |
| Listar vistas explodidas | `explode.list` | – |
| Excluir vista explodida | `explode.delete` | – |

## `expr.*` (1)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Avaliar expressão | `expr.evaluate` | – |

## `face.*` (1)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Excluir face | `face.delete` | SÓLIDO › MODIFICAR |

## `library.*` (3)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Inspecionar biblioteca local | `library.inspect` | – |
| Exportar biblioteca do projeto | `library.export` | – |
| Aplicar entrada de biblioteca local | `library.apply` | – |

## `material.*` (7)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Material físico | `material.assign` | SÓLIDO › MODIFICAR |
| Listar materiais | `material.list` | – |
| Novo material físico | `material.create` | – |
| Editar material físico | `material.edit` | – |
| Excluir material físico | `material.delete` | – |
| Validar dados físicos de materiais | `material.properties.validate` | – |
| Módulo de elasticidade direcional | `material.properties.directional` | – |

## `mesh.*` (1)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Converter malha | `mesh.convert` | – |

## `model.*` (3)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Listar arestas | `model.edges` | – |
| Listar faces | `model.faces` | – |
| Listar roscas | `model.threads` | – |

## `motion.*` (5)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Estudo de movimento | `motion.study` | SÓLIDO › MONTAGEM |
| Reproduzir estudo de movimento | `motion.play` | – |
| Exportar estudo de movimento | `motion.export` | – |
| Listar estudos de movimento | `motion.list` | – |
| Excluir estudo de movimento | `motion.delete` | – |

## `occurrence.*` (4)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Propriedades físicas da ocorrência | `occurrence.properties` | – |
| Colar (instância) | `occurrence.copy` | – |
| Mover ocorrência | `occurrence.move` | – |
| Fixar | `occurrence.ground` | – |

## `parameters.*` (13)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Alterar parâmetros | `parameters.change` | SÓLIDO › MODIFICAR |
| Listar parâmetros | `parameters.list` | – |
| Adicionar parâmetro do usuário | `parameters.add` | – |
| Renomear parâmetro | `parameters.rename` | – |
| Excluir parâmetro | `parameters.delete` | – |
| Favoritar parâmetro | `parameters.favorite` | – |
| Mover parâmetro | `parameters.move` | – |
| Comentar parâmetro | `parameters.comment` | – |
| Usos do parâmetro | `parameters.users` | – |
| Grafo de parâmetros | `parameters.graph` | – |
| Exportar parâmetros | `parameters.export` | – |
| Importar parâmetros | `parameters.import` | – |
| Completar expressão | `parameters.complete` | – |

## `parts.*` (4)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Peças padronizadas | `parts.library` | – |
| Inserir fixador | `parts.fastener` | SÓLIDO › INSERIR |
| Inserir peça | `parts.insert` | – |
| Alterar tamanho da peça | `parts.resize` | – |

## `render.*` (10)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Verificar solicitação de renderização | `render.preflight` | – |
| Renderizar imagem | `render.start` | RENDER › IMAGEM |
| Progresso da renderização | `render.status` | – |
| Cancelar renderização | `render.cancel` | – |
| Exportar imagem renderizada | `render.export` | – |
| Sistemas de renderização | `render.backends` | – |
| Configurações de renderização | `render.settings.get` | – |
| Salvar configurações de renderização | `render.settings.set` | – |
| Inspecionar cena de renderização | `render.scene` | RENDER › CENA |
| Aplicar câmera de renderização | `render.settings.apply` | – |

## `select.*` (2)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Selecionar | `select.set` | – |
| Limpar seleção | `select.clear` | – |

## `selection.*` (1)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Excluir | `selection.delete` | – |

## `view.*` (6)

| Comando | ID | Barra de ferramentas |
|---|---|---|
| Nova vista nomeada | `view.save` | – |
| Renomear vista nomeada | `view.rename` | – |
| Excluir vista nomeada | `view.delete` | – |
| Listar vistas nomeadas | `view.list` | – |
| Restaurar vista nomeada | `view.apply` | – |
| Navegar pela vista | `view.navigate` | – |
