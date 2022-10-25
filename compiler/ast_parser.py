import json
from unittest import case

f = open('test.ast', 'r')

ast = json.load(f)

#todo: add some sort of scope variable list
#todo: add some sort of global function list

def handle_compoundStmt(ast):
    pass

def handle_functionDecl(ast):
    handle_kind(ast['inner'][0])
    pass

def handle_kind(ast):
    match ast['kind']:
        case 'TypedefDecl':
            pass
        case 'FunctionDecl':
            handle_functionDecl(ast);
        case 'CompoundStmt':
            handle_compoundStmt(ast)
        case other:
            print(f"Type [{ast['kind']}] not found!")
            exit()


def parse_node(ast):
    if 'inner' in ast:
        arr = ast['inner']
        for obj in arr:
            handle_kind(obj)

parse_node(ast)