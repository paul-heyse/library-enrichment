# `ruff_python_ast::generated`

Crate `ruff_python_ast` · 89 public items · structured records in [`model/ruff_python_ast.generated.json`](../model/ruff_python_ast.generated.json)

## AnyNodeRef

`enum` · `ruff_python_ast::generated::AnyNodeRef`

Also reachable as `ruff_python_ast::AnyNodeRef`

```rust
enum AnyNodeRef<'a>
```

**Variants**: `ModModule`, `ModExpression`, `StmtFunctionDef`, `StmtClassDef`, `StmtReturn`, `StmtDelete`, `StmtTypeAlias`, `StmtAssign`, `StmtAugAssign`, `StmtAnnAssign`, `StmtFor`, `StmtWhile`, `StmtIf`, `StmtWith`, `StmtMatch`, `StmtRaise`, `StmtTry`, `StmtAssert`, `StmtImport`, `StmtImportFrom`, `StmtGlobal`, `StmtNonlocal`, `StmtExpr`, `StmtPass`, `StmtBreak`, `StmtContinue`, `StmtIpyEscapeCommand`, `ExprBoolOp`, `ExprNamed`, `ExprBinOp`, `ExprUnaryOp`, `ExprLambda`, `ExprIf`, `ExprDict`, `ExprSet`, `ExprListComp`, `ExprSetComp`, `ExprDictComp`, `ExprGenerator`, `ExprAwait`, `ExprYield`, `ExprYieldFrom`, `ExprCompare`, `ExprCall`, `ExprFString`, `ExprTString`, `ExprStringLiteral`, `ExprBytesLiteral`, `ExprNumberLiteral`, `ExprBooleanLiteral`, `ExprNoneLiteral`, `ExprEllipsisLiteral`, `ExprAttribute`, `ExprSubscript`, `ExprStarred`, `ExprName`, `ExprList`, `ExprTuple`, `ExprSlice`, `ExprIpyEscapeCommand`, `ExceptHandlerExceptHandler`, `InterpolatedElement`, `InterpolatedStringLiteralElement`, `PatternMatchValue`, `PatternMatchSingleton`, `PatternMatchSequence`, `PatternMatchMapping`, `PatternMatchClass`, `PatternMatchStar`, `PatternMatchAs`, `PatternMatchOr`, `TypeParamTypeVar`, `TypeParamTypeVarTuple`, `TypeParamParamSpec`, `InterpolatedStringFormatSpec`, `PatternArguments`, `PatternKeyword`, `Comprehension`, `Arguments`, `Parameters`, `Parameter`, `ParameterWithDefault`, `Keyword`, `Alias`, `WithItem`, `MatchCase`, `Decorator`, `ElifElseClause`, `TypeParams`, `FString`, `TString`, `StringLiteral`, `BytesLiteral`, `Identifier`

**Implements**: `core::convert::From`, `pyrefly_util::display::DisplayWith`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (492)

```rust
fn alias(self) -> Option<&'a Alias>
fn arguments(self) -> Option<&'a Arguments>
fn as_alias(&self) -> Option<&&'a Alias>
fn as_arguments(&self) -> Option<&&'a Arguments>
fn as_bytes_literal(&self) -> Option<&&'a BytesLiteral>
fn as_comprehension(&self) -> Option<&&'a Comprehension>
fn as_decorator(&self) -> Option<&&'a Decorator>
fn as_elif_else_clause(&self) -> Option<&&'a ElifElseClause>
fn as_except_handler_except_handler(&self) -> Option<&&'a ExceptHandlerExceptHandler>
fn as_except_handler_ref(self) -> Option<ExceptHandlerRef<'a>>
fn as_expr_attribute(&self) -> Option<&&'a ExprAttribute>
fn as_expr_await(&self) -> Option<&&'a ExprAwait>
fn as_expr_bin_op(&self) -> Option<&&'a ExprBinOp>
fn as_expr_bool_op(&self) -> Option<&&'a ExprBoolOp>
fn as_expr_boolean_literal(&self) -> Option<&&'a ExprBooleanLiteral>
fn as_expr_bytes_literal(&self) -> Option<&&'a ExprBytesLiteral>
fn as_expr_call(&self) -> Option<&&'a ExprCall>
fn as_expr_compare(&self) -> Option<&&'a ExprCompare>
fn as_expr_dict(&self) -> Option<&&'a ExprDict>
fn as_expr_dict_comp(&self) -> Option<&&'a ExprDictComp>
fn as_expr_ellipsis_literal(&self) -> Option<&&'a ExprEllipsisLiteral>
fn as_expr_f_string(&self) -> Option<&&'a ExprFString>
fn as_expr_generator(&self) -> Option<&&'a ExprGenerator>
fn as_expr_if(&self) -> Option<&&'a ExprIf>
fn as_expr_ipy_escape_command(&self) -> Option<&&'a ExprIpyEscapeCommand>
fn as_expr_lambda(&self) -> Option<&&'a ExprLambda>
fn as_expr_list(&self) -> Option<&&'a ExprList>
fn as_expr_list_comp(&self) -> Option<&&'a ExprListComp>
fn as_expr_name(&self) -> Option<&&'a ExprName>
fn as_expr_named(&self) -> Option<&&'a ExprNamed>
fn as_expr_none_literal(&self) -> Option<&&'a ExprNoneLiteral>
fn as_expr_number_literal(&self) -> Option<&&'a ExprNumberLiteral>
fn as_expr_ref(self) -> Option<ExprRef<'a>>
fn as_expr_set(&self) -> Option<&&'a ExprSet>
fn as_expr_set_comp(&self) -> Option<&&'a ExprSetComp>
fn as_expr_slice(&self) -> Option<&&'a ExprSlice>
fn as_expr_starred(&self) -> Option<&&'a ExprStarred>
fn as_expr_string_literal(&self) -> Option<&&'a ExprStringLiteral>
fn as_expr_subscript(&self) -> Option<&&'a ExprSubscript>
fn as_expr_t_string(&self) -> Option<&&'a ExprTString>
fn as_expr_tuple(&self) -> Option<&&'a ExprTuple>
fn as_expr_unary_op(&self) -> Option<&&'a ExprUnaryOp>
fn as_expr_yield(&self) -> Option<&&'a ExprYield>
fn as_expr_yield_from(&self) -> Option<&&'a ExprYieldFrom>
fn as_f_string(&self) -> Option<&&'a FString>
fn as_identifier(&self) -> Option<&&'a Identifier>
fn as_interpolated_element(&self) -> Option<&&'a InterpolatedElement>
fn as_interpolated_string_element_ref(self) -> Option<InterpolatedStringElementRef<'a>>
fn as_interpolated_string_format_spec(&self) -> Option<&&'a InterpolatedStringFormatSpec>
fn as_interpolated_string_literal_element(&self) -> Option<&&'a InterpolatedStringLiteralElement>
fn as_keyword(&self) -> Option<&&'a Keyword>
fn as_match_case(&self) -> Option<&&'a MatchCase>
fn as_mod_expression(&self) -> Option<&&'a ModExpression>
fn as_mod_module(&self) -> Option<&&'a ModModule>
fn as_mod_ref(self) -> Option<ModRef<'a>>
fn as_mut_alias(&mut self) -> Option<&mut &'a Alias>
fn as_mut_arguments(&mut self) -> Option<&mut &'a Arguments>
fn as_mut_bytes_literal(&mut self) -> Option<&mut &'a BytesLiteral>
fn as_mut_comprehension(&mut self) -> Option<&mut &'a Comprehension>
fn as_mut_decorator(&mut self) -> Option<&mut &'a Decorator>
fn as_mut_elif_else_clause(&mut self) -> Option<&mut &'a ElifElseClause>
fn as_mut_except_handler_except_handler(&mut self) -> Option<&mut &'a ExceptHandlerExceptHandler>
fn as_mut_expr_attribute(&mut self) -> Option<&mut &'a ExprAttribute>
fn as_mut_expr_await(&mut self) -> Option<&mut &'a ExprAwait>
fn as_mut_expr_bin_op(&mut self) -> Option<&mut &'a ExprBinOp>
fn as_mut_expr_bool_op(&mut self) -> Option<&mut &'a ExprBoolOp>
fn as_mut_expr_boolean_literal(&mut self) -> Option<&mut &'a ExprBooleanLiteral>
fn as_mut_expr_bytes_literal(&mut self) -> Option<&mut &'a ExprBytesLiteral>
fn as_mut_expr_call(&mut self) -> Option<&mut &'a ExprCall>
fn as_mut_expr_compare(&mut self) -> Option<&mut &'a ExprCompare>
fn as_mut_expr_dict(&mut self) -> Option<&mut &'a ExprDict>
fn as_mut_expr_dict_comp(&mut self) -> Option<&mut &'a ExprDictComp>
fn as_mut_expr_ellipsis_literal(&mut self) -> Option<&mut &'a ExprEllipsisLiteral>
fn as_mut_expr_f_string(&mut self) -> Option<&mut &'a ExprFString>
fn as_mut_expr_generator(&mut self) -> Option<&mut &'a ExprGenerator>
fn as_mut_expr_if(&mut self) -> Option<&mut &'a ExprIf>
fn as_mut_expr_ipy_escape_command(&mut self) -> Option<&mut &'a ExprIpyEscapeCommand>
fn as_mut_expr_lambda(&mut self) -> Option<&mut &'a ExprLambda>
fn as_mut_expr_list(&mut self) -> Option<&mut &'a ExprList>
fn as_mut_expr_list_comp(&mut self) -> Option<&mut &'a ExprListComp>
fn as_mut_expr_name(&mut self) -> Option<&mut &'a ExprName>
fn as_mut_expr_named(&mut self) -> Option<&mut &'a ExprNamed>
fn as_mut_expr_none_literal(&mut self) -> Option<&mut &'a ExprNoneLiteral>
fn as_mut_expr_number_literal(&mut self) -> Option<&mut &'a ExprNumberLiteral>
fn as_mut_expr_set(&mut self) -> Option<&mut &'a ExprSet>
fn as_mut_expr_set_comp(&mut self) -> Option<&mut &'a ExprSetComp>
fn as_mut_expr_slice(&mut self) -> Option<&mut &'a ExprSlice>
fn as_mut_expr_starred(&mut self) -> Option<&mut &'a ExprStarred>
fn as_mut_expr_string_literal(&mut self) -> Option<&mut &'a ExprStringLiteral>
fn as_mut_expr_subscript(&mut self) -> Option<&mut &'a ExprSubscript>
fn as_mut_expr_t_string(&mut self) -> Option<&mut &'a ExprTString>
fn as_mut_expr_tuple(&mut self) -> Option<&mut &'a ExprTuple>
fn as_mut_expr_unary_op(&mut self) -> Option<&mut &'a ExprUnaryOp>
fn as_mut_expr_yield(&mut self) -> Option<&mut &'a ExprYield>
fn as_mut_expr_yield_from(&mut self) -> Option<&mut &'a ExprYieldFrom>
fn as_mut_f_string(&mut self) -> Option<&mut &'a FString>
fn as_mut_identifier(&mut self) -> Option<&mut &'a Identifier>
fn as_mut_interpolated_element(&mut self) -> Option<&mut &'a InterpolatedElement>
fn as_mut_interpolated_string_format_spec(&mut self) -> Option<&mut &'a InterpolatedStringFormatSpec>
fn as_mut_interpolated_string_literal_element(&mut self) -> Option<&mut &'a InterpolatedStringLiteralElement>
fn as_mut_keyword(&mut self) -> Option<&mut &'a Keyword>
fn as_mut_match_case(&mut self) -> Option<&mut &'a MatchCase>
fn as_mut_mod_expression(&mut self) -> Option<&mut &'a ModExpression>
fn as_mut_mod_module(&mut self) -> Option<&mut &'a ModModule>
fn as_mut_parameter(&mut self) -> Option<&mut &'a Parameter>
fn as_mut_parameter_with_default(&mut self) -> Option<&mut &'a ParameterWithDefault>
fn as_mut_parameters(&mut self) -> Option<&mut &'a Parameters>
fn as_mut_pattern_arguments(&mut self) -> Option<&mut &'a PatternArguments>
fn as_mut_pattern_keyword(&mut self) -> Option<&mut &'a PatternKeyword>
fn as_mut_pattern_match_as(&mut self) -> Option<&mut &'a PatternMatchAs>
fn as_mut_pattern_match_class(&mut self) -> Option<&mut &'a PatternMatchClass>
fn as_mut_pattern_match_mapping(&mut self) -> Option<&mut &'a PatternMatchMapping>
fn as_mut_pattern_match_or(&mut self) -> Option<&mut &'a PatternMatchOr>
fn as_mut_pattern_match_sequence(&mut self) -> Option<&mut &'a PatternMatchSequence>
fn as_mut_pattern_match_singleton(&mut self) -> Option<&mut &'a PatternMatchSingleton>
fn as_mut_pattern_match_star(&mut self) -> Option<&mut &'a PatternMatchStar>
fn as_mut_pattern_match_value(&mut self) -> Option<&mut &'a PatternMatchValue>
fn as_mut_stmt_ann_assign(&mut self) -> Option<&mut &'a StmtAnnAssign>
fn as_mut_stmt_assert(&mut self) -> Option<&mut &'a StmtAssert>
fn as_mut_stmt_assign(&mut self) -> Option<&mut &'a StmtAssign>
fn as_mut_stmt_aug_assign(&mut self) -> Option<&mut &'a StmtAugAssign>
fn as_mut_stmt_break(&mut self) -> Option<&mut &'a StmtBreak>
fn as_mut_stmt_class_def(&mut self) -> Option<&mut &'a StmtClassDef>
fn as_mut_stmt_continue(&mut self) -> Option<&mut &'a StmtContinue>
fn as_mut_stmt_delete(&mut self) -> Option<&mut &'a StmtDelete>
fn as_mut_stmt_expr(&mut self) -> Option<&mut &'a StmtExpr>
fn as_mut_stmt_for(&mut self) -> Option<&mut &'a StmtFor>
fn as_mut_stmt_function_def(&mut self) -> Option<&mut &'a StmtFunctionDef>
fn as_mut_stmt_global(&mut self) -> Option<&mut &'a StmtGlobal>
fn as_mut_stmt_if(&mut self) -> Option<&mut &'a StmtIf>
fn as_mut_stmt_import(&mut self) -> Option<&mut &'a StmtImport>
fn as_mut_stmt_import_from(&mut self) -> Option<&mut &'a StmtImportFrom>
fn as_mut_stmt_ipy_escape_command(&mut self) -> Option<&mut &'a StmtIpyEscapeCommand>
fn as_mut_stmt_match(&mut self) -> Option<&mut &'a StmtMatch>
fn as_mut_stmt_nonlocal(&mut self) -> Option<&mut &'a StmtNonlocal>
fn as_mut_stmt_pass(&mut self) -> Option<&mut &'a StmtPass>
fn as_mut_stmt_raise(&mut self) -> Option<&mut &'a StmtRaise>
fn as_mut_stmt_return(&mut self) -> Option<&mut &'a StmtReturn>
fn as_mut_stmt_try(&mut self) -> Option<&mut &'a StmtTry>
fn as_mut_stmt_type_alias(&mut self) -> Option<&mut &'a StmtTypeAlias>
fn as_mut_stmt_while(&mut self) -> Option<&mut &'a StmtWhile>
fn as_mut_stmt_with(&mut self) -> Option<&mut &'a StmtWith>
fn as_mut_string_literal(&mut self) -> Option<&mut &'a StringLiteral>
fn as_mut_t_string(&mut self) -> Option<&mut &'a TString>
fn as_mut_type_param_param_spec(&mut self) -> Option<&mut &'a TypeParamParamSpec>
fn as_mut_type_param_type_var(&mut self) -> Option<&mut &'a TypeParamTypeVar>
fn as_mut_type_param_type_var_tuple(&mut self) -> Option<&mut &'a TypeParamTypeVarTuple>
fn as_mut_type_params(&mut self) -> Option<&mut &'a TypeParams>
fn as_mut_with_item(&mut self) -> Option<&mut &'a WithItem>
fn as_parameter(&self) -> Option<&&'a Parameter>
fn as_parameter_with_default(&self) -> Option<&&'a ParameterWithDefault>
fn as_parameters(&self) -> Option<&&'a Parameters>
fn as_pattern_arguments(&self) -> Option<&&'a PatternArguments>
fn as_pattern_keyword(&self) -> Option<&&'a PatternKeyword>
fn as_pattern_match_as(&self) -> Option<&&'a PatternMatchAs>
fn as_pattern_match_class(&self) -> Option<&&'a PatternMatchClass>
fn as_pattern_match_mapping(&self) -> Option<&&'a PatternMatchMapping>
fn as_pattern_match_or(&self) -> Option<&&'a PatternMatchOr>
fn as_pattern_match_sequence(&self) -> Option<&&'a PatternMatchSequence>
fn as_pattern_match_singleton(&self) -> Option<&&'a PatternMatchSingleton>
fn as_pattern_match_star(&self) -> Option<&&'a PatternMatchStar>
fn as_pattern_match_value(&self) -> Option<&&'a PatternMatchValue>
fn as_pattern_ref(self) -> Option<PatternRef<'a>>
fn as_ptr(&self) -> std::ptr::NonNull<()>
fn as_stmt_ann_assign(&self) -> Option<&&'a StmtAnnAssign>
fn as_stmt_assert(&self) -> Option<&&'a StmtAssert>
fn as_stmt_assign(&self) -> Option<&&'a StmtAssign>
fn as_stmt_aug_assign(&self) -> Option<&&'a StmtAugAssign>
fn as_stmt_break(&self) -> Option<&&'a StmtBreak>
fn as_stmt_class_def(&self) -> Option<&&'a StmtClassDef>
fn as_stmt_continue(&self) -> Option<&&'a StmtContinue>
fn as_stmt_delete(&self) -> Option<&&'a StmtDelete>
fn as_stmt_expr(&self) -> Option<&&'a StmtExpr>
fn as_stmt_for(&self) -> Option<&&'a StmtFor>
fn as_stmt_function_def(&self) -> Option<&&'a StmtFunctionDef>
fn as_stmt_global(&self) -> Option<&&'a StmtGlobal>
fn as_stmt_if(&self) -> Option<&&'a StmtIf>
fn as_stmt_import(&self) -> Option<&&'a StmtImport>
fn as_stmt_import_from(&self) -> Option<&&'a StmtImportFrom>
fn as_stmt_ipy_escape_command(&self) -> Option<&&'a StmtIpyEscapeCommand>
fn as_stmt_match(&self) -> Option<&&'a StmtMatch>
fn as_stmt_nonlocal(&self) -> Option<&&'a StmtNonlocal>
fn as_stmt_pass(&self) -> Option<&&'a StmtPass>
fn as_stmt_raise(&self) -> Option<&&'a StmtRaise>
fn as_stmt_ref(self) -> Option<StmtRef<'a>>
fn as_stmt_return(&self) -> Option<&&'a StmtReturn>
fn as_stmt_try(&self) -> Option<&&'a StmtTry>
fn as_stmt_type_alias(&self) -> Option<&&'a StmtTypeAlias>
fn as_stmt_while(&self) -> Option<&&'a StmtWhile>
fn as_stmt_with(&self) -> Option<&&'a StmtWith>
fn as_string_literal(&self) -> Option<&&'a StringLiteral>
fn as_t_string(&self) -> Option<&&'a TString>
fn as_type_param_param_spec(&self) -> Option<&&'a TypeParamParamSpec>
fn as_type_param_ref(self) -> Option<TypeParamRef<'a>>
fn as_type_param_type_var(&self) -> Option<&&'a TypeParamTypeVar>
fn as_type_param_type_var_tuple(&self) -> Option<&&'a TypeParamTypeVarTuple>
fn as_type_params(&self) -> Option<&&'a TypeParams>
fn as_with_item(&self) -> Option<&&'a WithItem>
fn bytes_literal(self) -> Option<&'a BytesLiteral>
fn comprehension(self) -> Option<&'a Comprehension>
fn decorator(self) -> Option<&'a Decorator>
fn elif_else_clause(self) -> Option<&'a ElifElseClause>
fn except_handler_except_handler(self) -> Option<&'a ExceptHandlerExceptHandler>
fn expect_alias(self) -> &'a Alias where Self: ::std::fmt::Debug
fn expect_arguments(self) -> &'a Arguments where Self: ::std::fmt::Debug
fn expect_bytes_literal(self) -> &'a BytesLiteral where Self: ::std::fmt::Debug
fn expect_comprehension(self) -> &'a Comprehension where Self: ::std::fmt::Debug
fn expect_decorator(self) -> &'a Decorator where Self: ::std::fmt::Debug
fn expect_elif_else_clause(self) -> &'a ElifElseClause where Self: ::std::fmt::Debug
fn expect_except_handler_except_handler(self) -> &'a ExceptHandlerExceptHandler where Self: ::std::fmt::Debug
fn expect_expr_attribute(self) -> &'a ExprAttribute where Self: ::std::fmt::Debug
fn expect_expr_await(self) -> &'a ExprAwait where Self: ::std::fmt::Debug
fn expect_expr_bin_op(self) -> &'a ExprBinOp where Self: ::std::fmt::Debug
fn expect_expr_bool_op(self) -> &'a ExprBoolOp where Self: ::std::fmt::Debug
fn expect_expr_boolean_literal(self) -> &'a ExprBooleanLiteral where Self: ::std::fmt::Debug
fn expect_expr_bytes_literal(self) -> &'a ExprBytesLiteral where Self: ::std::fmt::Debug
fn expect_expr_call(self) -> &'a ExprCall where Self: ::std::fmt::Debug
fn expect_expr_compare(self) -> &'a ExprCompare where Self: ::std::fmt::Debug
fn expect_expr_dict(self) -> &'a ExprDict where Self: ::std::fmt::Debug
fn expect_expr_dict_comp(self) -> &'a ExprDictComp where Self: ::std::fmt::Debug
fn expect_expr_ellipsis_literal(self) -> &'a ExprEllipsisLiteral where Self: ::std::fmt::Debug
fn expect_expr_f_string(self) -> &'a ExprFString where Self: ::std::fmt::Debug
fn expect_expr_generator(self) -> &'a ExprGenerator where Self: ::std::fmt::Debug
fn expect_expr_if(self) -> &'a ExprIf where Self: ::std::fmt::Debug
fn expect_expr_ipy_escape_command(self) -> &'a ExprIpyEscapeCommand where Self: ::std::fmt::Debug
fn expect_expr_lambda(self) -> &'a ExprLambda where Self: ::std::fmt::Debug
fn expect_expr_list(self) -> &'a ExprList where Self: ::std::fmt::Debug
fn expect_expr_list_comp(self) -> &'a ExprListComp where Self: ::std::fmt::Debug
fn expect_expr_name(self) -> &'a ExprName where Self: ::std::fmt::Debug
fn expect_expr_named(self) -> &'a ExprNamed where Self: ::std::fmt::Debug
fn expect_expr_none_literal(self) -> &'a ExprNoneLiteral where Self: ::std::fmt::Debug
fn expect_expr_number_literal(self) -> &'a ExprNumberLiteral where Self: ::std::fmt::Debug
fn expect_expr_set(self) -> &'a ExprSet where Self: ::std::fmt::Debug
fn expect_expr_set_comp(self) -> &'a ExprSetComp where Self: ::std::fmt::Debug
fn expect_expr_slice(self) -> &'a ExprSlice where Self: ::std::fmt::Debug
fn expect_expr_starred(self) -> &'a ExprStarred where Self: ::std::fmt::Debug
fn expect_expr_string_literal(self) -> &'a ExprStringLiteral where Self: ::std::fmt::Debug
fn expect_expr_subscript(self) -> &'a ExprSubscript where Self: ::std::fmt::Debug
fn expect_expr_t_string(self) -> &'a ExprTString where Self: ::std::fmt::Debug
fn expect_expr_tuple(self) -> &'a ExprTuple where Self: ::std::fmt::Debug
fn expect_expr_unary_op(self) -> &'a ExprUnaryOp where Self: ::std::fmt::Debug
fn expect_expr_yield(self) -> &'a ExprYield where Self: ::std::fmt::Debug
fn expect_expr_yield_from(self) -> &'a ExprYieldFrom where Self: ::std::fmt::Debug
fn expect_f_string(self) -> &'a FString where Self: ::std::fmt::Debug
fn expect_identifier(self) -> &'a Identifier where Self: ::std::fmt::Debug
fn expect_interpolated_element(self) -> &'a InterpolatedElement where Self: ::std::fmt::Debug
fn expect_interpolated_string_format_spec(self) -> &'a InterpolatedStringFormatSpec where Self: ::std::fmt::Debug
fn expect_interpolated_string_literal_element(self) -> &'a InterpolatedStringLiteralElement where Self: ::std::fmt::Debug
fn expect_keyword(self) -> &'a Keyword where Self: ::std::fmt::Debug
fn expect_match_case(self) -> &'a MatchCase where Self: ::std::fmt::Debug
fn expect_mod_expression(self) -> &'a ModExpression where Self: ::std::fmt::Debug
fn expect_mod_module(self) -> &'a ModModule where Self: ::std::fmt::Debug
fn expect_parameter(self) -> &'a Parameter where Self: ::std::fmt::Debug
fn expect_parameter_with_default(self) -> &'a ParameterWithDefault where Self: ::std::fmt::Debug
fn expect_parameters(self) -> &'a Parameters where Self: ::std::fmt::Debug
fn expect_pattern_arguments(self) -> &'a PatternArguments where Self: ::std::fmt::Debug
fn expect_pattern_keyword(self) -> &'a PatternKeyword where Self: ::std::fmt::Debug
fn expect_pattern_match_as(self) -> &'a PatternMatchAs where Self: ::std::fmt::Debug
fn expect_pattern_match_class(self) -> &'a PatternMatchClass where Self: ::std::fmt::Debug
fn expect_pattern_match_mapping(self) -> &'a PatternMatchMapping where Self: ::std::fmt::Debug
fn expect_pattern_match_or(self) -> &'a PatternMatchOr where Self: ::std::fmt::Debug
fn expect_pattern_match_sequence(self) -> &'a PatternMatchSequence where Self: ::std::fmt::Debug
fn expect_pattern_match_singleton(self) -> &'a PatternMatchSingleton where Self: ::std::fmt::Debug
fn expect_pattern_match_star(self) -> &'a PatternMatchStar where Self: ::std::fmt::Debug
fn expect_pattern_match_value(self) -> &'a PatternMatchValue where Self: ::std::fmt::Debug
fn expect_stmt_ann_assign(self) -> &'a StmtAnnAssign where Self: ::std::fmt::Debug
fn expect_stmt_assert(self) -> &'a StmtAssert where Self: ::std::fmt::Debug
fn expect_stmt_assign(self) -> &'a StmtAssign where Self: ::std::fmt::Debug
fn expect_stmt_aug_assign(self) -> &'a StmtAugAssign where Self: ::std::fmt::Debug
fn expect_stmt_break(self) -> &'a StmtBreak where Self: ::std::fmt::Debug
fn expect_stmt_class_def(self) -> &'a StmtClassDef where Self: ::std::fmt::Debug
fn expect_stmt_continue(self) -> &'a StmtContinue where Self: ::std::fmt::Debug
fn expect_stmt_delete(self) -> &'a StmtDelete where Self: ::std::fmt::Debug
fn expect_stmt_expr(self) -> &'a StmtExpr where Self: ::std::fmt::Debug
fn expect_stmt_for(self) -> &'a StmtFor where Self: ::std::fmt::Debug
fn expect_stmt_function_def(self) -> &'a StmtFunctionDef where Self: ::std::fmt::Debug
fn expect_stmt_global(self) -> &'a StmtGlobal where Self: ::std::fmt::Debug
fn expect_stmt_if(self) -> &'a StmtIf where Self: ::std::fmt::Debug
fn expect_stmt_import(self) -> &'a StmtImport where Self: ::std::fmt::Debug
fn expect_stmt_import_from(self) -> &'a StmtImportFrom where Self: ::std::fmt::Debug
fn expect_stmt_ipy_escape_command(self) -> &'a StmtIpyEscapeCommand where Self: ::std::fmt::Debug
fn expect_stmt_match(self) -> &'a StmtMatch where Self: ::std::fmt::Debug
fn expect_stmt_nonlocal(self) -> &'a StmtNonlocal where Self: ::std::fmt::Debug
fn expect_stmt_pass(self) -> &'a StmtPass where Self: ::std::fmt::Debug
fn expect_stmt_raise(self) -> &'a StmtRaise where Self: ::std::fmt::Debug
fn expect_stmt_return(self) -> &'a StmtReturn where Self: ::std::fmt::Debug
fn expect_stmt_try(self) -> &'a StmtTry where Self: ::std::fmt::Debug
fn expect_stmt_type_alias(self) -> &'a StmtTypeAlias where Self: ::std::fmt::Debug
fn expect_stmt_while(self) -> &'a StmtWhile where Self: ::std::fmt::Debug
fn expect_stmt_with(self) -> &'a StmtWith where Self: ::std::fmt::Debug
fn expect_string_literal(self) -> &'a StringLiteral where Self: ::std::fmt::Debug
fn expect_t_string(self) -> &'a TString where Self: ::std::fmt::Debug
fn expect_type_param_param_spec(self) -> &'a TypeParamParamSpec where Self: ::std::fmt::Debug
fn expect_type_param_type_var(self) -> &'a TypeParamTypeVar where Self: ::std::fmt::Debug
fn expect_type_param_type_var_tuple(self) -> &'a TypeParamTypeVarTuple where Self: ::std::fmt::Debug
fn expect_type_params(self) -> &'a TypeParams where Self: ::std::fmt::Debug
fn expect_with_item(self) -> &'a WithItem where Self: ::std::fmt::Debug
fn expr_attribute(self) -> Option<&'a ExprAttribute>
fn expr_await(self) -> Option<&'a ExprAwait>
fn expr_bin_op(self) -> Option<&'a ExprBinOp>
fn expr_bool_op(self) -> Option<&'a ExprBoolOp>
fn expr_boolean_literal(self) -> Option<&'a ExprBooleanLiteral>
fn expr_bytes_literal(self) -> Option<&'a ExprBytesLiteral>
fn expr_call(self) -> Option<&'a ExprCall>
fn expr_compare(self) -> Option<&'a ExprCompare>
fn expr_dict(self) -> Option<&'a ExprDict>
fn expr_dict_comp(self) -> Option<&'a ExprDictComp>
fn expr_ellipsis_literal(self) -> Option<&'a ExprEllipsisLiteral>
fn expr_f_string(self) -> Option<&'a ExprFString>
fn expr_generator(self) -> Option<&'a ExprGenerator>
fn expr_if(self) -> Option<&'a ExprIf>
fn expr_ipy_escape_command(self) -> Option<&'a ExprIpyEscapeCommand>
fn expr_lambda(self) -> Option<&'a ExprLambda>
fn expr_list(self) -> Option<&'a ExprList>
fn expr_list_comp(self) -> Option<&'a ExprListComp>
fn expr_name(self) -> Option<&'a ExprName>
fn expr_named(self) -> Option<&'a ExprNamed>
fn expr_none_literal(self) -> Option<&'a ExprNoneLiteral>
fn expr_number_literal(self) -> Option<&'a ExprNumberLiteral>
fn expr_set(self) -> Option<&'a ExprSet>
fn expr_set_comp(self) -> Option<&'a ExprSetComp>
fn expr_slice(self) -> Option<&'a ExprSlice>
fn expr_starred(self) -> Option<&'a ExprStarred>
fn expr_string_literal(self) -> Option<&'a ExprStringLiteral>
fn expr_subscript(self) -> Option<&'a ExprSubscript>
fn expr_t_string(self) -> Option<&'a ExprTString>
fn expr_tuple(self) -> Option<&'a ExprTuple>
fn expr_unary_op(self) -> Option<&'a ExprUnaryOp>
fn expr_yield(self) -> Option<&'a ExprYield>
fn expr_yield_from(self) -> Option<&'a ExprYieldFrom>
fn f_string(self) -> Option<&'a FString>
fn identifier(self) -> Option<&'a Identifier>
fn interpolated_element(self) -> Option<&'a InterpolatedElement>
fn interpolated_string_format_spec(self) -> Option<&'a InterpolatedStringFormatSpec>
fn interpolated_string_literal_element(self) -> Option<&'a InterpolatedStringLiteralElement>
const fn is_alias(&self) -> bool
const fn is_alternative_branch_with_node(self) -> bool
const fn is_arguments(&self) -> bool
const fn is_bytes_literal(&self) -> bool
const fn is_comprehension(&self) -> bool
const fn is_decorator(&self) -> bool
const fn is_elif_else_clause(&self) -> bool
const fn is_except_handler(self) -> bool
const fn is_except_handler_except_handler(&self) -> bool
const fn is_expr_attribute(&self) -> bool
const fn is_expr_await(&self) -> bool
const fn is_expr_bin_op(&self) -> bool
const fn is_expr_bool_op(&self) -> bool
const fn is_expr_boolean_literal(&self) -> bool
const fn is_expr_bytes_literal(&self) -> bool
const fn is_expr_call(&self) -> bool
const fn is_expr_compare(&self) -> bool
const fn is_expr_dict(&self) -> bool
const fn is_expr_dict_comp(&self) -> bool
const fn is_expr_ellipsis_literal(&self) -> bool
const fn is_expr_f_string(&self) -> bool
const fn is_expr_generator(&self) -> bool
const fn is_expr_if(&self) -> bool
const fn is_expr_ipy_escape_command(&self) -> bool
const fn is_expr_lambda(&self) -> bool
const fn is_expr_list(&self) -> bool
const fn is_expr_list_comp(&self) -> bool
const fn is_expr_name(&self) -> bool
const fn is_expr_named(&self) -> bool
const fn is_expr_none_literal(&self) -> bool
const fn is_expr_number_literal(&self) -> bool
const fn is_expr_set(&self) -> bool
const fn is_expr_set_comp(&self) -> bool
const fn is_expr_slice(&self) -> bool
const fn is_expr_starred(&self) -> bool
const fn is_expr_string_literal(&self) -> bool
const fn is_expr_subscript(&self) -> bool
const fn is_expr_t_string(&self) -> bool
const fn is_expr_tuple(&self) -> bool
const fn is_expr_unary_op(&self) -> bool
const fn is_expr_yield(&self) -> bool
const fn is_expr_yield_from(&self) -> bool
const fn is_expression(self) -> bool
const fn is_f_string(&self) -> bool
fn is_first_statement_in_alternate_body(&self, body: AnyNodeRef<'_>) -> bool
fn is_first_statement_in_body(&self, body: AnyNodeRef<'_>) -> bool
const fn is_identifier(&self) -> bool
const fn is_interpolated_element(&self) -> bool
const fn is_interpolated_string_element(self) -> bool
const fn is_interpolated_string_format_spec(&self) -> bool
const fn is_interpolated_string_literal_element(&self) -> bool
const fn is_keyword(&self) -> bool
const fn is_match_case(&self) -> bool
const fn is_mod_expression(&self) -> bool
const fn is_mod_module(&self) -> bool
const fn is_module(self) -> bool
const fn is_parameter(&self) -> bool
const fn is_parameter_with_default(&self) -> bool
const fn is_parameters(&self) -> bool
const fn is_pattern(self) -> bool
const fn is_pattern_arguments(&self) -> bool
const fn is_pattern_keyword(&self) -> bool
const fn is_pattern_match_as(&self) -> bool
const fn is_pattern_match_class(&self) -> bool
const fn is_pattern_match_mapping(&self) -> bool
const fn is_pattern_match_or(&self) -> bool
const fn is_pattern_match_sequence(&self) -> bool
const fn is_pattern_match_singleton(&self) -> bool
const fn is_pattern_match_star(&self) -> bool
const fn is_pattern_match_value(&self) -> bool
const fn is_statement(self) -> bool
const fn is_stmt_ann_assign(&self) -> bool
const fn is_stmt_assert(&self) -> bool
const fn is_stmt_assign(&self) -> bool
const fn is_stmt_aug_assign(&self) -> bool
const fn is_stmt_break(&self) -> bool
const fn is_stmt_class_def(&self) -> bool
const fn is_stmt_continue(&self) -> bool
const fn is_stmt_delete(&self) -> bool
const fn is_stmt_expr(&self) -> bool
const fn is_stmt_for(&self) -> bool
const fn is_stmt_function_def(&self) -> bool
const fn is_stmt_global(&self) -> bool
const fn is_stmt_if(&self) -> bool
const fn is_stmt_import(&self) -> bool
const fn is_stmt_import_from(&self) -> bool
const fn is_stmt_ipy_escape_command(&self) -> bool
const fn is_stmt_match(&self) -> bool
const fn is_stmt_nonlocal(&self) -> bool
const fn is_stmt_pass(&self) -> bool
const fn is_stmt_raise(&self) -> bool
const fn is_stmt_return(&self) -> bool
const fn is_stmt_try(&self) -> bool
const fn is_stmt_type_alias(&self) -> bool
const fn is_stmt_while(&self) -> bool
const fn is_stmt_with(&self) -> bool
const fn is_string_literal(&self) -> bool
const fn is_t_string(&self) -> bool
const fn is_type_param(self) -> bool
const fn is_type_param_param_spec(&self) -> bool
const fn is_type_param_type_var(&self) -> bool
const fn is_type_param_type_var_tuple(&self) -> bool
const fn is_type_params(&self) -> bool
const fn is_with_item(&self) -> bool
fn keyword(self) -> Option<&'a Keyword>
const fn kind(self) -> NodeKind
fn last_child_in_body(&self) -> Option<AnyNodeRef<'a>>
fn match_case(self) -> Option<&'a MatchCase>
fn mod_expression(self) -> Option<&'a ModExpression>
fn mod_module(self) -> Option<&'a ModModule>
fn parameter(self) -> Option<&'a Parameter>
fn parameter_with_default(self) -> Option<&'a ParameterWithDefault>
fn parameters(self) -> Option<&'a Parameters>
fn pattern_arguments(self) -> Option<&'a PatternArguments>
fn pattern_keyword(self) -> Option<&'a PatternKeyword>
fn pattern_match_as(self) -> Option<&'a PatternMatchAs>
fn pattern_match_class(self) -> Option<&'a PatternMatchClass>
fn pattern_match_mapping(self) -> Option<&'a PatternMatchMapping>
fn pattern_match_or(self) -> Option<&'a PatternMatchOr>
fn pattern_match_sequence(self) -> Option<&'a PatternMatchSequence>
fn pattern_match_singleton(self) -> Option<&'a PatternMatchSingleton>
fn pattern_match_star(self) -> Option<&'a PatternMatchStar>
fn pattern_match_value(self) -> Option<&'a PatternMatchValue>
fn ptr_eq(self, other: AnyNodeRef<'_>) -> bool
fn stmt_ann_assign(self) -> Option<&'a StmtAnnAssign>
fn stmt_assert(self) -> Option<&'a StmtAssert>
fn stmt_assign(self) -> Option<&'a StmtAssign>
fn stmt_aug_assign(self) -> Option<&'a StmtAugAssign>
fn stmt_break(self) -> Option<&'a StmtBreak>
fn stmt_class_def(self) -> Option<&'a StmtClassDef>
fn stmt_continue(self) -> Option<&'a StmtContinue>
fn stmt_delete(self) -> Option<&'a StmtDelete>
fn stmt_expr(self) -> Option<&'a StmtExpr>
fn stmt_for(self) -> Option<&'a StmtFor>
fn stmt_function_def(self) -> Option<&'a StmtFunctionDef>
fn stmt_global(self) -> Option<&'a StmtGlobal>
fn stmt_if(self) -> Option<&'a StmtIf>
fn stmt_import(self) -> Option<&'a StmtImport>
fn stmt_import_from(self) -> Option<&'a StmtImportFrom>
fn stmt_ipy_escape_command(self) -> Option<&'a StmtIpyEscapeCommand>
fn stmt_match(self) -> Option<&'a StmtMatch>
fn stmt_nonlocal(self) -> Option<&'a StmtNonlocal>
fn stmt_pass(self) -> Option<&'a StmtPass>
fn stmt_raise(self) -> Option<&'a StmtRaise>
fn stmt_return(self) -> Option<&'a StmtReturn>
fn stmt_try(self) -> Option<&'a StmtTry>
fn stmt_type_alias(self) -> Option<&'a StmtTypeAlias>
fn stmt_while(self) -> Option<&'a StmtWhile>
fn stmt_with(self) -> Option<&'a StmtWith>
fn string_literal(self) -> Option<&'a StringLiteral>
fn t_string(self) -> Option<&'a TString>
fn type_param_param_spec(self) -> Option<&'a TypeParamParamSpec>
fn type_param_type_var(self) -> Option<&'a TypeParamTypeVar>
fn type_param_type_var_tuple(self) -> Option<&'a TypeParamTypeVarTuple>
fn type_params(self) -> Option<&'a TypeParams>
fn visit_source_order<'b, V>(self, visitor: &mut V) where V: visitor::source_order::SourceOrderVisitor<'b> + ?Sized, 'a: 'b
fn with_item(self) -> Option<&'a WithItem>
```

**via `core::convert::From`**

```rust
fn from(node: &'a TString) -> AnyNodeRef<'a>
fn from(node: &'a ExprSetComp) -> AnyNodeRef<'a>
fn from(node: &'a ExprName) -> AnyNodeRef<'a>
fn from(node: &'a ModExpression) -> AnyNodeRef<'a>
fn from(node: &'a InterpolatedStringFormatSpec) -> AnyNodeRef<'a>
fn from(node: &'a StmtGlobal) -> AnyNodeRef<'a>
fn from(node: &'a Identifier) -> AnyNodeRef<'a>
fn from(node: &'a Pattern) -> AnyNodeRef<'a>
fn from(node: &'a TypeParam) -> AnyNodeRef<'a>
fn from(node: &'a ExprAwait) -> AnyNodeRef<'a>
fn from(node: &'a ExprSlice) -> AnyNodeRef<'a>
fn from(node: &'a StmtReturn) -> AnyNodeRef<'a>
fn from(value: LiteralExpressionRef<'a>) -> Self
fn from(node: &'a Comprehension) -> AnyNodeRef<'a>
fn from(node: &'a StmtPass) -> AnyNodeRef<'a>
fn from(node: PatternRef<'a>) -> AnyNodeRef<'a>
fn from(node: &'a ExprCompare) -> AnyNodeRef<'a>
fn from(node: &'a InterpolatedElement) -> AnyNodeRef<'a>
fn from(node: &'a StmtAssign) -> AnyNodeRef<'a>
fn from(node: &'a Parameter) -> AnyNodeRef<'a>
fn from(node: &'a StmtIpyEscapeCommand) -> AnyNodeRef<'a>
fn from(node: &'a ExprTString) -> AnyNodeRef<'a>
fn from(node: &'a PatternMatchSingleton) -> AnyNodeRef<'a>
fn from(node: &'a StmtFor) -> AnyNodeRef<'a>
fn from(node: &'a Alias) -> AnyNodeRef<'a>
fn from(node: &'a ExprBinOp) -> AnyNodeRef<'a>
fn from(node: &'a ExprNumberLiteral) -> AnyNodeRef<'a>
fn from(node: &'a PatternMatchClass) -> AnyNodeRef<'a>
fn from(node: &'a StmtWith) -> AnyNodeRef<'a>
fn from(node: &'a Decorator) -> AnyNodeRef<'a>
fn from(node: &'a ExprIf) -> AnyNodeRef<'a>
fn from(node: &'a ExprEllipsisLiteral) -> AnyNodeRef<'a>
fn from(value: &StringLike<'a>) -> Self
fn from(node: &'a PatternMatchOr) -> AnyNodeRef<'a>
fn from(node: &'a StmtTry) -> AnyNodeRef<'a>
fn from(node: &'a FString) -> AnyNodeRef<'a>
fn from(node: TypeParamRef<'a>) -> AnyNodeRef<'a>
fn from(node: &'a ExprListComp) -> AnyNodeRef<'a>
fn from(node: &'a ExprStarred) -> AnyNodeRef<'a>
fn from(node: &'a ModModule) -> AnyNodeRef<'a>
fn from(node: &'a TypeParamParamSpec) -> AnyNodeRef<'a>
fn from(node: ExceptHandlerRef<'a>) -> AnyNodeRef<'a>
fn from(node: &'a StmtImportFrom) -> AnyNodeRef<'a>
fn from(node: &'a BytesLiteral) -> AnyNodeRef<'a>
fn from(node: &'a ExprGenerator) -> AnyNodeRef<'a>
fn from(node: &'a ExprTuple) -> AnyNodeRef<'a>
fn from(node: &'a StmtClassDef) -> AnyNodeRef<'a>
fn from(node: &'a PatternKeyword) -> AnyNodeRef<'a>
fn from(node: &'a StmtExpr) -> AnyNodeRef<'a>
fn from(node: &'a Mod) -> AnyNodeRef<'a>
fn from(node: &'a ExprYieldFrom) -> AnyNodeRef<'a>
fn from(node: &'a ExceptHandlerExceptHandler) -> AnyNodeRef<'a>
fn from(node: &'a StmtTypeAlias) -> AnyNodeRef<'a>
fn from(node: &'a Parameters) -> AnyNodeRef<'a>
fn from(node: &'a StmtContinue) -> AnyNodeRef<'a>
fn from(node: &'a Expr) -> AnyNodeRef<'a>
fn from(node: InterpolatedStringElementRef<'a>) -> AnyNodeRef<'a>
fn from(node: &'a ExprFString) -> AnyNodeRef<'a>
fn from(node: &'a PatternMatchValue) -> AnyNodeRef<'a>
fn from(node: &'a StmtAnnAssign) -> AnyNodeRef<'a>
fn from(node: &'a Keyword) -> AnyNodeRef<'a>
fn from(node: &'a ExprNamed) -> AnyNodeRef<'a>
fn from(value: StringLikePart<'a>) -> Self
fn from(node: &'a ExprBytesLiteral) -> AnyNodeRef<'a>
fn from(node: &'a PatternMatchMapping) -> AnyNodeRef<'a>
fn from(node: &'a StmtIf) -> AnyNodeRef<'a>
fn from(node: &'a MatchCase) -> AnyNodeRef<'a>
fn from(node: &'a ExprLambda) -> AnyNodeRef<'a>
fn from(node: &'a InterpolatedStringElement) -> AnyNodeRef<'a>
fn from(node: &'a ExprNoneLiteral) -> AnyNodeRef<'a>
fn from(value: StringLike<'a>) -> Self
fn from(node: &'a PatternMatchAs) -> AnyNodeRef<'a>
fn from(node: &'a StmtRaise) -> AnyNodeRef<'a>
fn from(node: &'a TypeParams) -> AnyNodeRef<'a>
fn from(node: ExprRef<'a>) -> AnyNodeRef<'a>
fn from(node: &'a ExprSet) -> AnyNodeRef<'a>
fn from(node: &'a ExprSubscript) -> AnyNodeRef<'a>
fn from(node: &'a TypeParamTypeVarTuple) -> AnyNodeRef<'a>
fn from(node: &'a StmtImport) -> AnyNodeRef<'a>
fn from(node: &'a StringLiteral) -> AnyNodeRef<'a>
fn from(node: &'a ExprDictComp) -> AnyNodeRef<'a>
fn from(node: &'a ExprList) -> AnyNodeRef<'a>
fn from(node: &'a StmtFunctionDef) -> AnyNodeRef<'a>
fn from(node: &'a ExceptHandler) -> AnyNodeRef<'a>
fn from(node: &'a PatternArguments) -> AnyNodeRef<'a>
fn from(node: &'a StmtNonlocal) -> AnyNodeRef<'a>
fn from(node: &'a ExprYield) -> AnyNodeRef<'a>
fn from(node: &'a Stmt) -> AnyNodeRef<'a>
fn from(node: &'a ExprIpyEscapeCommand) -> AnyNodeRef<'a>
fn from(node: &'a StmtDelete) -> AnyNodeRef<'a>
fn from(node: &'a Arguments) -> AnyNodeRef<'a>
fn from(node: &'a StmtBreak) -> AnyNodeRef<'a>
fn from(node: &'a ExprCall) -> AnyNodeRef<'a>
fn from(node: &'a InterpolatedStringLiteralElement) -> AnyNodeRef<'a>
fn from(node: &'a StmtAugAssign) -> AnyNodeRef<'a>
fn from(node: &'a ParameterWithDefault) -> AnyNodeRef<'a>
fn from(node: &'a ExprBoolOp) -> AnyNodeRef<'a>
fn from(value: &StringLikePart<'a>) -> Self
fn from(node: &'a ExprStringLiteral) -> AnyNodeRef<'a>
fn from(node: &'a PatternMatchSequence) -> AnyNodeRef<'a>
fn from(node: &'a StmtWhile) -> AnyNodeRef<'a>
fn from(node: &'a WithItem) -> AnyNodeRef<'a>
fn from(node: ModRef<'a>) -> AnyNodeRef<'a>
fn from(node: &'a ExprUnaryOp) -> AnyNodeRef<'a>
fn from(node: &'a ExprBooleanLiteral) -> AnyNodeRef<'a>
fn from(node: StmtRef<'a>) -> AnyNodeRef<'a>
fn from(node: &'a PatternMatchStar) -> AnyNodeRef<'a>
fn from(node: &'a StmtMatch) -> AnyNodeRef<'a>
fn from(node: &'a ElifElseClause) -> AnyNodeRef<'a>
fn from(node: &'a ExprDict) -> AnyNodeRef<'a>
fn from(node: &'a ExprAttribute) -> AnyNodeRef<'a>
fn from(node: &'a TypeParamTypeVar) -> AnyNodeRef<'a>
fn from(node: &'a StmtAssert) -> AnyNodeRef<'a>
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

A flattened enumeration of all AST nodes.

---

## AnyRootNodeRef

`enum` · `ruff_python_ast::generated::AnyRootNodeRef`

Also reachable as `ruff_python_ast::AnyRootNodeRef`

```rust
enum AnyRootNodeRef<'a>
```

**Variants**: `Mod`, `Stmt`, `Expr`, `ExceptHandler`, `InterpolatedStringElement`, `Pattern`, `TypeParam`, `InterpolatedStringFormatSpec`, `PatternArguments`, `PatternKeyword`, `Comprehension`, `Arguments`, `Parameters`, `Parameter`, `ParameterWithDefault`, `Keyword`, `Alias`, `WithItem`, `MatchCase`, `Decorator`, `ElifElseClause`, `TypeParams`, `FString`, `TString`, `StringLiteral`, `BytesLiteral`, `Identifier`

**Implements**: `core::convert::From`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
unsafe fn from_raw_parts(kind: RootNodeKind, pointer: std::ptr::NonNull<()>) -> Self
fn into_raw_parts(self) -> (RootNodeKind, std::ptr::NonNull<()>)
fn visit_source_order<'b, V>(self, visitor: &mut V) where V: visitor::source_order::SourceOrderVisitor<'b> + ?Sized, 'a: 'b
```

**via `core::convert::From`**

```rust
fn from(node: &'a StringLiteral) -> AnyRootNodeRef<'a>
fn from(node: &'a WithItem) -> AnyRootNodeRef<'a>
fn from(node: &'a Comprehension) -> AnyRootNodeRef<'a>
fn from(node: &'a Expr) -> AnyRootNodeRef<'a>
fn from(node: &'a ElifElseClause) -> AnyRootNodeRef<'a>
fn from(node: &'a Parameter) -> AnyRootNodeRef<'a>
fn from(node: &'a InterpolatedStringElement) -> AnyRootNodeRef<'a>
fn from(node: &'a TString) -> AnyRootNodeRef<'a>
fn from(node: &'a Alias) -> AnyRootNodeRef<'a>
fn from(node: &'a PatternKeyword) -> AnyRootNodeRef<'a>
fn from(node: &'a ExceptHandler) -> AnyRootNodeRef<'a>
fn from(node: &'a Stmt) -> AnyRootNodeRef<'a>
fn from(node: &'a Identifier) -> AnyRootNodeRef<'a>
fn from(node: &'a Mod) -> AnyRootNodeRef<'a>
fn from(node: &'a Decorator) -> AnyRootNodeRef<'a>
fn from(node: &'a Parameters) -> AnyRootNodeRef<'a>
fn from(node: &'a FString) -> AnyRootNodeRef<'a>
fn from(node: &'a Keyword) -> AnyRootNodeRef<'a>
fn from(node: &'a PatternArguments) -> AnyRootNodeRef<'a>
fn from(node: &'a Pattern) -> AnyRootNodeRef<'a>
fn from(node: &'a BytesLiteral) -> AnyRootNodeRef<'a>
fn from(node: &'a TypeParam) -> AnyRootNodeRef<'a>
fn from(node: &'a MatchCase) -> AnyRootNodeRef<'a>
fn from(node: &'a Arguments) -> AnyRootNodeRef<'a>
fn from(node: &'a TypeParams) -> AnyRootNodeRef<'a>
fn from(node: &'a ParameterWithDefault) -> AnyRootNodeRef<'a>
fn from(node: &'a InterpolatedStringFormatSpec) -> AnyRootNodeRef<'a>
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An enumeration of all AST nodes.

Unlike `AnyNodeRef`, this type does not flatten nested enums, so its variants only
consist of the "root" AST node types. This is useful as it exposes references to the
original enums, not just references to their inner values.

For example, `AnyRootNodeRef::Mod` contains a reference to the `Mod` enum, while
`AnyNodeRef` has top-level `AnyNodeRef::ModModule` and `AnyNodeRef::ModExpression`
variants.

---

## ExceptHandler

`enum` · `ruff_python_ast::generated::ExceptHandler`

Also reachable as `ruff_python_ast::ExceptHandler`

```rust
enum ExceptHandler
```

**Variants**: `ExceptHandler`

**Implements**: `core::convert::From`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn as_except_handler(&self) -> Option<&ExceptHandlerExceptHandler>
fn as_except_handler_mut(&mut self) -> Option<&mut ExceptHandlerExceptHandler>
fn except_handler(self) -> Option<ExceptHandlerExceptHandler>
fn expect_except_handler(self) -> ExceptHandlerExceptHandler
const fn is_except_handler(&self) -> bool
```

**via `core::convert::From`**

```rust
fn from(node: ExceptHandlerExceptHandler) -> Self
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [excepthandler](https://docs.python.org/3/library/ast.html#ast.excepthandler)

---

## ExceptHandlerRef

`enum` · `ruff_python_ast::generated::ExceptHandlerRef`

Also reachable as `ruff_python_ast::ExceptHandlerRef`

```rust
enum ExceptHandlerRef<'a>
```

**Variants**: `ExceptHandler`

**Implements**: `core::convert::From`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (5)

```rust
fn as_except_handler(&self) -> Option<&&'a ExceptHandlerExceptHandler>
fn as_mut_except_handler(&mut self) -> Option<&mut &'a ExceptHandlerExceptHandler>
fn except_handler(self) -> Option<&'a ExceptHandlerExceptHandler>
fn expect_except_handler(self) -> &'a ExceptHandlerExceptHandler where Self: ::std::fmt::Debug
const fn is_except_handler(&self) -> bool
```

**via `core::convert::From`**

```rust
fn from(node: &'a ExceptHandlerExceptHandler) -> Self
fn from(node: &'a ExceptHandler) -> Self
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [excepthandler](https://docs.python.org/3/library/ast.html#ast.excepthandler)

---

## Expr

`enum` · `ruff_python_ast::generated::Expr`

Also reachable as `pyrefly::query::TypeQueryExpr`, `ruff_python_ast::Expr`

```rust
enum Expr
```

**Variants**: `BoolOp`, `Named`, `BinOp`, `UnaryOp`, `Lambda`, `If`, `Dict`, `Set`, `ListComp`, `SetComp`, `DictComp`, `Generator`, `Await`, `Yield`, `YieldFrom`, `Compare`, `Call`, `FString`, `TString`, `StringLiteral`, `BytesLiteral`, `NumberLiteral`, `BooleanLiteral`, `NoneLiteral`, `EllipsisLiteral`, `Attribute`, `Subscript`, `Starred`, `Name`, `List`, `Tuple`, `Slice`, `IpyEscapeCommand`

**Implements**: `core::convert::From`, `pyrefly_util::display::DisplayWith`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (169)

```rust
fn as_attribute_expr(&self) -> Option<&ExprAttribute>
fn as_attribute_expr_mut(&mut self) -> Option<&mut ExprAttribute>
fn as_await_expr(&self) -> Option<&ExprAwait>
fn as_await_expr_mut(&mut self) -> Option<&mut ExprAwait>
fn as_bin_op_expr(&self) -> Option<&ExprBinOp>
fn as_bin_op_expr_mut(&mut self) -> Option<&mut ExprBinOp>
fn as_bool_op_expr(&self) -> Option<&ExprBoolOp>
fn as_bool_op_expr_mut(&mut self) -> Option<&mut ExprBoolOp>
fn as_boolean_literal_expr(&self) -> Option<&ExprBooleanLiteral>
fn as_boolean_literal_expr_mut(&mut self) -> Option<&mut ExprBooleanLiteral>
fn as_bytes_literal_expr(&self) -> Option<&ExprBytesLiteral>
fn as_bytes_literal_expr_mut(&mut self) -> Option<&mut ExprBytesLiteral>
fn as_call_expr(&self) -> Option<&ExprCall>
fn as_call_expr_mut(&mut self) -> Option<&mut ExprCall>
fn as_compare_expr(&self) -> Option<&ExprCompare>
fn as_compare_expr_mut(&mut self) -> Option<&mut ExprCompare>
fn as_dict_comp_expr(&self) -> Option<&ExprDictComp>
fn as_dict_comp_expr_mut(&mut self) -> Option<&mut ExprDictComp>
fn as_dict_expr(&self) -> Option<&ExprDict>
fn as_dict_expr_mut(&mut self) -> Option<&mut ExprDict>
fn as_ellipsis_literal_expr(&self) -> Option<&ExprEllipsisLiteral>
fn as_ellipsis_literal_expr_mut(&mut self) -> Option<&mut ExprEllipsisLiteral>
fn as_f_string_expr(&self) -> Option<&ExprFString>
fn as_f_string_expr_mut(&mut self) -> Option<&mut ExprFString>
fn as_generator_expr(&self) -> Option<&ExprGenerator>
fn as_generator_expr_mut(&mut self) -> Option<&mut ExprGenerator>
fn as_if_expr(&self) -> Option<&ExprIf>
fn as_if_expr_mut(&mut self) -> Option<&mut ExprIf>
fn as_ipy_escape_command_expr(&self) -> Option<&ExprIpyEscapeCommand>
fn as_ipy_escape_command_expr_mut(&mut self) -> Option<&mut ExprIpyEscapeCommand>
fn as_lambda_expr(&self) -> Option<&ExprLambda>
fn as_lambda_expr_mut(&mut self) -> Option<&mut ExprLambda>
fn as_list_comp_expr(&self) -> Option<&ExprListComp>
fn as_list_comp_expr_mut(&mut self) -> Option<&mut ExprListComp>
fn as_list_expr(&self) -> Option<&ExprList>
fn as_list_expr_mut(&mut self) -> Option<&mut ExprList>
fn as_literal_expr(&self) -> Option<LiteralExpressionRef<'_>>
fn as_name_expr(&self) -> Option<&ExprName>
fn as_name_expr_mut(&mut self) -> Option<&mut ExprName>
fn as_named_expr(&self) -> Option<&ExprNamed>
fn as_named_expr_mut(&mut self) -> Option<&mut ExprNamed>
fn as_none_literal_expr(&self) -> Option<&ExprNoneLiteral>
fn as_none_literal_expr_mut(&mut self) -> Option<&mut ExprNoneLiteral>
fn as_number_literal_expr(&self) -> Option<&ExprNumberLiteral>
fn as_number_literal_expr_mut(&mut self) -> Option<&mut ExprNumberLiteral>
fn as_set_comp_expr(&self) -> Option<&ExprSetComp>
fn as_set_comp_expr_mut(&mut self) -> Option<&mut ExprSetComp>
fn as_set_expr(&self) -> Option<&ExprSet>
fn as_set_expr_mut(&mut self) -> Option<&mut ExprSet>
fn as_slice_expr(&self) -> Option<&ExprSlice>
fn as_slice_expr_mut(&mut self) -> Option<&mut ExprSlice>
fn as_starred_expr(&self) -> Option<&ExprStarred>
fn as_starred_expr_mut(&mut self) -> Option<&mut ExprStarred>
fn as_string_literal_expr(&self) -> Option<&ExprStringLiteral>
fn as_string_literal_expr_mut(&mut self) -> Option<&mut ExprStringLiteral>
fn as_subscript_expr(&self) -> Option<&ExprSubscript>
fn as_subscript_expr_mut(&mut self) -> Option<&mut ExprSubscript>
fn as_t_string_expr(&self) -> Option<&ExprTString>
fn as_t_string_expr_mut(&mut self) -> Option<&mut ExprTString>
fn as_tuple_expr(&self) -> Option<&ExprTuple>
fn as_tuple_expr_mut(&mut self) -> Option<&mut ExprTuple>
fn as_unary_op_expr(&self) -> Option<&ExprUnaryOp>
fn as_unary_op_expr_mut(&mut self) -> Option<&mut ExprUnaryOp>
fn as_yield_expr(&self) -> Option<&ExprYield>
fn as_yield_expr_mut(&mut self) -> Option<&mut ExprYield>
fn as_yield_from_expr(&self) -> Option<&ExprYieldFrom>
fn as_yield_from_expr_mut(&mut self) -> Option<&mut ExprYieldFrom>
fn attribute_expr(self) -> Option<ExprAttribute>
fn await_expr(self) -> Option<ExprAwait>
fn bin_op_expr(self) -> Option<ExprBinOp>
fn bool_op_expr(self) -> Option<ExprBoolOp>
fn boolean_literal_expr(self) -> Option<ExprBooleanLiteral>
fn bytes_literal_expr(self) -> Option<ExprBytesLiteral>
fn call_expr(self) -> Option<ExprCall>
fn compare_expr(self) -> Option<ExprCompare>
fn dict_comp_expr(self) -> Option<ExprDictComp>
fn dict_expr(self) -> Option<ExprDict>
fn ellipsis_literal_expr(self) -> Option<ExprEllipsisLiteral>
fn expect_attribute_expr(self) -> ExprAttribute
fn expect_await_expr(self) -> ExprAwait
fn expect_bin_op_expr(self) -> ExprBinOp
fn expect_bool_op_expr(self) -> ExprBoolOp
fn expect_boolean_literal_expr(self) -> ExprBooleanLiteral
fn expect_bytes_literal_expr(self) -> ExprBytesLiteral
fn expect_call_expr(self) -> ExprCall
fn expect_compare_expr(self) -> ExprCompare
fn expect_dict_comp_expr(self) -> ExprDictComp
fn expect_dict_expr(self) -> ExprDict
fn expect_ellipsis_literal_expr(self) -> ExprEllipsisLiteral
fn expect_f_string_expr(self) -> ExprFString
fn expect_generator_expr(self) -> ExprGenerator
fn expect_if_expr(self) -> ExprIf
fn expect_ipy_escape_command_expr(self) -> ExprIpyEscapeCommand
fn expect_lambda_expr(self) -> ExprLambda
fn expect_list_comp_expr(self) -> ExprListComp
fn expect_list_expr(self) -> ExprList
fn expect_name_expr(self) -> ExprName
fn expect_named_expr(self) -> ExprNamed
fn expect_none_literal_expr(self) -> ExprNoneLiteral
fn expect_number_literal_expr(self) -> ExprNumberLiteral
fn expect_set_comp_expr(self) -> ExprSetComp
fn expect_set_expr(self) -> ExprSet
fn expect_slice_expr(self) -> ExprSlice
fn expect_starred_expr(self) -> ExprStarred
fn expect_string_literal_expr(self) -> ExprStringLiteral
fn expect_subscript_expr(self) -> ExprSubscript
fn expect_t_string_expr(self) -> ExprTString
fn expect_tuple_expr(self) -> ExprTuple
fn expect_unary_op_expr(self) -> ExprUnaryOp
fn expect_yield_expr(self) -> ExprYield
fn expect_yield_from_expr(self) -> ExprYieldFrom
fn expression_value(&self) -> &Self
fn f_string_expr(self) -> Option<ExprFString>
fn generator_expr(self) -> Option<ExprGenerator>
fn if_expr(self) -> Option<ExprIf>
fn ipy_escape_command_expr(self) -> Option<ExprIpyEscapeCommand>
const fn is_attribute_expr(&self) -> bool
const fn is_await_expr(&self) -> bool
const fn is_bin_op_expr(&self) -> bool
const fn is_bool_op_expr(&self) -> bool
const fn is_boolean_literal_expr(&self) -> bool
const fn is_bytes_literal_expr(&self) -> bool
const fn is_call_expr(&self) -> bool
const fn is_compare_expr(&self) -> bool
const fn is_dict_comp_expr(&self) -> bool
const fn is_dict_expr(&self) -> bool
const fn is_ellipsis_literal_expr(&self) -> bool
const fn is_f_string_expr(&self) -> bool
const fn is_generator_expr(&self) -> bool
const fn is_if_expr(&self) -> bool
const fn is_ipy_escape_command_expr(&self) -> bool
const fn is_lambda_expr(&self) -> bool
const fn is_list_comp_expr(&self) -> bool
const fn is_list_expr(&self) -> bool
fn is_literal_expr(&self) -> bool
const fn is_name_expr(&self) -> bool
const fn is_named_expr(&self) -> bool
const fn is_none_literal_expr(&self) -> bool
const fn is_number_literal_expr(&self) -> bool
const fn is_set_comp_expr(&self) -> bool
const fn is_set_expr(&self) -> bool
const fn is_slice_expr(&self) -> bool
const fn is_starred_expr(&self) -> bool
const fn is_string_literal_expr(&self) -> bool
const fn is_subscript_expr(&self) -> bool
const fn is_t_string_expr(&self) -> bool
const fn is_tuple_expr(&self) -> bool
const fn is_unary_op_expr(&self) -> bool
const fn is_yield_expr(&self) -> bool
const fn is_yield_from_expr(&self) -> bool
fn lambda_expr(self) -> Option<ExprLambda>
fn list_comp_expr(self) -> Option<ExprListComp>
fn list_expr(self) -> Option<ExprList>
fn name_expr(self) -> Option<ExprName>
fn named_expr(self) -> Option<ExprNamed>
fn none_literal_expr(self) -> Option<ExprNoneLiteral>
fn number_literal_expr(self) -> Option<ExprNumberLiteral>
fn precedence(&self) -> OperatorPrecedence
fn set_comp_expr(self) -> Option<ExprSetComp>
fn set_expr(self) -> Option<ExprSet>
fn slice_expr(self) -> Option<ExprSlice>
fn starred_expr(self) -> Option<ExprStarred>
fn string_literal_expr(self) -> Option<ExprStringLiteral>
fn subscript_expr(self) -> Option<ExprSubscript>
fn t_string_expr(self) -> Option<ExprTString>
fn tuple_expr(self) -> Option<ExprTuple>
fn unary_op_expr(self) -> Option<ExprUnaryOp>
fn yield_expr(self) -> Option<ExprYield>
fn yield_from_expr(self) -> Option<ExprYieldFrom>
```

**via `core::convert::From`**

```rust
fn from(node: ExprName) -> Self
fn from(node: ExprAwait) -> Self
fn from(node: ExprSlice) -> Self
fn from(node: ExprCompare) -> Self
fn from(node: ExprTString) -> Self
fn from(node: ExprBinOp) -> Self
fn from(node: ExprNumberLiteral) -> Self
fn from(node: ExprIf) -> Self
fn from(payload: StringLiteral) -> Self
fn from(node: ExprEllipsisLiteral) -> Self
fn from(node: ExprListComp) -> Self
fn from(node: ExprStarred) -> Self
fn from(node: ExprGenerator) -> Self
fn from(node: ExprTuple) -> Self
fn from(node: ExprYieldFrom) -> Self
fn from(node: ExprFString) -> Self
fn from(node: ExprNamed) -> Self
fn from(node: ExprBytesLiteral) -> Self
fn from(node: ExprLambda) -> Self
fn from(payload: TString) -> Self
fn from(node: ExprNoneLiteral) -> Self
fn from(node: ExprSet) -> Self
fn from(node: ExprSubscript) -> Self
fn from(node: ExprDictComp) -> Self
fn from(node: ExprList) -> Self
fn from(node: ExprYield) -> Self
fn from(node: ExprIpyEscapeCommand) -> Self
fn from(node: ExprCall) -> Self
fn from(node: ExprBoolOp) -> Self
fn from(node: ExprStringLiteral) -> Self
fn from(node: ExprUnaryOp) -> Self
fn from(payload: FString) -> Self
fn from(node: ExprBooleanLiteral) -> Self
fn from(node: ExprDict) -> Self
fn from(payload: BytesLiteral) -> Self
fn from(node: ExprAttribute) -> Self
fn from(node: ExprSetComp) -> Self
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [expr](https://docs.python.org/3/library/ast.html#ast.expr)

---

## ExprRef

`enum` · `ruff_python_ast::generated::ExprRef`

Also reachable as `ruff_python_ast::ExprRef`

```rust
enum ExprRef<'a>
```

**Variants**: `BoolOp`, `Named`, `BinOp`, `UnaryOp`, `Lambda`, `If`, `Dict`, `Set`, `ListComp`, `SetComp`, `DictComp`, `Generator`, `Await`, `Yield`, `YieldFrom`, `Compare`, `Call`, `FString`, `TString`, `StringLiteral`, `BytesLiteral`, `NumberLiteral`, `BooleanLiteral`, `NoneLiteral`, `EllipsisLiteral`, `Attribute`, `Subscript`, `Starred`, `Name`, `List`, `Tuple`, `Slice`, `IpyEscapeCommand`

**Implements**: `core::convert::From`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (167)

```rust
fn as_attribute_expr(&self) -> Option<&&'a ExprAttribute>
fn as_await_expr(&self) -> Option<&&'a ExprAwait>
fn as_bin_op_expr(&self) -> Option<&&'a ExprBinOp>
fn as_bool_op_expr(&self) -> Option<&&'a ExprBoolOp>
fn as_boolean_literal_expr(&self) -> Option<&&'a ExprBooleanLiteral>
fn as_bytes_literal_expr(&self) -> Option<&&'a ExprBytesLiteral>
fn as_call_expr(&self) -> Option<&&'a ExprCall>
fn as_compare_expr(&self) -> Option<&&'a ExprCompare>
fn as_dict_comp_expr(&self) -> Option<&&'a ExprDictComp>
fn as_dict_expr(&self) -> Option<&&'a ExprDict>
fn as_ellipsis_literal_expr(&self) -> Option<&&'a ExprEllipsisLiteral>
fn as_f_string_expr(&self) -> Option<&&'a ExprFString>
fn as_generator_expr(&self) -> Option<&&'a ExprGenerator>
fn as_if_expr(&self) -> Option<&&'a ExprIf>
fn as_ipy_escape_command_expr(&self) -> Option<&&'a ExprIpyEscapeCommand>
fn as_lambda_expr(&self) -> Option<&&'a ExprLambda>
fn as_list_comp_expr(&self) -> Option<&&'a ExprListComp>
fn as_list_expr(&self) -> Option<&&'a ExprList>
fn as_mut_attribute_expr(&mut self) -> Option<&mut &'a ExprAttribute>
fn as_mut_await_expr(&mut self) -> Option<&mut &'a ExprAwait>
fn as_mut_bin_op_expr(&mut self) -> Option<&mut &'a ExprBinOp>
fn as_mut_bool_op_expr(&mut self) -> Option<&mut &'a ExprBoolOp>
fn as_mut_boolean_literal_expr(&mut self) -> Option<&mut &'a ExprBooleanLiteral>
fn as_mut_bytes_literal_expr(&mut self) -> Option<&mut &'a ExprBytesLiteral>
fn as_mut_call_expr(&mut self) -> Option<&mut &'a ExprCall>
fn as_mut_compare_expr(&mut self) -> Option<&mut &'a ExprCompare>
fn as_mut_dict_comp_expr(&mut self) -> Option<&mut &'a ExprDictComp>
fn as_mut_dict_expr(&mut self) -> Option<&mut &'a ExprDict>
fn as_mut_ellipsis_literal_expr(&mut self) -> Option<&mut &'a ExprEllipsisLiteral>
fn as_mut_f_string_expr(&mut self) -> Option<&mut &'a ExprFString>
fn as_mut_generator_expr(&mut self) -> Option<&mut &'a ExprGenerator>
fn as_mut_if_expr(&mut self) -> Option<&mut &'a ExprIf>
fn as_mut_ipy_escape_command_expr(&mut self) -> Option<&mut &'a ExprIpyEscapeCommand>
fn as_mut_lambda_expr(&mut self) -> Option<&mut &'a ExprLambda>
fn as_mut_list_comp_expr(&mut self) -> Option<&mut &'a ExprListComp>
fn as_mut_list_expr(&mut self) -> Option<&mut &'a ExprList>
fn as_mut_name_expr(&mut self) -> Option<&mut &'a ExprName>
fn as_mut_named_expr(&mut self) -> Option<&mut &'a ExprNamed>
fn as_mut_none_literal_expr(&mut self) -> Option<&mut &'a ExprNoneLiteral>
fn as_mut_number_literal_expr(&mut self) -> Option<&mut &'a ExprNumberLiteral>
fn as_mut_set_comp_expr(&mut self) -> Option<&mut &'a ExprSetComp>
fn as_mut_set_expr(&mut self) -> Option<&mut &'a ExprSet>
fn as_mut_slice_expr(&mut self) -> Option<&mut &'a ExprSlice>
fn as_mut_starred_expr(&mut self) -> Option<&mut &'a ExprStarred>
fn as_mut_string_literal_expr(&mut self) -> Option<&mut &'a ExprStringLiteral>
fn as_mut_subscript_expr(&mut self) -> Option<&mut &'a ExprSubscript>
fn as_mut_t_string_expr(&mut self) -> Option<&mut &'a ExprTString>
fn as_mut_tuple_expr(&mut self) -> Option<&mut &'a ExprTuple>
fn as_mut_unary_op_expr(&mut self) -> Option<&mut &'a ExprUnaryOp>
fn as_mut_yield_expr(&mut self) -> Option<&mut &'a ExprYield>
fn as_mut_yield_from_expr(&mut self) -> Option<&mut &'a ExprYieldFrom>
fn as_name_expr(&self) -> Option<&&'a ExprName>
fn as_named_expr(&self) -> Option<&&'a ExprNamed>
fn as_none_literal_expr(&self) -> Option<&&'a ExprNoneLiteral>
fn as_number_literal_expr(&self) -> Option<&&'a ExprNumberLiteral>
fn as_set_comp_expr(&self) -> Option<&&'a ExprSetComp>
fn as_set_expr(&self) -> Option<&&'a ExprSet>
fn as_slice_expr(&self) -> Option<&&'a ExprSlice>
fn as_starred_expr(&self) -> Option<&&'a ExprStarred>
fn as_string_literal_expr(&self) -> Option<&&'a ExprStringLiteral>
fn as_subscript_expr(&self) -> Option<&&'a ExprSubscript>
fn as_t_string_expr(&self) -> Option<&&'a ExprTString>
fn as_tuple_expr(&self) -> Option<&&'a ExprTuple>
fn as_unary_op_expr(&self) -> Option<&&'a ExprUnaryOp>
fn as_yield_expr(&self) -> Option<&&'a ExprYield>
fn as_yield_from_expr(&self) -> Option<&&'a ExprYieldFrom>
fn attribute_expr(self) -> Option<&'a ExprAttribute>
fn await_expr(self) -> Option<&'a ExprAwait>
fn bin_op_expr(self) -> Option<&'a ExprBinOp>
fn bool_op_expr(self) -> Option<&'a ExprBoolOp>
fn boolean_literal_expr(self) -> Option<&'a ExprBooleanLiteral>
fn bytes_literal_expr(self) -> Option<&'a ExprBytesLiteral>
fn call_expr(self) -> Option<&'a ExprCall>
fn compare_expr(self) -> Option<&'a ExprCompare>
fn dict_comp_expr(self) -> Option<&'a ExprDictComp>
fn dict_expr(self) -> Option<&'a ExprDict>
fn ellipsis_literal_expr(self) -> Option<&'a ExprEllipsisLiteral>
fn expect_attribute_expr(self) -> &'a ExprAttribute where Self: ::std::fmt::Debug
fn expect_await_expr(self) -> &'a ExprAwait where Self: ::std::fmt::Debug
fn expect_bin_op_expr(self) -> &'a ExprBinOp where Self: ::std::fmt::Debug
fn expect_bool_op_expr(self) -> &'a ExprBoolOp where Self: ::std::fmt::Debug
fn expect_boolean_literal_expr(self) -> &'a ExprBooleanLiteral where Self: ::std::fmt::Debug
fn expect_bytes_literal_expr(self) -> &'a ExprBytesLiteral where Self: ::std::fmt::Debug
fn expect_call_expr(self) -> &'a ExprCall where Self: ::std::fmt::Debug
fn expect_compare_expr(self) -> &'a ExprCompare where Self: ::std::fmt::Debug
fn expect_dict_comp_expr(self) -> &'a ExprDictComp where Self: ::std::fmt::Debug
fn expect_dict_expr(self) -> &'a ExprDict where Self: ::std::fmt::Debug
fn expect_ellipsis_literal_expr(self) -> &'a ExprEllipsisLiteral where Self: ::std::fmt::Debug
fn expect_f_string_expr(self) -> &'a ExprFString where Self: ::std::fmt::Debug
fn expect_generator_expr(self) -> &'a ExprGenerator where Self: ::std::fmt::Debug
fn expect_if_expr(self) -> &'a ExprIf where Self: ::std::fmt::Debug
fn expect_ipy_escape_command_expr(self) -> &'a ExprIpyEscapeCommand where Self: ::std::fmt::Debug
fn expect_lambda_expr(self) -> &'a ExprLambda where Self: ::std::fmt::Debug
fn expect_list_comp_expr(self) -> &'a ExprListComp where Self: ::std::fmt::Debug
fn expect_list_expr(self) -> &'a ExprList where Self: ::std::fmt::Debug
fn expect_name_expr(self) -> &'a ExprName where Self: ::std::fmt::Debug
fn expect_named_expr(self) -> &'a ExprNamed where Self: ::std::fmt::Debug
fn expect_none_literal_expr(self) -> &'a ExprNoneLiteral where Self: ::std::fmt::Debug
fn expect_number_literal_expr(self) -> &'a ExprNumberLiteral where Self: ::std::fmt::Debug
fn expect_set_comp_expr(self) -> &'a ExprSetComp where Self: ::std::fmt::Debug
fn expect_set_expr(self) -> &'a ExprSet where Self: ::std::fmt::Debug
fn expect_slice_expr(self) -> &'a ExprSlice where Self: ::std::fmt::Debug
fn expect_starred_expr(self) -> &'a ExprStarred where Self: ::std::fmt::Debug
fn expect_string_literal_expr(self) -> &'a ExprStringLiteral where Self: ::std::fmt::Debug
fn expect_subscript_expr(self) -> &'a ExprSubscript where Self: ::std::fmt::Debug
fn expect_t_string_expr(self) -> &'a ExprTString where Self: ::std::fmt::Debug
fn expect_tuple_expr(self) -> &'a ExprTuple where Self: ::std::fmt::Debug
fn expect_unary_op_expr(self) -> &'a ExprUnaryOp where Self: ::std::fmt::Debug
fn expect_yield_expr(self) -> &'a ExprYield where Self: ::std::fmt::Debug
fn expect_yield_from_expr(self) -> &'a ExprYieldFrom where Self: ::std::fmt::Debug
fn f_string_expr(self) -> Option<&'a ExprFString>
fn generator_expr(self) -> Option<&'a ExprGenerator>
fn if_expr(self) -> Option<&'a ExprIf>
fn ipy_escape_command_expr(self) -> Option<&'a ExprIpyEscapeCommand>
const fn is_attribute_expr(&self) -> bool
const fn is_await_expr(&self) -> bool
const fn is_bin_op_expr(&self) -> bool
const fn is_bool_op_expr(&self) -> bool
const fn is_boolean_literal_expr(&self) -> bool
const fn is_bytes_literal_expr(&self) -> bool
const fn is_call_expr(&self) -> bool
const fn is_compare_expr(&self) -> bool
const fn is_dict_comp_expr(&self) -> bool
const fn is_dict_expr(&self) -> bool
const fn is_ellipsis_literal_expr(&self) -> bool
const fn is_f_string_expr(&self) -> bool
const fn is_generator_expr(&self) -> bool
const fn is_if_expr(&self) -> bool
const fn is_ipy_escape_command_expr(&self) -> bool
const fn is_lambda_expr(&self) -> bool
const fn is_list_comp_expr(&self) -> bool
const fn is_list_expr(&self) -> bool
fn is_literal_expr(&self) -> bool
const fn is_name_expr(&self) -> bool
const fn is_named_expr(&self) -> bool
const fn is_none_literal_expr(&self) -> bool
const fn is_number_literal_expr(&self) -> bool
const fn is_set_comp_expr(&self) -> bool
const fn is_set_expr(&self) -> bool
const fn is_slice_expr(&self) -> bool
const fn is_starred_expr(&self) -> bool
const fn is_string_literal_expr(&self) -> bool
const fn is_subscript_expr(&self) -> bool
const fn is_t_string_expr(&self) -> bool
const fn is_tuple_expr(&self) -> bool
const fn is_unary_op_expr(&self) -> bool
const fn is_yield_expr(&self) -> bool
const fn is_yield_from_expr(&self) -> bool
fn lambda_expr(self) -> Option<&'a ExprLambda>
fn list_comp_expr(self) -> Option<&'a ExprListComp>
fn list_expr(self) -> Option<&'a ExprList>
fn name_expr(self) -> Option<&'a ExprName>
fn named_expr(self) -> Option<&'a ExprNamed>
fn none_literal_expr(self) -> Option<&'a ExprNoneLiteral>
fn number_literal_expr(self) -> Option<&'a ExprNumberLiteral>
fn precedence(&self) -> OperatorPrecedence
fn set_comp_expr(self) -> Option<&'a ExprSetComp>
fn set_expr(self) -> Option<&'a ExprSet>
fn slice_expr(self) -> Option<&'a ExprSlice>
fn starred_expr(self) -> Option<&'a ExprStarred>
fn string_literal_expr(self) -> Option<&'a ExprStringLiteral>
fn subscript_expr(self) -> Option<&'a ExprSubscript>
fn t_string_expr(self) -> Option<&'a ExprTString>
fn tuple_expr(self) -> Option<&'a ExprTuple>
fn unary_op_expr(self) -> Option<&'a ExprUnaryOp>
fn yield_expr(self) -> Option<&'a ExprYield>
fn yield_from_expr(self) -> Option<&'a ExprYieldFrom>
```

**via `core::convert::From`**

```rust
fn from(node: &'a ExprBinOp) -> Self
fn from(node: &'a ExprNumberLiteral) -> Self
fn from(node: &'a ExprIf) -> Self
fn from(node: &'a ExprEllipsisLiteral) -> Self
fn from(node: &'a ExprListComp) -> Self
fn from(node: &'a ExprStarred) -> Self
fn from(node: &'a ExprGenerator) -> Self
fn from(node: &'a ExprTuple) -> Self
fn from(node: &'a ExprYieldFrom) -> Self
fn from(node: &'a ExprFString) -> Self
fn from(node: &'a ExprNamed) -> Self
fn from(node: &'a ExprBytesLiteral) -> Self
fn from(node: &'a ExprLambda) -> Self
fn from(node: &'a ExprNoneLiteral) -> Self
fn from(node: &'a ExprSet) -> Self
fn from(node: &'a ExprSubscript) -> Self
fn from(node: &'a ExprDictComp) -> Self
fn from(node: &'a ExprList) -> Self
fn from(node: &'a ExprYield) -> Self
fn from(node: &'a ExprIpyEscapeCommand) -> Self
fn from(node: &'a ExprCall) -> Self
fn from(node: &'a ExprBoolOp) -> Self
fn from(node: &'a ExprStringLiteral) -> Self
fn from(node: &'a ExprUnaryOp) -> Self
fn from(node: &'a ExprBooleanLiteral) -> Self
fn from(value: &'a Box<Expr>) -> Self
fn from(node: &'a ExprDict) -> Self
fn from(node: &'a ExprAttribute) -> Self
fn from(node: &'a Expr) -> Self
fn from(node: &'a ExprSetComp) -> Self
fn from(node: &'a ExprName) -> Self
fn from(node: &'a ExprAwait) -> Self
fn from(node: &'a ExprSlice) -> Self
fn from(value: &StringLike<'a>) -> Self
fn from(node: &'a ExprCompare) -> Self
fn from(node: &'a ExprTString) -> Self
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [expr](https://docs.python.org/3/library/ast.html#ast.expr)

---

## InterpolatedStringElement

`enum` · `ruff_python_ast::generated::InterpolatedStringElement`

Also reachable as `ruff_python_ast::InterpolatedStringElement`

```rust
enum InterpolatedStringElement
```

**Variants**: `Interpolation`, `Literal`

**Implements**: `core::convert::From`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn as_interpolation(&self) -> Option<&InterpolatedElement>
fn as_interpolation_mut(&mut self) -> Option<&mut InterpolatedElement>
fn as_literal(&self) -> Option<&InterpolatedStringLiteralElement>
fn as_literal_mut(&mut self) -> Option<&mut InterpolatedStringLiteralElement>
fn expect_interpolation(self) -> InterpolatedElement
fn expect_literal(self) -> InterpolatedStringLiteralElement
fn interpolation(self) -> Option<InterpolatedElement>
const fn is_interpolation(&self) -> bool
const fn is_literal(&self) -> bool
fn literal(self) -> Option<InterpolatedStringLiteralElement>
```

**via `core::convert::From`**

```rust
fn from(node: InterpolatedStringLiteralElement) -> Self
fn from(node: InterpolatedElement) -> Self
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

---

## InterpolatedStringElementRef

`enum` · `ruff_python_ast::generated::InterpolatedStringElementRef`

Also reachable as `ruff_python_ast::InterpolatedStringElementRef`

```rust
enum InterpolatedStringElementRef<'a>
```

**Variants**: `Interpolation`, `Literal`

**Implements**: `core::convert::From`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn as_interpolation(&self) -> Option<&&'a InterpolatedElement>
fn as_literal(&self) -> Option<&&'a InterpolatedStringLiteralElement>
fn as_mut_interpolation(&mut self) -> Option<&mut &'a InterpolatedElement>
fn as_mut_literal(&mut self) -> Option<&mut &'a InterpolatedStringLiteralElement>
fn expect_interpolation(self) -> &'a InterpolatedElement where Self: ::std::fmt::Debug
fn expect_literal(self) -> &'a InterpolatedStringLiteralElement where Self: ::std::fmt::Debug
fn interpolation(self) -> Option<&'a InterpolatedElement>
const fn is_interpolation(&self) -> bool
const fn is_literal(&self) -> bool
fn literal(self) -> Option<&'a InterpolatedStringLiteralElement>
```

**via `core::convert::From`**

```rust
fn from(node: &'a InterpolatedStringLiteralElement) -> Self
fn from(node: &'a InterpolatedStringElement) -> Self
fn from(node: &'a InterpolatedElement) -> Self
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

---

## Mod

`enum` · `ruff_python_ast::generated::Mod`

Also reachable as `ruff_python_ast::Mod`

```rust
enum Mod
```

**Variants**: `Module`, `Expression`

**Implements**: `core::convert::From`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn as_expression(&self) -> Option<&ModExpression>
fn as_expression_mut(&mut self) -> Option<&mut ModExpression>
fn as_module(&self) -> Option<&ModModule>
fn as_module_mut(&mut self) -> Option<&mut ModModule>
fn expect_expression(self) -> ModExpression
fn expect_module(self) -> ModModule
fn expression(self) -> Option<ModExpression>
const fn is_expression(&self) -> bool
const fn is_module(&self) -> bool
fn module(self) -> Option<ModModule>
```

**via `core::convert::From`**

```rust
fn from(node: ModExpression) -> Self
fn from(node: ModModule) -> Self
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [mod](https://docs.python.org/3/library/ast.html#ast.mod)

---

## ModRef

`enum` · `ruff_python_ast::generated::ModRef`

Also reachable as `ruff_python_ast::ModRef`

```rust
enum ModRef<'a>
```

**Variants**: `Module`, `Expression`

**Implements**: `core::convert::From`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn as_expression(&self) -> Option<&&'a ModExpression>
fn as_module(&self) -> Option<&&'a ModModule>
fn as_mut_expression(&mut self) -> Option<&mut &'a ModExpression>
fn as_mut_module(&mut self) -> Option<&mut &'a ModModule>
fn expect_expression(self) -> &'a ModExpression where Self: ::std::fmt::Debug
fn expect_module(self) -> &'a ModModule where Self: ::std::fmt::Debug
fn expression(self) -> Option<&'a ModExpression>
const fn is_expression(&self) -> bool
const fn is_module(&self) -> bool
fn module(self) -> Option<&'a ModModule>
```

**via `core::convert::From`**

```rust
fn from(node: &'a ModModule) -> Self
fn from(node: &'a Mod) -> Self
fn from(node: &'a ModExpression) -> Self
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [mod](https://docs.python.org/3/library/ast.html#ast.mod)

---

## NodeKind

`enum` · `ruff_python_ast::generated::NodeKind`

Also reachable as `ruff_python_ast::NodeKind`

```rust
enum NodeKind
```

**Variants**: `ModModule`, `ModExpression`, `StmtFunctionDef`, `StmtClassDef`, `StmtReturn`, `StmtDelete`, `StmtTypeAlias`, `StmtAssign`, `StmtAugAssign`, `StmtAnnAssign`, `StmtFor`, `StmtWhile`, `StmtIf`, `StmtWith`, `StmtMatch`, `StmtRaise`, `StmtTry`, `StmtAssert`, `StmtImport`, `StmtImportFrom`, `StmtGlobal`, `StmtNonlocal`, `StmtExpr`, `StmtPass`, `StmtBreak`, `StmtContinue`, `StmtIpyEscapeCommand`, `ExprBoolOp`, `ExprNamed`, `ExprBinOp`, `ExprUnaryOp`, `ExprLambda`, `ExprIf`, `ExprDict`, `ExprSet`, `ExprListComp`, `ExprSetComp`, `ExprDictComp`, `ExprGenerator`, `ExprAwait`, `ExprYield`, `ExprYieldFrom`, `ExprCompare`, `ExprCall`, `ExprFString`, `ExprTString`, `ExprStringLiteral`, `ExprBytesLiteral`, `ExprNumberLiteral`, `ExprBooleanLiteral`, `ExprNoneLiteral`, `ExprEllipsisLiteral`, `ExprAttribute`, `ExprSubscript`, `ExprStarred`, `ExprName`, `ExprList`, `ExprTuple`, `ExprSlice`, `ExprIpyEscapeCommand`, `ExceptHandlerExceptHandler`, `InterpolatedElement`, `InterpolatedStringLiteralElement`, `PatternMatchValue`, `PatternMatchSingleton`, `PatternMatchSequence`, `PatternMatchMapping`, `PatternMatchClass`, `PatternMatchStar`, `PatternMatchAs`, `PatternMatchOr`, `TypeParamTypeVar`, `TypeParamTypeVarTuple`, `TypeParamParamSpec`, `InterpolatedStringFormatSpec`, `PatternArguments`, `PatternKeyword`, `Comprehension`, `Arguments`, `Parameters`, `Parameter`, `ParameterWithDefault`, `Keyword`, `Alias`, `WithItem`, `MatchCase`, `Decorator`, `ElifElseClause`, `TypeParams`, `FString`, `TString`, `StringLiteral`, `BytesLiteral`, `Identifier`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

---

## Pattern

`enum` · `ruff_python_ast::generated::Pattern`

Also reachable as `ruff_python_ast::Pattern`

```rust
enum Pattern
```

**Variants**: `MatchValue`, `MatchSingleton`, `MatchSequence`, `MatchMapping`, `MatchClass`, `MatchStar`, `MatchAs`, `MatchOr`

**Implements**: `core::convert::From`, `pyrefly_util::visit::Visit`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (43)

```rust
fn as_match_as(&self) -> Option<&PatternMatchAs>
fn as_match_as_mut(&mut self) -> Option<&mut PatternMatchAs>
fn as_match_class(&self) -> Option<&PatternMatchClass>
fn as_match_class_mut(&mut self) -> Option<&mut PatternMatchClass>
fn as_match_mapping(&self) -> Option<&PatternMatchMapping>
fn as_match_mapping_mut(&mut self) -> Option<&mut PatternMatchMapping>
fn as_match_or(&self) -> Option<&PatternMatchOr>
fn as_match_or_mut(&mut self) -> Option<&mut PatternMatchOr>
fn as_match_sequence(&self) -> Option<&PatternMatchSequence>
fn as_match_sequence_mut(&mut self) -> Option<&mut PatternMatchSequence>
fn as_match_singleton(&self) -> Option<&PatternMatchSingleton>
fn as_match_singleton_mut(&mut self) -> Option<&mut PatternMatchSingleton>
fn as_match_star(&self) -> Option<&PatternMatchStar>
fn as_match_star_mut(&mut self) -> Option<&mut PatternMatchStar>
fn as_match_value(&self) -> Option<&PatternMatchValue>
fn as_match_value_mut(&mut self) -> Option<&mut PatternMatchValue>
fn expect_match_as(self) -> PatternMatchAs
fn expect_match_class(self) -> PatternMatchClass
fn expect_match_mapping(self) -> PatternMatchMapping
fn expect_match_or(self) -> PatternMatchOr
fn expect_match_sequence(self) -> PatternMatchSequence
fn expect_match_singleton(self) -> PatternMatchSingleton
fn expect_match_star(self) -> PatternMatchStar
fn expect_match_value(self) -> PatternMatchValue
fn irrefutable_pattern(&self) -> Option<IrrefutablePattern>
fn is_irrefutable(&self) -> bool
const fn is_match_as(&self) -> bool
const fn is_match_class(&self) -> bool
const fn is_match_mapping(&self) -> bool
const fn is_match_or(&self) -> bool
const fn is_match_sequence(&self) -> bool
const fn is_match_singleton(&self) -> bool
const fn is_match_star(&self) -> bool
const fn is_match_value(&self) -> bool
fn is_wildcard(&self) -> bool
fn match_as(self) -> Option<PatternMatchAs>
fn match_class(self) -> Option<PatternMatchClass>
fn match_mapping(self) -> Option<PatternMatchMapping>
fn match_or(self) -> Option<PatternMatchOr>
fn match_sequence(self) -> Option<PatternMatchSequence>
fn match_singleton(self) -> Option<PatternMatchSingleton>
fn match_star(self) -> Option<PatternMatchStar>
fn match_value(self) -> Option<PatternMatchValue>
```

**via `core::convert::From`**

```rust
fn from(node: PatternMatchValue) -> Self
fn from(node: PatternMatchMapping) -> Self
fn from(node: PatternMatchAs) -> Self
fn from(node: PatternMatchSequence) -> Self
fn from(node: PatternMatchStar) -> Self
fn from(node: PatternMatchSingleton) -> Self
fn from(node: PatternMatchClass) -> Self
fn from(node: PatternMatchOr) -> Self
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [pattern](https://docs.python.org/3/library/ast.html#ast.pattern)

---

## PatternRef

`enum` · `ruff_python_ast::generated::PatternRef`

Also reachable as `ruff_python_ast::PatternRef`

```rust
enum PatternRef<'a>
```

**Variants**: `MatchValue`, `MatchSingleton`, `MatchSequence`, `MatchMapping`, `MatchClass`, `MatchStar`, `MatchAs`, `MatchOr`

**Implements**: `core::convert::From`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (40)

```rust
fn as_match_as(&self) -> Option<&&'a PatternMatchAs>
fn as_match_class(&self) -> Option<&&'a PatternMatchClass>
fn as_match_mapping(&self) -> Option<&&'a PatternMatchMapping>
fn as_match_or(&self) -> Option<&&'a PatternMatchOr>
fn as_match_sequence(&self) -> Option<&&'a PatternMatchSequence>
fn as_match_singleton(&self) -> Option<&&'a PatternMatchSingleton>
fn as_match_star(&self) -> Option<&&'a PatternMatchStar>
fn as_match_value(&self) -> Option<&&'a PatternMatchValue>
fn as_mut_match_as(&mut self) -> Option<&mut &'a PatternMatchAs>
fn as_mut_match_class(&mut self) -> Option<&mut &'a PatternMatchClass>
fn as_mut_match_mapping(&mut self) -> Option<&mut &'a PatternMatchMapping>
fn as_mut_match_or(&mut self) -> Option<&mut &'a PatternMatchOr>
fn as_mut_match_sequence(&mut self) -> Option<&mut &'a PatternMatchSequence>
fn as_mut_match_singleton(&mut self) -> Option<&mut &'a PatternMatchSingleton>
fn as_mut_match_star(&mut self) -> Option<&mut &'a PatternMatchStar>
fn as_mut_match_value(&mut self) -> Option<&mut &'a PatternMatchValue>
fn expect_match_as(self) -> &'a PatternMatchAs where Self: ::std::fmt::Debug
fn expect_match_class(self) -> &'a PatternMatchClass where Self: ::std::fmt::Debug
fn expect_match_mapping(self) -> &'a PatternMatchMapping where Self: ::std::fmt::Debug
fn expect_match_or(self) -> &'a PatternMatchOr where Self: ::std::fmt::Debug
fn expect_match_sequence(self) -> &'a PatternMatchSequence where Self: ::std::fmt::Debug
fn expect_match_singleton(self) -> &'a PatternMatchSingleton where Self: ::std::fmt::Debug
fn expect_match_star(self) -> &'a PatternMatchStar where Self: ::std::fmt::Debug
fn expect_match_value(self) -> &'a PatternMatchValue where Self: ::std::fmt::Debug
const fn is_match_as(&self) -> bool
const fn is_match_class(&self) -> bool
const fn is_match_mapping(&self) -> bool
const fn is_match_or(&self) -> bool
const fn is_match_sequence(&self) -> bool
const fn is_match_singleton(&self) -> bool
const fn is_match_star(&self) -> bool
const fn is_match_value(&self) -> bool
fn match_as(self) -> Option<&'a PatternMatchAs>
fn match_class(self) -> Option<&'a PatternMatchClass>
fn match_mapping(self) -> Option<&'a PatternMatchMapping>
fn match_or(self) -> Option<&'a PatternMatchOr>
fn match_sequence(self) -> Option<&'a PatternMatchSequence>
fn match_singleton(self) -> Option<&'a PatternMatchSingleton>
fn match_star(self) -> Option<&'a PatternMatchStar>
fn match_value(self) -> Option<&'a PatternMatchValue>
```

**via `core::convert::From`**

```rust
fn from(node: &'a PatternMatchClass) -> Self
fn from(node: &'a Pattern) -> Self
fn from(node: &'a PatternMatchOr) -> Self
fn from(node: &'a PatternMatchValue) -> Self
fn from(node: &'a PatternMatchMapping) -> Self
fn from(node: &'a PatternMatchAs) -> Self
fn from(node: &'a PatternMatchSequence) -> Self
fn from(node: &'a PatternMatchStar) -> Self
fn from(node: &'a PatternMatchSingleton) -> Self
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [pattern](https://docs.python.org/3/library/ast.html#ast.pattern)

---

## RootNodeKind

`enum` · `ruff_python_ast::generated::RootNodeKind`

Also reachable as `ruff_python_ast::RootNodeKind`

```rust
enum RootNodeKind
```

**Variants**: `Mod`, `Stmt`, `Expr`, `ExceptHandler`, `InterpolatedStringElement`, `Pattern`, `TypeParam`, `InterpolatedStringFormatSpec`, `PatternArguments`, `PatternKeyword`, `Comprehension`, `Arguments`, `Parameters`, `Parameter`, `ParameterWithDefault`, `Keyword`, `Alias`, `WithItem`, `MatchCase`, `Decorator`, `ElifElseClause`, `TypeParams`, `FString`, `TString`, `StringLiteral`, `BytesLiteral`, `Identifier`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn from_u8(value: u8) -> Option<Self>
```

The unflattened enum or struct type stored by an [`AnyRootNodeRef`].

Unlike [`NodeKind`], this does not distinguish variants of root enums such as [`Stmt`]
and [`Expr`].

---

## Stmt

`enum` · `ruff_python_ast::generated::Stmt`

Also reachable as `pyrefly::query::TypeQueryStmt`, `ruff_python_ast::Stmt`

```rust
enum Stmt
```

**Variants**: `FunctionDef`, `ClassDef`, `Return`, `Delete`, `TypeAlias`, `Assign`, `AugAssign`, `AnnAssign`, `For`, `While`, `If`, `With`, `Match`, `Raise`, `Try`, `Assert`, `Import`, `ImportFrom`, `Global`, `Nonlocal`, `Expr`, `Pass`, `Break`, `Continue`, `IpyEscapeCommand`

**Implements**: `core::convert::From`, `pyrefly_util::display::DisplayWith`, `pyrefly_util::visit::Visit`, `ruff_python_ast::identifier::Identifier`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (125)

```rust
fn ann_assign_stmt(self) -> Option<StmtAnnAssign>
fn as_ann_assign_stmt(&self) -> Option<&StmtAnnAssign>
fn as_ann_assign_stmt_mut(&mut self) -> Option<&mut StmtAnnAssign>
fn as_assert_stmt(&self) -> Option<&StmtAssert>
fn as_assert_stmt_mut(&mut self) -> Option<&mut StmtAssert>
fn as_assign_stmt(&self) -> Option<&StmtAssign>
fn as_assign_stmt_mut(&mut self) -> Option<&mut StmtAssign>
fn as_aug_assign_stmt(&self) -> Option<&StmtAugAssign>
fn as_aug_assign_stmt_mut(&mut self) -> Option<&mut StmtAugAssign>
fn as_break_stmt(&self) -> Option<&StmtBreak>
fn as_break_stmt_mut(&mut self) -> Option<&mut StmtBreak>
fn as_class_def_stmt(&self) -> Option<&StmtClassDef>
fn as_class_def_stmt_mut(&mut self) -> Option<&mut StmtClassDef>
fn as_continue_stmt(&self) -> Option<&StmtContinue>
fn as_continue_stmt_mut(&mut self) -> Option<&mut StmtContinue>
fn as_delete_stmt(&self) -> Option<&StmtDelete>
fn as_delete_stmt_mut(&mut self) -> Option<&mut StmtDelete>
fn as_expr_stmt(&self) -> Option<&StmtExpr>
fn as_expr_stmt_mut(&mut self) -> Option<&mut StmtExpr>
fn as_for_stmt(&self) -> Option<&StmtFor>
fn as_for_stmt_mut(&mut self) -> Option<&mut StmtFor>
fn as_function_def_stmt(&self) -> Option<&StmtFunctionDef>
fn as_function_def_stmt_mut(&mut self) -> Option<&mut StmtFunctionDef>
fn as_global_stmt(&self) -> Option<&StmtGlobal>
fn as_global_stmt_mut(&mut self) -> Option<&mut StmtGlobal>
fn as_if_stmt(&self) -> Option<&StmtIf>
fn as_if_stmt_mut(&mut self) -> Option<&mut StmtIf>
fn as_import_from_stmt(&self) -> Option<&StmtImportFrom>
fn as_import_from_stmt_mut(&mut self) -> Option<&mut StmtImportFrom>
fn as_import_stmt(&self) -> Option<&StmtImport>
fn as_import_stmt_mut(&mut self) -> Option<&mut StmtImport>
fn as_ipy_escape_command_stmt(&self) -> Option<&StmtIpyEscapeCommand>
fn as_ipy_escape_command_stmt_mut(&mut self) -> Option<&mut StmtIpyEscapeCommand>
fn as_match_stmt(&self) -> Option<&StmtMatch>
fn as_match_stmt_mut(&mut self) -> Option<&mut StmtMatch>
fn as_nonlocal_stmt(&self) -> Option<&StmtNonlocal>
fn as_nonlocal_stmt_mut(&mut self) -> Option<&mut StmtNonlocal>
fn as_pass_stmt(&self) -> Option<&StmtPass>
fn as_pass_stmt_mut(&mut self) -> Option<&mut StmtPass>
fn as_raise_stmt(&self) -> Option<&StmtRaise>
fn as_raise_stmt_mut(&mut self) -> Option<&mut StmtRaise>
fn as_return_stmt(&self) -> Option<&StmtReturn>
fn as_return_stmt_mut(&mut self) -> Option<&mut StmtReturn>
fn as_try_stmt(&self) -> Option<&StmtTry>
fn as_try_stmt_mut(&mut self) -> Option<&mut StmtTry>
fn as_type_alias_stmt(&self) -> Option<&StmtTypeAlias>
fn as_type_alias_stmt_mut(&mut self) -> Option<&mut StmtTypeAlias>
fn as_while_stmt(&self) -> Option<&StmtWhile>
fn as_while_stmt_mut(&mut self) -> Option<&mut StmtWhile>
fn as_with_stmt(&self) -> Option<&StmtWith>
fn as_with_stmt_mut(&mut self) -> Option<&mut StmtWith>
fn assert_stmt(self) -> Option<StmtAssert>
fn assign_stmt(self) -> Option<StmtAssign>
fn aug_assign_stmt(self) -> Option<StmtAugAssign>
fn break_stmt(self) -> Option<StmtBreak>
fn class_def_stmt(self) -> Option<StmtClassDef>
fn continue_stmt(self) -> Option<StmtContinue>
fn delete_stmt(self) -> Option<StmtDelete>
fn expect_ann_assign_stmt(self) -> StmtAnnAssign
fn expect_assert_stmt(self) -> StmtAssert
fn expect_assign_stmt(self) -> StmtAssign
fn expect_aug_assign_stmt(self) -> StmtAugAssign
fn expect_break_stmt(self) -> StmtBreak
fn expect_class_def_stmt(self) -> StmtClassDef
fn expect_continue_stmt(self) -> StmtContinue
fn expect_delete_stmt(self) -> StmtDelete
fn expect_expr_stmt(self) -> StmtExpr
fn expect_for_stmt(self) -> StmtFor
fn expect_function_def_stmt(self) -> StmtFunctionDef
fn expect_global_stmt(self) -> StmtGlobal
fn expect_if_stmt(self) -> StmtIf
fn expect_import_from_stmt(self) -> StmtImportFrom
fn expect_import_stmt(self) -> StmtImport
fn expect_ipy_escape_command_stmt(self) -> StmtIpyEscapeCommand
fn expect_match_stmt(self) -> StmtMatch
fn expect_nonlocal_stmt(self) -> StmtNonlocal
fn expect_pass_stmt(self) -> StmtPass
fn expect_raise_stmt(self) -> StmtRaise
fn expect_return_stmt(self) -> StmtReturn
fn expect_try_stmt(self) -> StmtTry
fn expect_type_alias_stmt(self) -> StmtTypeAlias
fn expect_while_stmt(self) -> StmtWhile
fn expect_with_stmt(self) -> StmtWith
fn expr_stmt(self) -> Option<StmtExpr>
fn for_stmt(self) -> Option<StmtFor>
fn function_def_stmt(self) -> Option<StmtFunctionDef>
fn global_stmt(self) -> Option<StmtGlobal>
fn if_stmt(self) -> Option<StmtIf>
fn import_from_stmt(self) -> Option<StmtImportFrom>
fn import_stmt(self) -> Option<StmtImport>
fn ipy_escape_command_stmt(self) -> Option<StmtIpyEscapeCommand>
const fn is_ann_assign_stmt(&self) -> bool
const fn is_assert_stmt(&self) -> bool
const fn is_assign_stmt(&self) -> bool
const fn is_aug_assign_stmt(&self) -> bool
const fn is_break_stmt(&self) -> bool
const fn is_class_def_stmt(&self) -> bool
const fn is_continue_stmt(&self) -> bool
const fn is_delete_stmt(&self) -> bool
const fn is_expr_stmt(&self) -> bool
const fn is_for_stmt(&self) -> bool
const fn is_function_def_stmt(&self) -> bool
const fn is_global_stmt(&self) -> bool
const fn is_if_stmt(&self) -> bool
const fn is_import_from_stmt(&self) -> bool
const fn is_import_stmt(&self) -> bool
const fn is_ipy_escape_command_stmt(&self) -> bool
const fn is_match_stmt(&self) -> bool
const fn is_nonlocal_stmt(&self) -> bool
const fn is_pass_stmt(&self) -> bool
const fn is_raise_stmt(&self) -> bool
const fn is_return_stmt(&self) -> bool
const fn is_try_stmt(&self) -> bool
const fn is_type_alias_stmt(&self) -> bool
const fn is_while_stmt(&self) -> bool
const fn is_with_stmt(&self) -> bool
fn match_stmt(self) -> Option<StmtMatch>
fn nonlocal_stmt(self) -> Option<StmtNonlocal>
fn pass_stmt(self) -> Option<StmtPass>
fn raise_stmt(self) -> Option<StmtRaise>
fn return_stmt(self) -> Option<StmtReturn>
fn try_stmt(self) -> Option<StmtTry>
fn type_alias_stmt(self) -> Option<StmtTypeAlias>
fn while_stmt(self) -> Option<StmtWhile>
fn with_stmt(self) -> Option<StmtWith>
```

**via `core::convert::From`**

```rust
fn from(node: StmtAnnAssign) -> Self
fn from(node: StmtIf) -> Self
fn from(node: StmtRaise) -> Self
fn from(node: StmtImport) -> Self
fn from(node: StmtFunctionDef) -> Self
fn from(node: StmtNonlocal) -> Self
fn from(node: StmtDelete) -> Self
fn from(node: StmtBreak) -> Self
fn from(node: StmtAugAssign) -> Self
fn from(node: StmtWhile) -> Self
fn from(node: StmtMatch) -> Self
fn from(node: StmtAssert) -> Self
fn from(node: StmtGlobal) -> Self
fn from(node: StmtReturn) -> Self
fn from(node: StmtPass) -> Self
fn from(node: StmtAssign) -> Self
fn from(node: StmtIpyEscapeCommand) -> Self
fn from(node: StmtFor) -> Self
fn from(node: StmtWith) -> Self
fn from(node: StmtTry) -> Self
fn from(node: StmtImportFrom) -> Self
fn from(node: StmtClassDef) -> Self
fn from(node: StmtExpr) -> Self
fn from(node: StmtTypeAlias) -> Self
fn from(node: StmtContinue) -> Self
```

**via `ruff_python_ast::identifier::Identifier`**

```rust
fn identifier(&self) -> TextRange
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [stmt](https://docs.python.org/3/library/ast.html#ast.stmt)

---

## StmtRef

`enum` · `ruff_python_ast::generated::StmtRef`

Also reachable as `ruff_python_ast::StmtRef`

```rust
enum StmtRef<'a>
```

**Variants**: `FunctionDef`, `ClassDef`, `Return`, `Delete`, `TypeAlias`, `Assign`, `AugAssign`, `AnnAssign`, `For`, `While`, `If`, `With`, `Match`, `Raise`, `Try`, `Assert`, `Import`, `ImportFrom`, `Global`, `Nonlocal`, `Expr`, `Pass`, `Break`, `Continue`, `IpyEscapeCommand`

**Implements**: `core::convert::From`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (125)

```rust
fn ann_assign_stmt(self) -> Option<&'a StmtAnnAssign>
fn as_ann_assign_stmt(&self) -> Option<&&'a StmtAnnAssign>
fn as_assert_stmt(&self) -> Option<&&'a StmtAssert>
fn as_assign_stmt(&self) -> Option<&&'a StmtAssign>
fn as_aug_assign_stmt(&self) -> Option<&&'a StmtAugAssign>
fn as_break_stmt(&self) -> Option<&&'a StmtBreak>
fn as_class_def_stmt(&self) -> Option<&&'a StmtClassDef>
fn as_continue_stmt(&self) -> Option<&&'a StmtContinue>
fn as_delete_stmt(&self) -> Option<&&'a StmtDelete>
fn as_expr_stmt(&self) -> Option<&&'a StmtExpr>
fn as_for_stmt(&self) -> Option<&&'a StmtFor>
fn as_function_def_stmt(&self) -> Option<&&'a StmtFunctionDef>
fn as_global_stmt(&self) -> Option<&&'a StmtGlobal>
fn as_if_stmt(&self) -> Option<&&'a StmtIf>
fn as_import_from_stmt(&self) -> Option<&&'a StmtImportFrom>
fn as_import_stmt(&self) -> Option<&&'a StmtImport>
fn as_ipy_escape_command_stmt(&self) -> Option<&&'a StmtIpyEscapeCommand>
fn as_match_stmt(&self) -> Option<&&'a StmtMatch>
fn as_mut_ann_assign_stmt(&mut self) -> Option<&mut &'a StmtAnnAssign>
fn as_mut_assert_stmt(&mut self) -> Option<&mut &'a StmtAssert>
fn as_mut_assign_stmt(&mut self) -> Option<&mut &'a StmtAssign>
fn as_mut_aug_assign_stmt(&mut self) -> Option<&mut &'a StmtAugAssign>
fn as_mut_break_stmt(&mut self) -> Option<&mut &'a StmtBreak>
fn as_mut_class_def_stmt(&mut self) -> Option<&mut &'a StmtClassDef>
fn as_mut_continue_stmt(&mut self) -> Option<&mut &'a StmtContinue>
fn as_mut_delete_stmt(&mut self) -> Option<&mut &'a StmtDelete>
fn as_mut_expr_stmt(&mut self) -> Option<&mut &'a StmtExpr>
fn as_mut_for_stmt(&mut self) -> Option<&mut &'a StmtFor>
fn as_mut_function_def_stmt(&mut self) -> Option<&mut &'a StmtFunctionDef>
fn as_mut_global_stmt(&mut self) -> Option<&mut &'a StmtGlobal>
fn as_mut_if_stmt(&mut self) -> Option<&mut &'a StmtIf>
fn as_mut_import_from_stmt(&mut self) -> Option<&mut &'a StmtImportFrom>
fn as_mut_import_stmt(&mut self) -> Option<&mut &'a StmtImport>
fn as_mut_ipy_escape_command_stmt(&mut self) -> Option<&mut &'a StmtIpyEscapeCommand>
fn as_mut_match_stmt(&mut self) -> Option<&mut &'a StmtMatch>
fn as_mut_nonlocal_stmt(&mut self) -> Option<&mut &'a StmtNonlocal>
fn as_mut_pass_stmt(&mut self) -> Option<&mut &'a StmtPass>
fn as_mut_raise_stmt(&mut self) -> Option<&mut &'a StmtRaise>
fn as_mut_return_stmt(&mut self) -> Option<&mut &'a StmtReturn>
fn as_mut_try_stmt(&mut self) -> Option<&mut &'a StmtTry>
fn as_mut_type_alias_stmt(&mut self) -> Option<&mut &'a StmtTypeAlias>
fn as_mut_while_stmt(&mut self) -> Option<&mut &'a StmtWhile>
fn as_mut_with_stmt(&mut self) -> Option<&mut &'a StmtWith>
fn as_nonlocal_stmt(&self) -> Option<&&'a StmtNonlocal>
fn as_pass_stmt(&self) -> Option<&&'a StmtPass>
fn as_raise_stmt(&self) -> Option<&&'a StmtRaise>
fn as_return_stmt(&self) -> Option<&&'a StmtReturn>
fn as_try_stmt(&self) -> Option<&&'a StmtTry>
fn as_type_alias_stmt(&self) -> Option<&&'a StmtTypeAlias>
fn as_while_stmt(&self) -> Option<&&'a StmtWhile>
fn as_with_stmt(&self) -> Option<&&'a StmtWith>
fn assert_stmt(self) -> Option<&'a StmtAssert>
fn assign_stmt(self) -> Option<&'a StmtAssign>
fn aug_assign_stmt(self) -> Option<&'a StmtAugAssign>
fn break_stmt(self) -> Option<&'a StmtBreak>
fn class_def_stmt(self) -> Option<&'a StmtClassDef>
fn continue_stmt(self) -> Option<&'a StmtContinue>
fn delete_stmt(self) -> Option<&'a StmtDelete>
fn expect_ann_assign_stmt(self) -> &'a StmtAnnAssign where Self: ::std::fmt::Debug
fn expect_assert_stmt(self) -> &'a StmtAssert where Self: ::std::fmt::Debug
fn expect_assign_stmt(self) -> &'a StmtAssign where Self: ::std::fmt::Debug
fn expect_aug_assign_stmt(self) -> &'a StmtAugAssign where Self: ::std::fmt::Debug
fn expect_break_stmt(self) -> &'a StmtBreak where Self: ::std::fmt::Debug
fn expect_class_def_stmt(self) -> &'a StmtClassDef where Self: ::std::fmt::Debug
fn expect_continue_stmt(self) -> &'a StmtContinue where Self: ::std::fmt::Debug
fn expect_delete_stmt(self) -> &'a StmtDelete where Self: ::std::fmt::Debug
fn expect_expr_stmt(self) -> &'a StmtExpr where Self: ::std::fmt::Debug
fn expect_for_stmt(self) -> &'a StmtFor where Self: ::std::fmt::Debug
fn expect_function_def_stmt(self) -> &'a StmtFunctionDef where Self: ::std::fmt::Debug
fn expect_global_stmt(self) -> &'a StmtGlobal where Self: ::std::fmt::Debug
fn expect_if_stmt(self) -> &'a StmtIf where Self: ::std::fmt::Debug
fn expect_import_from_stmt(self) -> &'a StmtImportFrom where Self: ::std::fmt::Debug
fn expect_import_stmt(self) -> &'a StmtImport where Self: ::std::fmt::Debug
fn expect_ipy_escape_command_stmt(self) -> &'a StmtIpyEscapeCommand where Self: ::std::fmt::Debug
fn expect_match_stmt(self) -> &'a StmtMatch where Self: ::std::fmt::Debug
fn expect_nonlocal_stmt(self) -> &'a StmtNonlocal where Self: ::std::fmt::Debug
fn expect_pass_stmt(self) -> &'a StmtPass where Self: ::std::fmt::Debug
fn expect_raise_stmt(self) -> &'a StmtRaise where Self: ::std::fmt::Debug
fn expect_return_stmt(self) -> &'a StmtReturn where Self: ::std::fmt::Debug
fn expect_try_stmt(self) -> &'a StmtTry where Self: ::std::fmt::Debug
fn expect_type_alias_stmt(self) -> &'a StmtTypeAlias where Self: ::std::fmt::Debug
fn expect_while_stmt(self) -> &'a StmtWhile where Self: ::std::fmt::Debug
fn expect_with_stmt(self) -> &'a StmtWith where Self: ::std::fmt::Debug
fn expr_stmt(self) -> Option<&'a StmtExpr>
fn for_stmt(self) -> Option<&'a StmtFor>
fn function_def_stmt(self) -> Option<&'a StmtFunctionDef>
fn global_stmt(self) -> Option<&'a StmtGlobal>
fn if_stmt(self) -> Option<&'a StmtIf>
fn import_from_stmt(self) -> Option<&'a StmtImportFrom>
fn import_stmt(self) -> Option<&'a StmtImport>
fn ipy_escape_command_stmt(self) -> Option<&'a StmtIpyEscapeCommand>
const fn is_ann_assign_stmt(&self) -> bool
const fn is_assert_stmt(&self) -> bool
const fn is_assign_stmt(&self) -> bool
const fn is_aug_assign_stmt(&self) -> bool
const fn is_break_stmt(&self) -> bool
const fn is_class_def_stmt(&self) -> bool
const fn is_continue_stmt(&self) -> bool
const fn is_delete_stmt(&self) -> bool
const fn is_expr_stmt(&self) -> bool
const fn is_for_stmt(&self) -> bool
const fn is_function_def_stmt(&self) -> bool
const fn is_global_stmt(&self) -> bool
const fn is_if_stmt(&self) -> bool
const fn is_import_from_stmt(&self) -> bool
const fn is_import_stmt(&self) -> bool
const fn is_ipy_escape_command_stmt(&self) -> bool
const fn is_match_stmt(&self) -> bool
const fn is_nonlocal_stmt(&self) -> bool
const fn is_pass_stmt(&self) -> bool
const fn is_raise_stmt(&self) -> bool
const fn is_return_stmt(&self) -> bool
const fn is_try_stmt(&self) -> bool
const fn is_type_alias_stmt(&self) -> bool
const fn is_while_stmt(&self) -> bool
const fn is_with_stmt(&self) -> bool
fn match_stmt(self) -> Option<&'a StmtMatch>
fn nonlocal_stmt(self) -> Option<&'a StmtNonlocal>
fn pass_stmt(self) -> Option<&'a StmtPass>
fn raise_stmt(self) -> Option<&'a StmtRaise>
fn return_stmt(self) -> Option<&'a StmtReturn>
fn try_stmt(self) -> Option<&'a StmtTry>
fn type_alias_stmt(self) -> Option<&'a StmtTypeAlias>
fn while_stmt(self) -> Option<&'a StmtWhile>
fn with_stmt(self) -> Option<&'a StmtWith>
```

**via `core::convert::From`**

```rust
fn from(node: &'a StmtTry) -> Self
fn from(node: &'a StmtImportFrom) -> Self
fn from(node: &'a StmtClassDef) -> Self
fn from(node: &'a StmtExpr) -> Self
fn from(node: &'a StmtTypeAlias) -> Self
fn from(node: &'a StmtContinue) -> Self
fn from(node: &'a StmtAnnAssign) -> Self
fn from(node: &'a StmtIf) -> Self
fn from(node: &'a StmtRaise) -> Self
fn from(node: &'a StmtImport) -> Self
fn from(node: &'a StmtFunctionDef) -> Self
fn from(node: &'a StmtNonlocal) -> Self
fn from(node: &'a StmtDelete) -> Self
fn from(node: &'a StmtBreak) -> Self
fn from(node: &'a StmtAugAssign) -> Self
fn from(node: &'a StmtWhile) -> Self
fn from(node: &'a StmtMatch) -> Self
fn from(node: &'a StmtAssert) -> Self
fn from(node: &'a Stmt) -> Self
fn from(node: &'a StmtGlobal) -> Self
fn from(node: &'a StmtReturn) -> Self
fn from(node: &'a StmtPass) -> Self
fn from(node: &'a StmtAssign) -> Self
fn from(node: &'a StmtIpyEscapeCommand) -> Self
fn from(node: &'a StmtFor) -> Self
fn from(node: &'a StmtWith) -> Self
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [stmt](https://docs.python.org/3/library/ast.html#ast.stmt)

---

## TypeParam

`enum` · `ruff_python_ast::generated::TypeParam`

Also reachable as `ruff_python_ast::TypeParam`

```rust
enum TypeParam
```

**Variants**: `TypeVar`, `TypeVarTuple`, `ParamSpec`

**Implements**: `core::convert::From`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (17)

```rust
fn as_param_spec(&self) -> Option<&TypeParamParamSpec>
fn as_param_spec_mut(&mut self) -> Option<&mut TypeParamParamSpec>
fn as_type_var(&self) -> Option<&TypeParamTypeVar>
fn as_type_var_mut(&mut self) -> Option<&mut TypeParamTypeVar>
fn as_type_var_tuple(&self) -> Option<&TypeParamTypeVarTuple>
fn as_type_var_tuple_mut(&mut self) -> Option<&mut TypeParamTypeVarTuple>
fn default(&self) -> Option<&Expr>
fn expect_param_spec(self) -> TypeParamParamSpec
fn expect_type_var(self) -> TypeParamTypeVar
fn expect_type_var_tuple(self) -> TypeParamTypeVarTuple
const fn is_param_spec(&self) -> bool
const fn is_type_var(&self) -> bool
const fn is_type_var_tuple(&self) -> bool
const fn name(&self) -> &Identifier
fn param_spec(self) -> Option<TypeParamParamSpec>
fn type_var(self) -> Option<TypeParamTypeVar>
fn type_var_tuple(self) -> Option<TypeParamTypeVarTuple>
```

**via `core::convert::From`**

```rust
fn from(node: TypeParamTypeVarTuple) -> Self
fn from(node: TypeParamTypeVar) -> Self
fn from(node: TypeParamParamSpec) -> Self
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [type_param](https://docs.python.org/3/library/ast.html#ast.type_param)

---

## TypeParamRef

`enum` · `ruff_python_ast::generated::TypeParamRef`

Also reachable as `ruff_python_ast::TypeParamRef`

```rust
enum TypeParamRef<'a>
```

**Variants**: `TypeVar`, `TypeVarTuple`, `ParamSpec`

**Implements**: `core::convert::From`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (15)

```rust
fn as_mut_param_spec(&mut self) -> Option<&mut &'a TypeParamParamSpec>
fn as_mut_type_var(&mut self) -> Option<&mut &'a TypeParamTypeVar>
fn as_mut_type_var_tuple(&mut self) -> Option<&mut &'a TypeParamTypeVarTuple>
fn as_param_spec(&self) -> Option<&&'a TypeParamParamSpec>
fn as_type_var(&self) -> Option<&&'a TypeParamTypeVar>
fn as_type_var_tuple(&self) -> Option<&&'a TypeParamTypeVarTuple>
fn expect_param_spec(self) -> &'a TypeParamParamSpec where Self: ::std::fmt::Debug
fn expect_type_var(self) -> &'a TypeParamTypeVar where Self: ::std::fmt::Debug
fn expect_type_var_tuple(self) -> &'a TypeParamTypeVarTuple where Self: ::std::fmt::Debug
const fn is_param_spec(&self) -> bool
const fn is_type_var(&self) -> bool
const fn is_type_var_tuple(&self) -> bool
fn param_spec(self) -> Option<&'a TypeParamParamSpec>
fn type_var(self) -> Option<&'a TypeParamTypeVar>
fn type_var_tuple(self) -> Option<&'a TypeParamTypeVarTuple>
```

**via `core::convert::From`**

```rust
fn from(node: &'a TypeParam) -> Self
fn from(node: &'a TypeParamParamSpec) -> Self
fn from(node: &'a TypeParamTypeVarTuple) -> Self
fn from(node: &'a TypeParamTypeVar) -> Self
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [type_param](https://docs.python.org/3/library/ast.html#ast.type_param)

---

## ExprAttribute

`struct` · `ruff_python_ast::generated::ExprAttribute`

Also reachable as `ruff_python_ast::ExprAttribute`

```rust
struct ExprAttribute
```

**Fields**: `node_index`, `range`, `value`, `attr`, `ctx`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Attribute](https://docs.python.org/3/library/ast.html#ast.Attribute)

---

## ExprAwait

`struct` · `ruff_python_ast::generated::ExprAwait`

Also reachable as `ruff_python_ast::ExprAwait`

```rust
struct ExprAwait
```

**Fields**: `node_index`, `range`, `value`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Await](https://docs.python.org/3/library/ast.html#ast.Await)

---

## ExprBinOp

`struct` · `ruff_python_ast::generated::ExprBinOp`

Also reachable as `ruff_python_ast::ExprBinOp`

```rust
struct ExprBinOp
```

**Fields**: `node_index`, `range`, `left`, `op`, `right`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [BinOp](https://docs.python.org/3/library/ast.html#ast.BinOp)

---

## ExprBoolOp

`struct` · `ruff_python_ast::generated::ExprBoolOp`

Also reachable as `ruff_python_ast::ExprBoolOp`

```rust
struct ExprBoolOp
```

**Fields**: `node_index`, `range`, `op`, `values`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [BoolOp](https://docs.python.org/3/library/ast.html#ast.BoolOp)

---

## ExprBooleanLiteral

`struct` · `ruff_python_ast::generated::ExprBooleanLiteral`

Also reachable as `ruff_python_ast::ExprBooleanLiteral`

```rust
struct ExprBooleanLiteral
```

**Fields**: `node_index`, `range`, `value`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

---

## ExprBytesLiteral

`struct` · `ruff_python_ast::generated::ExprBytesLiteral`

Also reachable as `ruff_python_ast::ExprBytesLiteral`

```rust
struct ExprBytesLiteral
```

**Fields**: `node_index`, `range`, `value`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn as_single_part_bytestring(&self) -> Option<&BytesLiteral>
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An AST node that represents either a single-part bytestring literal
or an implicitly concatenated bytestring literal.

---

## ExprCall

`struct` · `ruff_python_ast::generated::ExprCall`

Also reachable as `ruff_python_ast::ExprCall`

```rust
struct ExprCall
```

**Fields**: `node_index`, `range_start`, `func`, `arguments`

**Implements**: `pyrefly_util::display::DisplayWith`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> TextRange
```

A call expression whose end offset is derived from its arguments.

The parser and error-recovery code must ensure that the call and its arguments
end at the same offset.

See also [Call](https://docs.python.org/3/library/ast.html#ast.Call)

---

## ExprCompare

`struct` · `ruff_python_ast::generated::ExprCompare`

Also reachable as `ruff_python_ast::ExprCompare`

```rust
struct ExprCompare
```

**Fields**: `node_index`, `range`, `left`, `ops`, `comparators`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Compare](https://docs.python.org/3/library/ast.html#ast.Compare)

---

## ExprDict

`struct` · `ruff_python_ast::generated::ExprDict`

Also reachable as `ruff_python_ast::ExprDict`

```rust
struct ExprDict
```

**Fields**: `node_index`, `range`, `items`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (7)

```rust
fn is_empty(&self) -> bool
fn iter(&self) -> std::slice::Iter<'_, DictItem>
fn iter_keys(&self) -> DictKeyIterator<'_>
fn iter_values(&self) -> DictValueIterator<'_>
fn key(&self, n: usize) -> Option<&Expr>
fn len(&self) -> usize
fn value(&self, n: usize) -> &Expr
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Dict](https://docs.python.org/3/library/ast.html#ast.Dict)

---

## ExprDictComp

`struct` · `ruff_python_ast::generated::ExprDictComp`

Also reachable as `ruff_python_ast::ExprDictComp`

```rust
struct ExprDictComp
```

**Fields**: `node_index`, `range`, `key`, `value`, `generators`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [DictComp](https://docs.python.org/3/library/ast.html#ast.DictComp)

---

## ExprEllipsisLiteral

`struct` · `ruff_python_ast::generated::ExprEllipsisLiteral`

Also reachable as `ruff_python_ast::ExprEllipsisLiteral`

```rust
struct ExprEllipsisLiteral
```

**Fields**: `node_index`, `range`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

---

## ExprFString

`struct` · `ruff_python_ast::generated::ExprFString`

Also reachable as `ruff_python_ast::ExprFString`

```rust
struct ExprFString
```

**Fields**: `node_index`, `range`, `value`

**Implements**: `pyrefly_util::visit::Visit`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn as_single_part_fstring(&self) -> Option<&FString>
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An AST node that represents either a single-part f-string literal
or an implicitly concatenated f-string literal.

This type differs from the original Python AST `JoinedStr` in that it
doesn't join the implicitly concatenated parts into a single string. Instead,
it keeps them separate and provide various methods to access the parts.

See also [JoinedStr](https://docs.python.org/3/library/ast.html#ast.JoinedStr)

---

## ExprGenerator

`struct` · `ruff_python_ast::generated::ExprGenerator`

Also reachable as `ruff_python_ast::ExprGenerator`

```rust
struct ExprGenerator
```

**Fields**: `node_index`, `range`, `elt`, `generators`, `parenthesized`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [GeneratorExp](https://docs.python.org/3/library/ast.html#ast.GeneratorExp)

---

## ExprIf

`struct` · `ruff_python_ast::generated::ExprIf`

Also reachable as `ruff_python_ast::ExprIf`

```rust
struct ExprIf
```

**Fields**: `node_index`, `range`, `test`, `body`, `orelse`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [IfExp](https://docs.python.org/3/library/ast.html#ast.IfExp)

---

## ExprIpyEscapeCommand

`struct` · `ruff_python_ast::generated::ExprIpyEscapeCommand`

Also reachable as `ruff_python_ast::ExprIpyEscapeCommand`

```rust
struct ExprIpyEscapeCommand
```

**Fields**: `node_index`, `range`, `kind`, `value`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An AST node used to represent a IPython escape command at the expression level.

For example,
```python
dir = !pwd
```

Here, the escape kind can only be `!` or `%` otherwise it is a syntax error.

For more information related to terminology and syntax of escape commands,
see [`StmtIpyEscapeCommand`].

---

## ExprLambda

`struct` · `ruff_python_ast::generated::ExprLambda`

Also reachable as `ruff_python_ast::ExprLambda`

```rust
struct ExprLambda
```

**Fields**: `node_index`, `range`, `parameters`, `body`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Lambda](https://docs.python.org/3/library/ast.html#ast.Lambda)

---

## ExprList

`struct` · `ruff_python_ast::generated::ExprList`

Also reachable as `ruff_python_ast::ExprList`

```rust
struct ExprList
```

**Fields**: `node_index`, `range`, `elts`, `ctx`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn is_empty(&self) -> bool
fn iter(&self) -> std::slice::Iter<'_, Expr>
fn len(&self) -> usize
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [List](https://docs.python.org/3/library/ast.html#ast.List)

---

## ExprListComp

`struct` · `ruff_python_ast::generated::ExprListComp`

Also reachable as `ruff_python_ast::ExprListComp`

```rust
struct ExprListComp
```

**Fields**: `node_index`, `range`, `elt`, `generators`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [ListComp](https://docs.python.org/3/library/ast.html#ast.ListComp)

---

## ExprName

`struct` · `ruff_python_ast::generated::ExprName`

Also reachable as `ruff_python_ast::ExprName`

```rust
struct ExprName
```

**Fields**: `node_index`, `range`, `id`, `ctx`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn id(&self) -> &Name
const fn is_invalid(&self) -> bool
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Name](https://docs.python.org/3/library/ast.html#ast.Name)

---

## ExprNamed

`struct` · `ruff_python_ast::generated::ExprNamed`

Also reachable as `ruff_python_ast::ExprNamed`

```rust
struct ExprNamed
```

**Fields**: `node_index`, `range`, `target`, `value`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [NamedExpr](https://docs.python.org/3/library/ast.html#ast.NamedExpr)

---

## ExprNoneLiteral

`struct` · `ruff_python_ast::generated::ExprNoneLiteral`

Also reachable as `ruff_python_ast::ExprNoneLiteral`

```rust
struct ExprNoneLiteral
```

**Fields**: `node_index`, `range`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

---

## ExprNumberLiteral

`struct` · `ruff_python_ast::generated::ExprNumberLiteral`

Also reachable as `ruff_python_ast::ExprNumberLiteral`

```rust
struct ExprNumberLiteral
```

**Fields**: `node_index`, `range`, `value`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

---

## ExprSet

`struct` · `ruff_python_ast::generated::ExprSet`

Also reachable as `ruff_python_ast::ExprSet`

```rust
struct ExprSet
```

**Fields**: `node_index`, `range`, `elts`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn is_empty(&self) -> bool
fn iter(&self) -> std::slice::Iter<'_, Expr>
fn len(&self) -> usize
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Set](https://docs.python.org/3/library/ast.html#ast.Set)

---

## ExprSetComp

`struct` · `ruff_python_ast::generated::ExprSetComp`

Also reachable as `ruff_python_ast::ExprSetComp`

```rust
struct ExprSetComp
```

**Fields**: `node_index`, `range`, `elt`, `generators`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [SetComp](https://docs.python.org/3/library/ast.html#ast.SetComp)

---

## ExprSlice

`struct` · `ruff_python_ast::generated::ExprSlice`

Also reachable as `ruff_python_ast::ExprSlice`

```rust
struct ExprSlice
```

**Fields**: `node_index`, `range`, `lower`, `upper`, `step`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Slice](https://docs.python.org/3/library/ast.html#ast.Slice)

---

## ExprStarred

`struct` · `ruff_python_ast::generated::ExprStarred`

Also reachable as `ruff_python_ast::ExprStarred`

```rust
struct ExprStarred
```

**Fields**: `node_index`, `range`, `value`, `ctx`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Starred](https://docs.python.org/3/library/ast.html#ast.Starred)

---

## ExprStringLiteral

`struct` · `ruff_python_ast::generated::ExprStringLiteral`

Also reachable as `ruff_python_ast::ExprStringLiteral`

```rust
struct ExprStringLiteral
```

**Fields**: `node_index`, `range`, `value`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn as_single_part_string(&self) -> Option<&StringLiteral>
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An AST node that represents either a single-part string literal
or an implicitly concatenated string literal.

---

## ExprSubscript

`struct` · `ruff_python_ast::generated::ExprSubscript`

Also reachable as `ruff_python_ast::ExprSubscript`

```rust
struct ExprSubscript
```

**Fields**: `node_index`, `range`, `value`, `slice`, `ctx`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Subscript](https://docs.python.org/3/library/ast.html#ast.Subscript)

---

## ExprTString

`struct` · `ruff_python_ast::generated::ExprTString`

Also reachable as `ruff_python_ast::ExprTString`

```rust
struct ExprTString
```

**Fields**: `node_index`, `range`, `value`

**Implements**: `pyrefly_util::visit::Visit`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
const fn as_single_part_tstring(&self) -> Option<&TString>
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An AST node that represents either a single-part t-string literal
or an implicitly concatenated t-string literal.

This type differs from the original Python AST `TemplateStr` in that it
doesn't join the implicitly concatenated parts into a single string. Instead,
it keeps them separate and provide various methods to access the parts.

See also [TemplateStr](https://docs.python.org/3/library/ast.html#ast.TemplateStr)

---

## ExprTuple

`struct` · `ruff_python_ast::generated::ExprTuple`

Also reachable as `ruff_python_ast::ExprTuple`

```rust
struct ExprTuple
```

**Fields**: `node_index`, `range`, `elts`, `ctx`, `parenthesized`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn is_empty(&self) -> bool
fn iter(&self) -> std::slice::Iter<'_, Expr>
fn len(&self) -> usize
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Tuple](https://docs.python.org/3/library/ast.html#ast.Tuple)

---

## ExprUnaryOp

`struct` · `ruff_python_ast::generated::ExprUnaryOp`

Also reachable as `ruff_python_ast::ExprUnaryOp`

```rust
struct ExprUnaryOp
```

**Fields**: `node_index`, `range`, `op`, `operand`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [UnaryOp](https://docs.python.org/3/library/ast.html#ast.UnaryOp)

---

## ExprYield

`struct` · `ruff_python_ast::generated::ExprYield`

Also reachable as `ruff_python_ast::ExprYield`

```rust
struct ExprYield
```

**Fields**: `node_index`, `range`, `value`

**Implements**: `pyrefly_util::display::DisplayWith`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Yield](https://docs.python.org/3/library/ast.html#ast.Yield)

---

## ExprYieldFrom

`struct` · `ruff_python_ast::generated::ExprYieldFrom`

Also reachable as `ruff_python_ast::ExprYieldFrom`

```rust
struct ExprYieldFrom
```

**Fields**: `node_index`, `range`, `value`

**Implements**: `pyrefly_util::display::DisplayWith`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [YieldFrom](https://docs.python.org/3/library/ast.html#ast.YieldFrom)

---

## ModExpression

`struct` · `ruff_python_ast::generated::ModExpression`

Also reachable as `ruff_python_ast::ModExpression`

```rust
struct ModExpression
```

**Fields**: `node_index`, `range`, `body`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Module](https://docs.python.org/3/library/ast.html#ast.Module)

---

## ModModule

`struct` · `ruff_python_ast::generated::ModModule`

Also reachable as `ruff_python_ast::ModModule`

```rust
struct ModModule
```

**Fields**: `node_index`, `range`, `body`

**Implements**: `pyrefly_util::visit::Visit`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Module](https://docs.python.org/3/library/ast.html#ast.Module)

---

## PatternMatchAs

`struct` · `ruff_python_ast::generated::PatternMatchAs`

Also reachable as `ruff_python_ast::PatternMatchAs`

```rust
struct PatternMatchAs
```

**Fields**: `node_index`, `range`, `pattern`, `name`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [MatchAs](https://docs.python.org/3/library/ast.html#ast.MatchAs)

---

## PatternMatchClass

`struct` · `ruff_python_ast::generated::PatternMatchClass`

Also reachable as `ruff_python_ast::PatternMatchClass`

```rust
struct PatternMatchClass
```

**Fields**: `node_index`, `range`, `cls`, `arguments`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [MatchClass](https://docs.python.org/3/library/ast.html#ast.MatchClass)

---

## PatternMatchMapping

`struct` · `ruff_python_ast::generated::PatternMatchMapping`

Also reachable as `ruff_python_ast::PatternMatchMapping`

```rust
struct PatternMatchMapping
```

**Fields**: `node_index`, `range`, `keys`, `patterns`, `rest`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [MatchMapping](https://docs.python.org/3/library/ast.html#ast.MatchMapping)

---

## PatternMatchOr

`struct` · `ruff_python_ast::generated::PatternMatchOr`

Also reachable as `ruff_python_ast::PatternMatchOr`

```rust
struct PatternMatchOr
```

**Fields**: `node_index`, `range`, `patterns`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [MatchOr](https://docs.python.org/3/library/ast.html#ast.MatchOr)

---

## PatternMatchSequence

`struct` · `ruff_python_ast::generated::PatternMatchSequence`

Also reachable as `ruff_python_ast::PatternMatchSequence`

```rust
struct PatternMatchSequence
```

**Fields**: `node_index`, `range`, `patterns`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [MatchSequence](https://docs.python.org/3/library/ast.html#ast.MatchSequence)

---

## PatternMatchSingleton

`struct` · `ruff_python_ast::generated::PatternMatchSingleton`

Also reachable as `ruff_python_ast::PatternMatchSingleton`

```rust
struct PatternMatchSingleton
```

**Fields**: `node_index`, `range`, `value`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [MatchSingleton](https://docs.python.org/3/library/ast.html#ast.MatchSingleton)

---

## PatternMatchStar

`struct` · `ruff_python_ast::generated::PatternMatchStar`

Also reachable as `ruff_python_ast::PatternMatchStar`

```rust
struct PatternMatchStar
```

**Fields**: `node_index`, `range`, `name`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [MatchStar](https://docs.python.org/3/library/ast.html#ast.MatchStar)

---

## PatternMatchValue

`struct` · `ruff_python_ast::generated::PatternMatchValue`

Also reachable as `ruff_python_ast::PatternMatchValue`

```rust
struct PatternMatchValue
```

**Fields**: `node_index`, `range`, `value`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [MatchValue](https://docs.python.org/3/library/ast.html#ast.MatchValue)

---

## StmtAnnAssign

`struct` · `ruff_python_ast::generated::StmtAnnAssign`

Also reachable as `ruff_python_ast::StmtAnnAssign`

```rust
struct StmtAnnAssign
```

**Fields**: `node_index`, `range`, `target`, `annotation`, `value`, `simple`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [AnnAssign](https://docs.python.org/3/library/ast.html#ast.AnnAssign)

---

## StmtAssert

`struct` · `ruff_python_ast::generated::StmtAssert`

Also reachable as `ruff_python_ast::StmtAssert`

```rust
struct StmtAssert
```

**Fields**: `node_index`, `range`, `test`, `msg`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Assert](https://docs.python.org/3/library/ast.html#ast.Assert)

---

## StmtAssign

`struct` · `ruff_python_ast::generated::StmtAssign`

Also reachable as `ruff_python_ast::StmtAssign`

```rust
struct StmtAssign
```

**Fields**: `node_index`, `range`, `targets`, `value`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Assign](https://docs.python.org/3/library/ast.html#ast.Assign)

---

## StmtAugAssign

`struct` · `ruff_python_ast::generated::StmtAugAssign`

Also reachable as `ruff_python_ast::StmtAugAssign`

```rust
struct StmtAugAssign
```

**Fields**: `node_index`, `range`, `target`, `op`, `value`

**Implements**: `pyrefly_util::display::DisplayWith`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [AugAssign](https://docs.python.org/3/library/ast.html#ast.AugAssign)

---

## StmtBreak

`struct` · `ruff_python_ast::generated::StmtBreak`

Also reachable as `ruff_python_ast::StmtBreak`

```rust
struct StmtBreak
```

**Fields**: `node_index`, `range`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Break](https://docs.python.org/3/library/ast.html#ast.Break)

---

## StmtClassDef

`struct` · `ruff_python_ast::generated::StmtClassDef`

Also reachable as `pyrefly::query::TypeQueryStmtClassDef`, `ruff_python_ast::StmtClassDef`

```rust
struct StmtClassDef
```

**Fields**: `node_index`, `range`, `decorator_list`, `name`, `type_params`, `arguments`, `body`

**Implements**: `ruff_python_ast::identifier::Identifier`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn bases(&self) -> &[Expr]
fn keywords(&self) -> &[Keyword]
```

**via `ruff_python_ast::identifier::Identifier`**

```rust
fn identifier(&self) -> TextRange
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [ClassDef](https://docs.python.org/3/library/ast.html#ast.ClassDef)

---

## StmtContinue

`struct` · `ruff_python_ast::generated::StmtContinue`

Also reachable as `ruff_python_ast::StmtContinue`

```rust
struct StmtContinue
```

**Fields**: `node_index`, `range`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Continue](https://docs.python.org/3/library/ast.html#ast.Continue)

---

## StmtDelete

`struct` · `ruff_python_ast::generated::StmtDelete`

Also reachable as `ruff_python_ast::StmtDelete`

```rust
struct StmtDelete
```

**Fields**: `node_index`, `range`, `targets`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Delete](https://docs.python.org/3/library/ast.html#ast.Delete)

---

## StmtExpr

`struct` · `ruff_python_ast::generated::StmtExpr`

Also reachable as `ruff_python_ast::StmtExpr`

```rust
struct StmtExpr
```

**Fields**: `node_index`, `range`, `value`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Expr](https://docs.python.org/3/library/ast.html#ast.Expr)

---

## StmtFor

`struct` · `ruff_python_ast::generated::StmtFor`

Also reachable as `ruff_python_ast::StmtFor`

```rust
struct StmtFor
```

**Fields**: `node_index`, `range`, `is_async`, `target`, `iter`, `body`, `orelse`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [For](https://docs.python.org/3/library/ast.html#ast.For)
and [AsyncFor](https://docs.python.org/3/library/ast.html#ast.AsyncFor).

This type differs from the original Python AST, as it collapses the synchronous and asynchronous variants into a single type.

---

## StmtFunctionDef

`struct` · `ruff_python_ast::generated::StmtFunctionDef`

Also reachable as `pyrefly::query::TypeQueryStmtFunctionDef`, `ruff_python_ast::StmtFunctionDef`

```rust
struct StmtFunctionDef
```

**Fields**: `node_index`, `range`, `is_async`, `decorator_list`, `name`, `type_params`, `parameters`, `returns`, `body`

**Implements**: `ruff_python_ast::identifier::Identifier`, `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::identifier::Identifier`**

```rust
fn identifier(&self) -> TextRange
```

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [FunctionDef](https://docs.python.org/3/library/ast.html#ast.FunctionDef)
and [AsyncFunctionDef](https://docs.python.org/3/library/ast.html#ast.AsyncFunctionDef).

This type differs from the original Python AST, as it collapses the synchronous and asynchronous variants into a single type.

---

## StmtGlobal

`struct` · `ruff_python_ast::generated::StmtGlobal`

Also reachable as `ruff_python_ast::StmtGlobal`

```rust
struct StmtGlobal
```

**Fields**: `node_index`, `range`, `names`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Global](https://docs.python.org/3/library/ast.html#ast.Global)

---

## StmtIf

`struct` · `ruff_python_ast::generated::StmtIf`

Also reachable as `ruff_python_ast::StmtIf`

```rust
struct StmtIf
```

**Fields**: `node_index`, `range`, `test`, `body`, `elif_else_clauses`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [If](https://docs.python.org/3/library/ast.html#ast.If)

---

## StmtImport

`struct` · `ruff_python_ast::generated::StmtImport`

Also reachable as `ruff_python_ast::StmtImport`

```rust
struct StmtImport
```

**Fields**: `node_index`, `range`, `names`, `is_lazy`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Import](https://docs.python.org/3/library/ast.html#ast.Import)

---

## StmtImportFrom

`struct` · `ruff_python_ast::generated::StmtImportFrom`

Also reachable as `ruff_python_ast::StmtImportFrom`

```rust
struct StmtImportFrom
```

**Fields**: `node_index`, `range`, `module`, `names`, `level`, `is_lazy`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [ImportFrom](https://docs.python.org/3/library/ast.html#ast.ImportFrom)

---

## StmtIpyEscapeCommand

`struct` · `ruff_python_ast::generated::StmtIpyEscapeCommand`

Also reachable as `ruff_python_ast::StmtIpyEscapeCommand`

```rust
struct StmtIpyEscapeCommand
```

**Fields**: `node_index`, `range`, `kind`, `value`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

An AST node used to represent a IPython escape command at the statement level.

For example,
```python
%matplotlib inline
```

## Terminology

Escape commands are special IPython syntax which starts with a token to identify
the escape kind followed by the command value itself. [Escape kind] are the kind
of escape commands that are recognized by the token: `%`, `%%`, `!`, `!!`,
`?`, `??`, `/`, `;`, and `,`.

Help command (or Dynamic Object Introspection as it's called) are the escape commands
of the kind `?` and `??`. For example, `?str.replace`. Help end command are a subset
of Help command where the token can be at the end of the line i.e., after the value.
For example, `str.replace?`.

Here's where things get tricky. I'll divide the help end command into two types for
better understanding:
1. Strict version: The token is _only_ at the end of the line. For example,
   `str.replace?` or `str.replace??`.
2. Combined version: Along with the `?` or `??` token, which are at the end of the
   line, there are other escape kind tokens that are present at the start as well.
   For example, `%matplotlib?` or `%%timeit?`.

Priority comes into picture for the "Combined version" mentioned above. How do
we determine the escape kind if there are tokens on both side of the value, i.e., which
token to choose? The Help end command always takes priority over any other token which
means that if there is `?`/`??` at the end then that is used to determine the kind.
For example, in `%matplotlib?` the escape kind is determined using the `?` token
instead of `%` token.

## Syntax

`<IpyEscapeKind><Command value>`

The simplest form is an escape kind token followed by the command value. For example,
`%matplotlib inline`, `/foo`, `!pwd`, etc.

`<Command value><IpyEscapeKind ("?" or "??")>`

The help end escape command would be the reverse of the above syntax. Here, the
escape kind token can only be either `?` or `??` and it is at the end of the line.
For example, `str.replace?`, `math.pi??`, etc.

`<IpyEscapeKind><Command value><EscapeKind ("?" or "??")>`

The final syntax is the combined version of the above two. For example, `%matplotlib?`,
`%%timeit??`, etc.

[Escape kind]: crate::IpyEscapeKind

---

## StmtMatch

`struct` · `ruff_python_ast::generated::StmtMatch`

Also reachable as `ruff_python_ast::StmtMatch`

```rust
struct StmtMatch
```

**Fields**: `node_index`, `range`, `subject`, `cases`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Match](https://docs.python.org/3/library/ast.html#ast.Match)

---

## StmtNonlocal

`struct` · `ruff_python_ast::generated::StmtNonlocal`

Also reachable as `ruff_python_ast::StmtNonlocal`

```rust
struct StmtNonlocal
```

**Fields**: `node_index`, `range`, `names`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Nonlocal](https://docs.python.org/3/library/ast.html#ast.Nonlocal)

---

## StmtPass

`struct` · `ruff_python_ast::generated::StmtPass`

Also reachable as `ruff_python_ast::StmtPass`

```rust
struct StmtPass
```

**Fields**: `node_index`, `range`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Pass](https://docs.python.org/3/library/ast.html#ast.Pass)

---

## StmtRaise

`struct` · `ruff_python_ast::generated::StmtRaise`

Also reachable as `ruff_python_ast::StmtRaise`

```rust
struct StmtRaise
```

**Fields**: `node_index`, `range`, `exc`, `cause`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Raise](https://docs.python.org/3/library/ast.html#ast.Raise)

---

## StmtReturn

`struct` · `ruff_python_ast::generated::StmtReturn`

Also reachable as `ruff_python_ast::StmtReturn`

```rust
struct StmtReturn
```

**Fields**: `node_index`, `range`, `value`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Return](https://docs.python.org/3/library/ast.html#ast.Return)

---

## StmtTry

`struct` · `ruff_python_ast::generated::StmtTry`

Also reachable as `ruff_python_ast::StmtTry`

```rust
struct StmtTry
```

**Fields**: `node_index`, `range`, `body`, `handlers`, `orelse`, `finalbody`, `is_star`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [Try](https://docs.python.org/3/library/ast.html#ast.Try)
and [TryStar](https://docs.python.org/3/library/ast.html#ast.TryStar)

---

## StmtTypeAlias

`struct` · `ruff_python_ast::generated::StmtTypeAlias`

Also reachable as `ruff_python_ast::StmtTypeAlias`

```rust
struct StmtTypeAlias
```

**Fields**: `node_index`, `range`, `name`, `type_params`, `value`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [TypeAlias](https://docs.python.org/3/library/ast.html#ast.TypeAlias)

---

## StmtWhile

`struct` · `ruff_python_ast::generated::StmtWhile`

Also reachable as `ruff_python_ast::StmtWhile`

```rust
struct StmtWhile
```

**Fields**: `node_index`, `range`, `test`, `body`, `orelse`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [While](https://docs.python.org/3/library/ast.html#ast.While)
and [AsyncWhile](https://docs.python.org/3/library/ast.html#ast.AsyncWhile).

---

## StmtWith

`struct` · `ruff_python_ast::generated::StmtWith`

Also reachable as `ruff_python_ast::StmtWith`

```rust
struct StmtWith
```

**Fields**: `node_index`, `range`, `is_async`, `items`, `body`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [With](https://docs.python.org/3/library/ast.html#ast.With)
and [AsyncWith](https://docs.python.org/3/library/ast.html#ast.AsyncWith).

This type differs from the original Python AST, as it collapses the synchronous and asynchronous variants into a single type.

---

## TypeParamParamSpec

`struct` · `ruff_python_ast::generated::TypeParamParamSpec`

Also reachable as `ruff_python_ast::TypeParamParamSpec`

```rust
struct TypeParamParamSpec
```

**Fields**: `node_index`, `range`, `name`, `default`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [ParamSpec](https://docs.python.org/3/library/ast.html#ast.ParamSpec)

---

## TypeParamTypeVar

`struct` · `ruff_python_ast::generated::TypeParamTypeVar`

Also reachable as `ruff_python_ast::TypeParamTypeVar`

```rust
struct TypeParamTypeVar
```

**Fields**: `node_index`, `range`, `name`, `bound`, `default`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [TypeVar](https://docs.python.org/3/library/ast.html#ast.TypeVar)

---

## TypeParamTypeVarTuple

`struct` · `ruff_python_ast::generated::TypeParamTypeVarTuple`

Also reachable as `ruff_python_ast::TypeParamTypeVarTuple`

```rust
struct TypeParamTypeVarTuple
```

**Fields**: `node_index`, `range`, `name`, `default`

**Implements**: `ruff_python_ast::node_index::HasNodeIndex`, `ruff_python_formatter::shared_traits::AsFormat`, `ruff_python_formatter::shared_traits::IntoFormat`, `ruff_text_size::traits::Ranged`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `ruff_python_ast::node_index::HasNodeIndex`**

```rust
fn node_index(&self) -> &AtomicNodeIndex
```

**via `ruff_text_size::traits::Ranged`**

```rust
fn range(&self) -> ruff_text_size::TextRange
```

See also [TypeVarTuple](https://docs.python.org/3/library/ast.html#ast.TypeVarTuple)

---
