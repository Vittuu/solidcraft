# UI localization

SolidCraft embeds its translations and works offline. The shared registry includes 25 locales:
`en`, `cs`, `da`, `de`, `en-GB`, `es`, `fr`, `gl`, `hu`, `id`, `it`, `ja`, `ko`, `ms`, `nl`, `pl`, `pt-BR`, `ro`, `ru`, `sv`, `tr`, `uk`, `vi`, `zh`, `zh-TW`.
They are registered in
`crates/engine/src/i18n.rs`. Adding a language requires an
embedded catalog and one registration there; the preferences selector and validation use the same
registry.

The UI re-exports the shared immutable catalogs and supplies its render scopes. CLI and MCP
presentation use the same locale registry without depending on the graphical front end.
`solidcraft-cli --language pt-BR --help` selects Brazilian Portuguese; `SOLIDCRAFT_LANGUAGE`
is the equivalent environment setting. Machine-readable command IDs, property names and enum
values retain their canonical spelling. Protocol descriptions containing literal JSON examples
use the plain source-text dictionary in `assets/locales/protocol/pt-BR.json`, independently of
the UI's named-placeholder templates.

The desktop host also accepts `solidcraft --language pt-BR` (or `SOLIDCRAFT_LANGUAGE`),
including its terminal help and crash-report summary. An explicit launch language overrides
stored preferences. The browser accepts `?language=pt-BR`; its loading message, page language,
metadata and application preference use that valid explicit choice. The HTML preloader
uses the small `solidcraft-language` cache only for startup text while the Rust application
loads. Once the application starts, its stored RON preferences are authoritative; a valid
language query overrides the loaded preference. The cache is then refreshed from the
application's selected language.

Parameter help is keyed by canonical command ID in `assets/locales/commands/pt-BR.json`.
Both `solidcraft-cli commands` and MCP command catalogs translate presentation fields only.
MCP searches accept localized captions and the original labels/IDs. The Portuguese book's
generated command reference uses the same dictionary and fails on missing parameter help.

The Portuguese manual also localizes mdBook's navigation, keyboard help, theme selector,
search and clipboard messages, redirects and 404 page through `assets/locales/book/pt-BR.json`.
The Rust book generator preserves authored chapter content and code, updates content hashes
for translated scripts and rejects catalog entries absent from the generated mdBook controls.

## Messages

`assets/locales/en.json` defines stable message IDs and English fallback text.
Keep IDs when changing wording. Other catalogs use the same IDs; missing messages fall back
to English. Use `app.tr("ui.preferences")` for static messages and
`Locale::format` for full sentences with named placeholders. Translate the entire sentence,
including punctuation, so a translator can reorder its arguments. Placeholder values are
inserted literally, even if a document name contains braces.

The English-label adapter (`app.tr_label`) supports existing engine and UI tables. It maps
display text to a message ID, then uses the same catalog lookup. New UI messages should use
stable IDs directly. The adapter does not change command IDs, model names, or scenario handles.
When a table's English wording changes, update the corresponding English catalog entry too.
Generated browser labels use named templates while their underlying model names stay intact.

Catalogs are parsed once and looked up through ordered maps. This is offline localization,
not a claim of zero runtime overhead. The catalog tests check registered locale IDs,
translation keys, and placeholder consistency. Run `cargo xtask ci` before submitting changes.

## Terms

The [glossary](book/src/reference/glossary.md) gives one English term for each idea, and each
entry has a note for translators: what the term must not be confused with. Its "Words with more
than one meaning" table lists the English words that need a separate catalogue entry for each
meaning (Construction, Flat, Round, Normal, Gap, Clearance).

## English (UK)

`en-GB.json` holds only the English messages whose spelling differs in UK English (Centre,
Customise, Organisation, Licence, colours, …); every other message falls back to English. When an
English message with a US spelling is added, add its UK spelling there too.

## Coverage

A message a locale never received falls back to English silently, so a gap shows up only as a row
that looks untranslated. Two guards catch that in the build:

- `every_complete_catalog_carries_every_english_message` — adding a key to `en.json`
  without giving it to a complete catalog fails the tests. English is the source catalog,
  Japanese, Chinese and Brazilian Portuguese are delivered complete; German, Korean, Russian, French,
  Ukrainian and Turkish are partial — their gaps fall back to English and are reported by
  `cargo xtask i18n` without failing the build. A new label needs en, ja, zh, pt-BR, de and ko;
  Russian, French, Ukrainian and Turkish may follow later.
- `every_toolbar_tab_and_panel_is_in_the_english_catalog` — tabs and panels reach the UI as bare
  source strings, resolved only by the English-label adapter, so a heading missing from `en.json`
  can never translate.

Command labels are the same case on a larger scale: every label in the engine's registry now has
an entry under `cmd.<command id>`, and `every_command_label_is_in_the_english_catalog` fails if a
command is added without one.

The context menu and the browser are the same case again, and they carry the most labels of all:
`every_menu_label_is_in_the_english_catalog` opens every menu the app can reach, flips the state
each conditional label branches on (origin shown, bodies locked, bodies hidden) so both branches
are seen, and fails on any label the catalog has never seen. Unit symbols are left out of the
sweep: `mm` is `mm` in every language, so they are not prose to translate. The sweep also asserts
it keeps reaching a broad set of labels, so it cannot pass by finding nothing.

`cargo xtask i18n` prints the same picture across every locale — which messages each one is
missing and which command labels English cannot translate. `--check` turns it into a non-zero exit
for CI. `--add-missing` writes each uncovered label into `en.json` under `cmd.<command id>`, with
the English label as its text, so the label adapter can find it and only the translations are left
to do. One key per distinct label: two commands that share a label (`Trim`, `Extend`) must not both
claim it, or the English-text lookup would silently pick one.

## Rendering and coverage

The Italian catalog uses original translations of SolidCraft's English messages, not text
extracted from commercial CAD software. Its core terminology is Schizzo (Sketch),
Estrusione (Extrude), Raccordo (Fillet), Smusso (Chamfer), Corpo (Body), Componente
(Component) and Vincolo (Constraint). Draft geometry is Sformo, while a report draft
is Bozza; Counterbore is Lamatura and Countersink is Svasatura. Established technical
terms such as Loft, Sweep, Spline and Offset remain unchanged. Command IDs, shortcuts,
file formats, units and user-provided names are not translated.

One immutable `Locale` type handles all translations. Table widgets without an app reference
use a render scope that restores the previous locale when it exits, including nested renders.
Standalone translated panels enter the same scope. Measure rendered text to size custom
widgets; UTF-8 byte lengths do not describe their displayed width.

The desktop host tries system CJK fonts across Windows, macOS and Linux, as well as an optional
`CRAFT_FONTS_DIR` font pack, and skips files that cannot be parsed. Chinese uses its own fallback
chain after the general CJK font: many Japanese CJK fonts (Hiragino Gothic, Meiryo, Yu Gothic)
match the CJK font search but lack simplified characters, so a second fallback found by the
Chinese font-name list (`PingFang`, `Microsoft YaHei`, `Noto Sans CJK SC`, …) covers them per
glyph. No fonts are redistributed in native desktop distribution packages; native builds rely
entirely on existing operating system fonts or craft-fonts.

In the browser, `solidcraft-web` bundles a self-hosted Japanese font (`BIZUDPGothic-Regular.ttf`, licensed
under SIL OFL 1.1) within the web distribution artifacts only. The font is loaded asynchronously from the same
origin (`./fonts/BIZUDPGothic-Regular.ttf`) on-demand when the interface language is a CJK one, or, with
any other interface language, the first time Japanese text (kana, kanji or full-width forms in a design name,
file name, body name or typed text) is drawn, without contacting any external CDNs or third-party servers.
A session that never shows such text downloads no font. If font loading fails with a CJK interface, the web
interface shows English for that session only to prevent missing glyphs (tofu); the saved language
preference is kept, so the next visit tries again. The bundled font specifically targets Japanese; it also
renders the kanji Chinese shares with Japanese, while Korean (Hangul) and simplified-only Chinese characters
are not covered, so a self-hosted SC/Korean font bundle remains follow-up work for full web coverage.

The current messages support named substitution, not locale-specific plural selection,
number/date formatting, or right-to-left layout and shaping. Add those capabilities when
introducing messages or languages that need them; use an established message formatter for
plural rules instead of adding language branches to panels. User-authored model names, command
IDs, expressions, units and machine-readable model data keep their canonical spelling.
Brazilian Portuguese translates project-authored diagnostic prose when presenting errors in
the UI, CLI and MCP. Object names, IDs, expressions, file paths and supplied data are not
translated. Opaque operating-system, third-party and debug
details also remain literal; the surrounding project-authored explanation is localized.

Project-owned diagnostic templates live in `assets/locales/diagnostics/pt-BR.json`.
`assets/locales/diagnostics/arguments.json` identifies only arguments proved to contain
project-owned diagnostic prose. Those arguments may be translated recursively within
the matcher's byte, slot, step and depth limits. Other arguments remain literal; unknown
or ambiguous messages also retain their original text.

`cargo xtask --language pt-BR <command>` selects Portuguese human reports, and
`SOLIDCRAFT_LANGUAGE` also applies. Arguments after the command continue to reach its
subprocesses without language-option rewriting.

## Messages with distinct contexts

The navigation action `ui.back` (Back) and cube label `view.back` (BACK) are separate.
`measure.at` is a label; `measure.coordinates` retains the named x/y/z template used by
the current measurement dialog. Toolbar and left-browser descriptions have their own
message IDs, so adopting the newer application-bar and vertical-history messages does not
change the descriptions of the current layout. UK English is a spelling overlay and inherits
unchanged text from English. Other partial catalogs keep the same English fallback.
