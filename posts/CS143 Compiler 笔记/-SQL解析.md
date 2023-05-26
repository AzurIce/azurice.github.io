## 注释

```lex
%option noyywrap nodefault yylineno case-insensitive

%{
void yyerror(char *s, ...);
%}

%s COMMENT

%%
    /* comments */
"--"[ \t].*      ;
"/*"             { old_state = YY_START; BEGIN COMMENT; }
<COMMENT>"*/"    { BEGIN old_state; }
<COMMENT>.|\n    ;
<COMMENT><<EOF>> { yyerror("lexer: unclosed comment"); }

    /* everything else */
[ \t\n]          ;
.                { yyerror("lexer: mystery character: '%c'", *yytext); }
%%

```



## 基本词法单元

```lex
    /* numbers */
-?[0-9]+           { return INTEGER; }

-?[0-9]+"."[0-9]*
| -?"."[0-9]+      { return FLOAT; }

    /* strings */
'(\\.|''|[^'\n])*' { return STRING; }

    /* operators */
[-+*/%]            { return yytext[0]; }

%left '+' '-'
%left '*' '/' '%'
%precedence NEG   /* negation--unary minus */

	/* comparators */
"<"  { yylval.subtok = 1; return CMP_LESS; }
">"  { yylval.subtok = 2; return CMP_GREATER; }
"="  { yylval.subtok = 4; return CMP_EQ; }
"<=" { yylval.subtok = 5; return CMP_LEQ; }
">=" { yylval.subtok = 6; return CMP_GEQ; }
"!=" { yylval.subtok = 4; return CMP_NEQ; }
"<>" { yylval.subtok = 3; return CMP_NEQ; }
```

## SELECT 语句

### 1. 最简单的 SELECT 语句

```yacc
%union {
	int intval;
	double floatval;
	char *strval;
}

%token <intval> INTEGER
%token <floatval> FLOAT
%token <strval> STRING
%token <intval> BOOLEAN
```



最简单的 SELECT 语句相当于对表达式进行计算并输出结果：

```sql
SELECT value_expr_list;
```

其中 `value_expr_list` 为一个或多个以逗号隔开的值表达式：

```yacc
value_expr_list := value_expr
                 | value_expr ',' value_expr
                 ;
```

#### > 值表达式

对于值表达式（`value_expr`），大概可以根据数值类型分为三类：

- 算数表达式（`arithmetic_expr`）
- 字符串表达式（`string-expr`）
- 布尔值表达式（`boolean_expr`）

```yacc
value_expr := arithmetic_expr
            | string_expr
            | boolean_expr
            ;
```

##### 1 | 算数表达式

首先，对于一般的算数表达式（我们称其为 `arithmetic_expr`）它可以是单个整数、小数：

```
arithmetic_expr := INTEGER
                 | FLOAT
                 ;
```

此外，表达式之间进行基本的加减乘除取模、布尔运算、比较运算也仍旧是一个表达式：

```
arithmetic_expr := arithmetic_expr '+' arithmetic_expr
                 | arithmetic_expr '-' arithmetic_expr
                 | arithmetic_expr '*' arithmetic_expr
                 | arithmetic_expr '/' arithmetic_expr
                 | arithmetic_expr '%' arithmetic_expr
                 | '-' arithmetic_expr  %prec NEG
                 | '(' arithmetic_expr ')'
                 ;
```

##### 2 | 字符串表达式

```
string_expr := STRING;
```

##### 3 | 布尔值表达式

```
boolean_expr := BOOLEAN
			  | arithmetic_expr CMP_LESS arithmetic_expr
			  | arithmetic_expr CMP_GREATER arithmetic_expr
			  | arithmetic_expr CMP_EQ arithmetic_expr
			  | arithmetic_expr CMP_LEQ arithmetic_expr
			  | arithmetic_expr CMP_GEQ arithmetic_expr
			  | arithmetic_expr CMP_NEQ arithmetic_expr
			  ;
```

```yacc
boolean_expr := boolean_expr OR boolean_expr
              | boolean_expr AND boolean_expr
              | NOT boolean_expr
              | '(' boolean_expr ')'
              ;
```

### 1. 基本的 SELECT 语句

一个基本的 SELECT 语句如下所示：

```sql
SELECT select_list FROM table_expression
```

其中 `select_list` 为一个或多个列名，而 `table_sxpression` 为一个或多个表名