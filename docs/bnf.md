# BNF

Here's Hatter in fake BNF:

```
stmt = if | for | while | def | assign | expr
if = 'if' expr block ('else' ('if' expr)? block)*
for = 'for' (word ',')? word 'in' expr block
while = 'while' expr block
def = 'def' word '(' word (',' word)* ')' block
assign = word (':=' | '=') expr
expr = call | op-call | tag | atom | ( '(' expr ')' )
call = word '(' (expr (',' expr)* )? ')'
op-call = expr op expr
tag = open-tag | close-tag
open-tag = '<' word? shorthand* attr* '>'
close-tag = '<' '/' word? '>'
shorthand = ('#' | '.' | ':' | '@') word
attr = word ('=' expr)?
atom = bool | num | string | word
bool = 'true' | 'false'
num = '-'? 0..9 ('.' 0..9+)?
string = ('"' [^"]* '"') | ('\'' [^\']* '\'') | ('`' [^`]* '`')
big-string = ('"""' [^(""")]* '"""') |
word = [\S]+
op = [\S\W\D]+
block = indent stmt+ dedent
```
