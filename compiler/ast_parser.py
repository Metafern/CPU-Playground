import json
from ast_methods import *

f = open('test.ast', 'r')

ast = json.load(f)

#todo: add some sort of scope variable list
#todo: add some sort of global function list

parse_node(ast)