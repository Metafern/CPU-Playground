import json
from unittest import case

f = open('test.ast', 'r')

ast = json.load(f)

def handle_functionDecl(ast):
    pass

def handle_kind(ast):
    match ast['kind']:
        case 'TypedefDecl':
            pass
        case 'FunctionDecl':
            handle_functionDecl(ast);




def parse_node(ast):
    for node in ast:
        print(node)
    if 'inner' in ast:
        arr = ast['inner']
        for obj in arr:
            handle_kind(obj)

parse_node(ast)