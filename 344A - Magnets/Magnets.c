#include <stdio.h>

int main() {
    char *line = NULL;
    size_t size;
    int n, previous, magnet;
    int groups = 0;

    scanf("%d", &n);

    for(int i = 0; i < n; i++) {
        scanf("%d", &magnet);
        if (i == 0) {
            previous = magnet;
            groups++;
        } else {
            if (magnet != previous) {
                groups++;
            }

            previous = magnet;
        }
    }

    printf("%d\n", groups);
    return 0;
}