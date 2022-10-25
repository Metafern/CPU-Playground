void called() {
int a = 1;
    int b = 2;
    int c = 1 + b;
    int* d = malloc(3);
    return;
}

void test() {
    called();
}

