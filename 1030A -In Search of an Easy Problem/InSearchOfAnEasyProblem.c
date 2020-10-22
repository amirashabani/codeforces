#include <stdio.h>
#include <stdbool.h>

int main() {
    int n, opinion;
    bool isEasy = true;

    scanf("%d", &n);

    for(int i = 0; i < n; i++) {
        scanf("%d", &opinion);
        if (opinion == 1) {
            isEasy = false;
        }
    }

    if (isEasy) {
        printf("EASY\n");
    } else {
        printf("HARD\n");
    }
    return 0;
}