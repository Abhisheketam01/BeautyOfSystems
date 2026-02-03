#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define N 10000000

int main()
{
    int *arr = malloc(N * sizeof(int));
    if (!arr)
        return 1;

    clock_t start, end;

    // Sequential access
    start = clock();
    for (int i = 0; i < N; i++)
    {
        arr[i] = i;
    }
    end = clock();
    printf("Sequential access time: %f seconds\n",
           (double)(end - start) / CLOCKS_PER_SEC);

    // Random access
    start = clock();
    for (int i = 0; i < N; i++)
    {
        int idx = rand() % N;
        arr[idx] = i;
    }
    end = clock();
    printf("Random access time: %f seconds\n",
           (double)(end - start) / CLOCKS_PER_SEC);

    free(arr);
    return 0;
}

/*
output 1-
Sequential access time: 0.019000 seconds
Random access time: 0.154000 seconds

output2-
Sequential access time: 0.027000 seconds
Random access time: 0.158000 seconds
*/