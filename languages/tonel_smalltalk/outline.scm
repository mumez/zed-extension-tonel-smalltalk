; Outline: one entry per Tonel method. @open / @close on `[` `]` let the outline
; panel treat the method body as a collapsible range (same pattern as Rust
; `function_item` in Zed's bundled grammars).

(method_definition
  (method_metadata)? @annotation
  (method_reference) @name
  .
  "[" @open
  (method_body)?
  .
  "]" @close) @item
