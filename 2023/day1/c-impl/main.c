#include <stdio.h>

int main() {
    FILE* cursor;
    char ch;
    char first_num = '\0';
    char last_num;
    int total = 0;

    cursor = fopen("../input", "r");
    do {
        ch = fgetc(cursor);
        if (ch >= '0' && ch <= '9') {
            if (first_num == '\0') {
                first_num = ch;
            }
            last_num = ch;
        }
        else if (ch == '\n') {
            total += ((int)(first_num - '0'))*10 + ((int)(last_num - '0'));
            first_num = '\0';
        }
    } while (ch != EOF);

    fclose(cursor);
    printf("%d", total);
    return 0;
}