
global_context = {}
curr_context = {}

curr_instructions = []

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
        print(f"WARN: Inner not found in {ast}")

def handle_var_decl(ast):
    curr_context[ast['id']] = ast
    if 'inner' in ast:
        parse_list(ast['inner'])
    else:
        print(f"WARN: Key not found in {ast['id']}")
    curr_instructions.append(f"STORE r0, sp")
def handle_integer_literal(ast):
    curr_instructions.append(f"LOAD {ast['value']}, r0")

def handle_binary_operator(ast):
    for expr in ast['inner']:
        pass

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
        case 'IntegerLiteral':
            handle_integer_literal(ast)
        case other:
            print(f"Type [{ast['kind']}] not found!")
            exit()

def parse_list(ast):
    for obj in ast:
        parse_node(obj)