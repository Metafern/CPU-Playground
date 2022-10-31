
from ctypes.wintypes import WORD


global_context = {}
curr_context = {}

stack_offset = 0
WORD_SIZE = 4
curr_instructions = []
next_register = 0
#this is all so gross... i gotta redo it all at some point (hopefully sooner rather than later)

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
    global stack_offset
    curr_context[ast['id']] = {'id': ast['id'], 'offset': stack_offset}
    if 'inner' in ast:
        parse_list(ast['inner'])
    else:
        print(f"WARN: Key not found in {ast['id']}")
    curr_instructions.append(f"PUSH r{next_register}")
    stack_offset += WORD_SIZE

def handle_integer_literal(ast):
    curr_instructions.append(f"LOAD {ast['value']}, r{next_register}")

def handle_binary_operator(ast):
    if 'inner' in ast:
        parse_list(ast['inner'])
    else:
        print(f"WARN: Inner not found in {ast}")
    

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
        case 'BinaryOperator':
            handle_binary_operator(ast)
        case 'ImplicitCastExpr':
            parse_list(ast['inner'])
        case other:
            print(f"Type [{ast['kind']}] not found!")
            print(curr_instructions)
            exit()

def parse_list(ast):
    for obj in ast:
        parse_node(obj)