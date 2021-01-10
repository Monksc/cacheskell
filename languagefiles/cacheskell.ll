
program             -> manyWhitespace multipleFuncs;
multipleFuncs       -> function manyWhitespace multipleFuncs | epsilon;

function            -> identifier atleastOneWhitespace funtionParameters multipleFuncCases manyWhitespace ';';
funtionParameters   -> identifier atleastOneWhitespace funtionParameters | epsilon;

multipleFuncCases   -> functionCases manyWhitespace multipleFuncCasesT;
multipleFuncCasesT  -> functionCases manyWhitespace multipleFuncCasesT | epsilon;
functionCases       -> '|' manyWhitespace boolExpression manyWhitespace '=' manyWhitespace expression;

functionCall        -> '(' manyWhitespace functionCallExpression manyWhitespace functionCallT ')';
functionCallT       -> expression manyWhitespace functionCallT | epsilon;

functionCallExpression -> identifier | functionCall;
boolExpression      -> expression;
expression          -> functionCall | variable;
variable            -> bool | integer | identifier;

bool                -> 'true' | 'false';

integer             -> digit integerTail;
integerTail         -> digit integerTail | epsilon;

digit               -> '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';

identifier          -> letter identifiertail;
identifiertail      -> letter identifiertail | digit identifiertail | epsilon;

letter              -> uppercase | lowercase;
lowercase           -> 'q' | 'w' | 'e' | 'r' | 't' | 'y' | 'u' | 'i' | 'o' | 'p' | 'a' | 's' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'z' | 'x' | 'c' | 'v' | 'b' | 'n' | 'm';
uppercase           -> 'Q' | 'W' | 'E' | 'R' | 'T' | 'Y' | 'U' | 'I' | 'O' | 'P' | 'A' | 'S' | 'D' | 'F' | 'G' | 'H' | 'J' | 'K' | 'L' | 'Z' | 'X' | 'C' | 'V' | 'B' | 'N' | 'M';


atleastOneWhitespace    -> whitespace manyWhitespace;
manyWhitespace      -> whitespace manyWhitespace | epsilon;
whitespace          -> ' ' | '\n' | '\t';

epsilon             -> '';


