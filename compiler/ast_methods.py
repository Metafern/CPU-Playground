
global_context = {}
curr_context = {}

def handle_function_decl(ast):
    curr_context = []
    parse_list(ast['inner'])
    pass

def handle_compound_stmt(ast):
    parse_list(ast['inner'])

def handle_decl_stmt(ast):
    if 'inner' in ast:
        parse_list(ast['inner'])
    else:
        print(f"WARN: Key not found in {ast['id']}")

def handle_var_decl(ast):
    curr_context[ast['id']] = ast
    if 'inner' in ast:
        parse_list(ast['inner'])
    else:
        print(f"WARN: Key not found in {ast['id']}")

def parse_node(ast):
    match ast['kind']:
        case 'TranslationUnitDecl':
            parse_list(ast['inner'])
        case 'TypedefDecl':
            pass
        case 'FunctionDecl':
            handle_function_decl(ast);
        case 'CompoundStmt':
            handle_compound_stmt(ast)
        case 'DeclStmt':
            handle_decl_stmt(ast)
        case 'VarDecl':
            handle_var_decl(ast)
        case other:
            print(f"Type [{ast['kind']}] not found!")
            exit()

def parse_list(ast):
    for obj in ast:
        parse_node(obj)