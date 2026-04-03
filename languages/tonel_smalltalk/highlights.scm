; Tonel definition keywords
["Class" "Trait" "Extension" "Package"] @keyword

; class-side keyword in method reference
(method_reference "class" @keyword)

; Method reference
(method_reference class_name: (identifier) @type)
(method_reference ">>" @operator)

; Selectors
(unary_selector (unary_identifier) @function.method)
(binary_selector (binary_operator) @operator)
(method_reference (keyword) @function.method)

; STON keys (metadata properties)
(ston_key (ston_symbol) @property)
(ston_key (string) @property)

; STON values
(ston_symbol) @string.special
(ston_list) @punctuation.bracket

; Smalltalk literals
(string) @string
(symbol) @string.special
(number) @number
(character) @character
(class_comment) @comment
(comment) @comment

; Pseudo-variables / built-ins
(true) @constant.builtin
(false) @constant.builtin
(nil) @constant.builtin
(self) @variable.builtin
(super) @variable.builtin
(thisContext) @variable.builtin

; Return operator
"^" @operator

; Cascade
";" @punctuation.delimiter

; Statement separator
"." @punctuation.delimiter

; Blocks and collections
["[" "]" "(" ")" "{" "}"] @punctuation.bracket

; Pragma
["<" ">"] @punctuation.bracket
(pragma_unary_selector (unary_identifier) @attribute)
(pragma_keyword_selector (keyword) @attribute)

; Temporaries delimiters
(temporaries "|" @punctuation.delimiter)

; Assignment operator
":=" @operator

; Binary operators in expressions
(binary_message (binary_operator) @operator)
(cascaded_binary_message (binary_operator) @operator)
